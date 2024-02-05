use std::sync::Arc;

use crate::{
    Log, PeerRpc, RaftNodeInner, Request, RequestVoteRequest, RequestVoteResponse, Response, Result,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tasks that a leader performs.
#[derive(Debug)]
pub(crate) struct LeaderTasks;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl LeaderTasks {
    /// Starts the leader tasks.
    pub(crate) async fn start<L, R, P>(node: Arc<RaftNodeInner<L, R, P>>) -> Result<()>
    where
        L: Log<R>,
        R: Request,
        P: Response + Send + 'static,
    {
        // Get the channels.
        let channels = node.get_channels();
        let in_rpc_rx = &mut *channels.in_rpc_rx.lock().await;
        let in_client_request_rx = &mut *channels.in_client_request_rx.lock().await;
        let shutdown_rx = &mut *channels.shutdown_rx.lock().await;

        loop {
            if !node.is_leader_state().await {
                break;
            }

            tokio::select! {
                _ = shutdown_rx.recv() => {
                    // Shutdown.
                    node.change_to_shutdown_state().await;
                },
                Some(request) = in_rpc_rx.recv() => match request {
                    PeerRpc::AppendEntries(_, _) => {
                        tracing::debug!(">> Received AppendEntries request.");
                    },
                    PeerRpc::RequestVote(RequestVoteRequest { term, ..}, response_tx) => {
                        // TODO(appcypher): Check if we can vote for the candidate. For now, we always vote for the candidate.
                        if term > node.get_current_term() {
                            node.change_to_follower_state().await;

                            // Send the response.
                            response_tx.send(RequestVoteResponse {
                                term, // TODO(appcypher): Gotta update term after this.
                                vote_granted: true,
                            }).await?;
                        } else {
                            // Send the response.
                            response_tx.send(RequestVoteResponse {
                                term: node.get_current_term(),
                                vote_granted: false,
                            }).await?;
                        }
                    }
                },
                Some(_) = in_client_request_rx.recv() => {
                    tracing::debug!("Received client request.");
                },
            }
        }

        Ok(())
    }
}
