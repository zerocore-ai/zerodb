use std::sync::Arc;

use crate::{
    AppendEntriesRequest, AppendEntriesResponse, Log, PeerRpc, RaftNodeInner, Request,
    RequestVoteRequest, RequestVoteResponse, Response, Result,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tasks that a follower performs.
#[derive(Debug)]
pub(crate) struct FollowerTasks;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl FollowerTasks {
    /// Starts the follower tasks.
    pub(crate) async fn start<L, R, P>(node: Arc<RaftNodeInner<L, R, P>>) -> Result<()>
    where
        L: Log<R>,
        R: Request,
        P: Response + Send + 'static,
    {
        // Create a election countdown.
        let mut election_countdown = node.new_election_countdown();

        // Get the channels.
        let channels = node.get_channels();
        let in_rpc_rx = &mut *channels.in_rpc_rx.lock().await;
        let in_client_request_rx = &mut *channels.in_client_request_rx.lock().await;
        let shutdown_rx = &mut *channels.shutdown_rx.lock().await;

        loop {
            if !node.is_follower_state().await {
                break;
            }

            tokio::select! {
                Some(request) = in_rpc_rx.recv() => match request {
                    PeerRpc::AppendEntries(AppendEntriesRequest { term, .. }, response_tx) => {
                        // TODO(appcypher): ...

                        // Send ok response.
                        response_tx.send(AppendEntriesResponse {
                            term, // TODO(appcypher): should we return our term instead?
                            success: true,
                            id: node.get_id(),
                        }).await?;

                        election_countdown.reset();
                    },
                    PeerRpc::RequestVote(RequestVoteRequest { term, candidate_id }, response_tx) => {
                        // Check if our term is stale.
                        if term > node.get_current_term() {
                            // Send granted response.
                            response_tx.send(RequestVoteResponse {
                                term, // TODO(appcypher): should we return our term instead?
                                vote_granted: true,
                                id: node.get_id(),
                            }).await?;

                            node.change_to_follower_state().await;
                            node.update_term_and_voted_for(term, candidate_id).await;
                            continue;
                        }

                        // We have either already voted or the request term is stale.
                        // Send not granted response.
                        response_tx.send(RequestVoteResponse {
                            term: node.get_current_term(),
                            vote_granted: false,
                            id: node.get_id(),
                        }).await?;
                    }
                },
                Some(_) = in_client_request_rx.recv() => {
                    // // Forward the client request to the leader.
                    // node.forward_client_request_to_leader(client_request).await?;
                },
                _ = shutdown_rx.recv() => {
                    // Shutdown.
                    node.change_to_shutdown_state().await;
                },
                _ = election_countdown.continuation() => {
                    // Become a candidate.
                    node.change_to_candidate_state().await;
                }
            }
        }

        Ok(())
    }
}
