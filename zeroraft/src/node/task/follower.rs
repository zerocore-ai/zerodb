use std::sync::Arc;

use crate::{Log, PeerRpc, RaftNodeInner, Request, RequestVoteResponse, Response, Result};

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
                    PeerRpc::AppendEntries(_, _) => {
                        tracing::debug!(">> Received AppendEntries request.");
                        // Reset election countdown.
                        election_countdown.reset();
                    },
                    PeerRpc::RequestVote(request, response_tx) => {
                        // TODO(appcypher): Check if we can vote for the candidate. For now, we always vote for the candidate.
                        let response = RequestVoteResponse {
                            term: request.term, // TODO(appcypher): Gotta update term after this.
                            vote_granted: true,
                        };

                        // Send the response.
                        response_tx.send(response).await?;
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
