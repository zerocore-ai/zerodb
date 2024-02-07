use std::{cmp::max, sync::Arc};

use tokio::sync::mpsc;

use crate::{
    AppendEntriesRequest, AppendEntriesResponse, Log, NodeId, PeerRpc, RaftNodeInner, Request,
    RequestVoteRequest, RequestVoteResponse, Response, Result,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tasks that a candidate performs.
#[derive(Debug)]
pub(crate) struct CandidateTasks;

/// The result of a vote request.
pub(crate) enum VoteResult {
    NoPeers,
    Granted,
    NotGranted,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl CandidateTasks {
    /// Starts the candidate tasks.
    pub(crate) async fn start<L, R, P>(node: Arc<RaftNodeInner<L, R, P>>) -> Result<()>
    where
        L: Log<R> + Sync + Send + 'static,
        R: Request + Sync + Send + 'static,
        P: Response + Send + 'static,
    {
        // Create a election countdown.
        let mut election_countdown = node.new_election_countdown();

        // Ask for votes.
        let mut vote_result_rx = CandidateTasks::request_votes(Arc::clone(&node)).await?;

        // Get the channels.
        let channels = node.get_channels();
        let in_rpc_rx = &mut *channels.in_rpc_rx.lock().await;
        let in_client_request_rx = &mut *channels.in_client_request_rx.lock().await;
        let shutdown_rx = &mut *channels.shutdown_rx.lock().await;

        loop {
            if !node.is_candidate_state().await {
                break;
            }

            tokio::select! {
                Some(result) = vote_result_rx.recv() => match result {
                    VoteResult::NoPeers => {
                        tracing::debug!(id = node.id.to_string(), "No peers to ask for votes so we become the leader");
                        node.change_to_leader_state().await;
                    },
                    VoteResult::Granted => {
                        tracing::debug!(id = node.id.to_string(), "Received enough votes to become leader");
                        node.change_to_leader_state().await;
                    },
                    VoteResult::NotGranted => {
                        tracing::debug!(id = node.id.to_string(), "Did not receive enough votes to become leader");
                        node.change_to_follower_state().await;
                    }
                },
                Some(request) = in_rpc_rx.recv() => match request {
                    PeerRpc::AppendEntries(AppendEntriesRequest { term, .. }, response_tx) => {
                        // TODO(appcypher): ...

                        // Send ok response.
                        response_tx.send(AppendEntriesResponse {
                            term, // TODO(appcypher): should we return our term instead?
                            success: true,
                            id: node.get_id(),
                        }).await?;

                        node.change_to_follower_state().await;
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
                        // Send rejected response.
                        response_tx.send(RequestVoteResponse {
                            term: node.get_current_term(),
                            vote_granted: false,
                            id: node.get_id(),
                        }).await?;
                    }
                },
                Some(_) = in_client_request_rx.recv() => {
                    tracing::debug!(id = node.id.to_string(), "Received client request");
                },
                _ = shutdown_rx.recv() => {
                    // Shutdown.
                    node.change_to_shutdown_state().await;
                },
                _ = election_countdown.continuation()  => {
                    // Reset election countdown.
                    election_countdown.reset();

                    // TODO(appcypher): Start a new vote session.
                    vote_result_rx = CandidateTasks::request_votes(Arc::clone(&node)).await?;
                }
            }
        }

        Ok(())
    }

    /// Requests votes from the peers.
    async fn request_votes<L, R, P>(
        node: Arc<RaftNodeInner<L, R, P>>,
    ) -> Result<mpsc::Receiver<VoteResult>>
    where
        L: Log<R> + Sync + Send + 'static,
        R: Request + Sync + Send + 'static,
        P: Response + Send + 'static,
    {
        node.increment_term().await;
        node.vote_for_self().await;

        // Create a channel to receive the vote result.
        let (vote_result_tx, vote_result_rx) = mpsc::channel(1);

        tokio::spawn(async move {
            // Filter out the current node from the peers.
            let valid_peers: Vec<NodeId> = node.get_valid_peers().cloned().collect();

            // Early exit to if there are no peers.
            if valid_peers.is_empty() {
                vote_result_tx.send(VoteResult::NoPeers).await?;
                return crate::Ok(());
            }

            // Create a channel to receive the vote responses.
            let (vote_tx, mut vote_rx) =
                mpsc::channel::<RequestVoteResponse>(max(valid_peers.len(), 1));

            // Send RequestVote RPC to all other servers.
            for peer in valid_peers {
                let vote_tx = vote_tx.clone();
                let node = Arc::clone(&node);

                // Send the RequestVote RPC in a separate task.
                tokio::spawn(async move {
                    node.send_request_vote_rpc(peer, vote_tx).await?;
                    crate::Ok(())
                });
            }

            // Drop the vote_tx so that we can wait for all the vote responses.
            drop(vote_tx);

            // Wait for all the vote responses.
            let mut votes_granted = 1;
            while let Some(vote) = vote_rx.recv().await {
                votes_granted += if vote.vote_granted { 1 } else { 0 };

                // We short-circuit if we have enough votes to become the leader.
                if votes_granted > node.get_peers().len() / 2 {
                    vote_result_tx.send(VoteResult::Granted).await?;
                    return crate::Ok(());
                }
            }

            vote_result_tx.send(VoteResult::NotGranted).await?;

            crate::Ok(())
        });

        Ok(vote_result_rx)
    }
}
