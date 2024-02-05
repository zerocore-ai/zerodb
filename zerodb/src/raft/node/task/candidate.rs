use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Log, PeerRpc, RaftNodeInner, Request, RequestVoteRequest, RequestVoteResponse, Response, Result,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tasks that a candidate performs.
#[derive(Debug)]
pub(crate) struct CandidateTasks;

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
                        tracing::debug!(">> No peers to ask for votes so we become the leader.");
                        node.change_to_leader_state().await;
                    },
                    VoteResult::Granted => {
                        tracing::debug!(">> Received enough votes to become leader.");
                        node.change_to_leader_state().await;
                    },
                    VoteResult::NotGranted => {
                        tracing::debug!(">> Did not receive enough votes to become leader.");
                        node.change_to_follower_state().await;
                    }
                },
                Some(request) = in_rpc_rx.recv() => match request {
                    PeerRpc::AppendEntries(_, _) => {
                        tracing::debug!(">> Received AppendEntries RPC.");
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
                    tracing::debug!(">> Received client request.");
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
            // Filter out the current node from the peers. // TODO(appcypher): Might want to move this to the RaftNodeInner. Filter it out at creation and updates.
            let valid_peers = node
                .get_peers()
                .iter()
                .filter(|peer| **peer != node.get_id())
                .collect::<Vec<_>>();

            // Early exit to if there are no peers.
            if valid_peers.is_empty() {
                vote_result_tx.send(VoteResult::NoPeers).await?;
                return crate::Ok(());
            }

            // Create a channel to receive the vote responses.
            let (vote_tx, mut vote_rx) = mpsc::channel::<RequestVoteResponse>(valid_peers.len());

            // Send RequestVote RPC to all other servers.
            for peer in valid_peers.iter() {
                let vote_tx = vote_tx.clone();
                let node = Arc::clone(&node);
                let peer = (*peer).clone();

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
