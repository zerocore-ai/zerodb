use crate::{
    roles::common, ClientRequest, ClientResponse, ClientResponseReason, PeerRpc, RaftNode, Request,
    Response, Result, Store,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tasks that a follower performs.
#[derive(Debug)]
pub(crate) struct FollowerRole;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl FollowerRole {
    /// Starts the follower tasks.
    pub(crate) async fn start<S, R, P>(node: RaftNode<S, R, P>) -> Result<()>
    where
        S: Store<R> + Send + Sync + 'static,
        R: Request + Send + Sync + 'static,
        P: Response + Send + Sync + 'static,
    {
        // Create a election timeout.
        let mut election_timeout = node.new_election_timeout();

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
                _ = shutdown_rx.recv() => {
                    node.change_to_shutdown_state().await;
                },
                Some(request) = in_rpc_rx.recv() => match request {
                    PeerRpc::AppendEntries(request, response_tx) => {
                        common::respond_to_append_entries(node.clone(), request, response_tx).await?;
                        election_timeout.reset();
                    },
                    PeerRpc::RequestVote(request, response_tx) => {
                        common::respond_to_request_vote(node.clone(), request, response_tx).await?;
                        election_timeout.reset();
                    },
                    PeerRpc::Config(_, _) => {
                        unimplemented!("Config RPC not implemented") // TODO(appcypher): Implement Config RPC.
                    },
                    PeerRpc::InstallSnapshot(_, _) => {
                        unimplemented!("InstallSnapshot RPC not implemented") // TODO(appcypher): Implement InstallSnapshot RPC.
                    }
                },
                Some(ClientRequest(_, response_tx)) = in_client_request_rx.recv() => {
                    // Check if there is a leader.
                    if let Some(leader_id) = node.get_leader_id().await {
                        // Redirect to the leader.
                        response_tx
                            .send(ClientResponse {
                                success: false,
                                reason: ClientResponseReason::Redirect,
                                leader_id: Some(leader_id),
                                payload: None
                            })
                            .await?;
                    } else {
                        // No leader yet.
                        response_tx
                            .send(ClientResponse {
                                success: false,
                                reason: ClientResponseReason::NoLeaderYet,
                                leader_id: None,
                                payload: None
                            })
                            .await?;
                    }

                },
                _ = election_timeout.continuation() => {
                    node.change_to_candidate_state().await;
                }
            }
        }

        Ok(())
    }
}
