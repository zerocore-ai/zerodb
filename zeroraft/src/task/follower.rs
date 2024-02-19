use std::{cmp::min, sync::Arc};

use crate::{
    task::common, AppendEntriesResponse, AppendEntriesResponseReason, ClientRequest,
    ClientResponse, ClientResponseReason, Log, PeerRpc, RaftNodeInner, Request, Response, Result,
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
        L: Log<R> + Send + Sync + 'static,
        R: Request + Send + Sync + 'static,
        P: Response + Send + Sync + 'static,
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
                _ = shutdown_rx.recv() => {
                    // Shutdown.
                    node.change_to_shutdown_state().await;
                },
                Some(request) = in_rpc_rx.recv() => match request {
                    PeerRpc::AppendEntries(request, response_tx) => {
                        common::respond_to_append_entries(Arc::clone(&node), request, response_tx, |node, request, response_tx| Box::pin(async move {
                            let our_term = node.get_current_term();
                            let our_id = node.get_id();
                            let our_last_log_index = node.log.lock().await.get_last_index().await;

                            // Check if we don't have the prev log index the leader is trying to append to.
                            if our_last_log_index < request.prev_log_index  {
                                response_tx
                                    .send(AppendEntriesResponse {
                                        term: our_term,
                                        success: false,
                                        id: our_id,
                                        reason: AppendEntriesResponseReason::LogDoesNotExist,
                                    })
                                    .await?;

                                return Ok(());
                            }

                            // Check if log index exists but the term doesn't match.
                            if let Some(entry) = node.log.lock().await.get_entry(request.prev_log_index).await {
                                if entry.term != request.prev_log_term {
                                    response_tx
                                        .send(AppendEntriesResponse {
                                            term: our_term,
                                            success: false,
                                            id: our_id,
                                            reason: AppendEntriesResponseReason::LogTermMismatch,
                                        })
                                        .await?;

                                    return Ok(());
                                }
                            }

                            // Remove extraneous entries.
                            node.log.lock().await.remove_entries_after(request.prev_log_index).await?;

                            // Append the entries.
                            node.log.lock().await.append_entries(request.entries).await?;

                            // Update the commit index.
                            node.log.lock().await.set_last_commit_index(min(
                                request.last_commit_index,
                                node.log.lock().await.get_last_index().await
                            )).await?;

                            // Respond to the append entries request.
                            response_tx
                                .send(AppendEntriesResponse {
                                    term: our_term,
                                    success: true,
                                    id: our_id,
                                    reason: AppendEntriesResponseReason::Ok,
                                })
                                .await?;

                            Ok(())
                        })).await?;
                    },
                    PeerRpc::RequestVote(request, response_tx) => {
                        common::respond_to_request_vote(Arc::clone(&node), request, response_tx).await?;
                        election_countdown.reset();
                    },
                    PeerRpc::Config(_, _) => {
                        // TODO(appcypher): Implement Config RPC.
                        unimplemented!("Config RPC not implemented")
                    },
                    PeerRpc::InstallSnapshot(_, _) => {
                        // TODO(appcypher): Implement InstallSnapshot RPC.
                        unimplemented!("InstallSnapshot RPC not implemented")
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
                _ = election_countdown.continuation() => {
                    // Become a candidate.
                    node.change_to_candidate_state().await;
                }
            }
        }

        Ok(())
    }
}
