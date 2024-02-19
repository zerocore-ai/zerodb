use std::{
    cmp::max,
    collections::HashSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use tokio::sync::{mpsc, Mutex};

use crate::{
    task::common, AppendEntriesResponse, AppendEntriesResponseReason, ClientRequest, ClientResponse, ClientResponseReason, Log, NodeId, PeerRpc, RaftNodeInner, Request, RequestVoteResponse, Response, Result
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tasks that a candidate performs.
#[derive(Debug)]
pub(crate) struct CandidateTasks;

/// This type is used to track the state of the vote session for each peer.
pub(crate) struct VoteSession<L, R, P>
where
    L: Log<R>,
    R: Request,
    P: Response,
{
    reached_peers: Mutex<HashSet<NodeId>>,
    node: Arc<RaftNodeInner<L, R, P>>,
    vote_result_tx: Mutex<mpsc::Sender<VoteResult>>,
    granted_votes: AtomicUsize,
}

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
        let (vote_session, mut vote_result_rx) = VoteSession::initialize(Arc::clone(&node)).await;
        vote_session.request_votes_from_all();

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
                _ = shutdown_rx.recv() => {
                    // Shutdown.
                    node.change_to_shutdown_state().await;
                },
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
                    PeerRpc::AppendEntries(request, response_tx) => {
                        common::respond_to_append_entries(Arc::clone(&node), request, response_tx, |node, _, response_tx| Box::pin(async move {
                            response_tx
                                .send(AppendEntriesResponse {
                                    term: node.get_current_term(),
                                    success: false,
                                    id: node.get_id(),
                                    reason: AppendEntriesResponseReason::NotAFollower,
                                })
                                .await?;

                            Ok(())
                        })).await?;
                    },
                    PeerRpc::RequestVote(request, response_tx) => {
                        common::respond_to_request_vote(Arc::clone(&node), request, response_tx).await?;
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
                _ = election_countdown.continuation()  => {
                    // Reset election countdown.
                    election_countdown.reset();

                    // Reset the vote session.
                    vote_result_rx = vote_session.reset().await;
                    vote_session.request_votes_from_all();
                }
            }
        }

        Ok(())
    }
}

impl<L, R, P> VoteSession<L, R, P>
where
    L: Log<R> + Sync + Send + 'static,
    R: Request + Sync + Send + 'static,
    P: Response + Send + 'static,
{
    /// Initialize a new vote session.
    pub async fn initialize(
        node: Arc<RaftNodeInner<L, R, P>>,
    ) -> (Arc<Self>, mpsc::Receiver<VoteResult>) {
        node.increment_term().await;
        node.vote_for_self().await;

        let (vote_result_tx, vote_result_rx) = mpsc::channel(1);
        let session = Arc::new(VoteSession {
            reached_peers: Mutex::new(HashSet::new()),
            node,
            vote_result_tx: Mutex::new(vote_result_tx),
            granted_votes: AtomicUsize::new(1),
        });

        (session, vote_result_rx)
    }

    /// Resets the vote session.
    pub async fn reset(&self) -> mpsc::Receiver<VoteResult> {
        self.node.increment_term().await;
        self.node.vote_for_self().await;

        let (vote_result_tx, vote_result_rx) = mpsc::channel(1);

        *self.reached_peers.lock().await = HashSet::new();
        *self.vote_result_tx.lock().await = vote_result_tx;
        self.granted_votes.store(1, Ordering::SeqCst);

        vote_result_rx
    }

    /// Requests votes from the peers.
    fn request_votes_from_all(self: &Arc<Self>) {
        let session = Arc::clone(self);
        tokio::spawn(async move {
            let peers = &mut *session.node.peers.lock().await;

            // Early exit to if there are no peers.
            if peers.is_empty() {
                session
                    .vote_result_tx
                    .lock()
                    .await
                    .send(VoteResult::NoPeers)
                    .await?;
                return crate::Ok(());
            }

            let reached_peers = &mut *session.reached_peers.lock().await;
            let unreached_peers = peers
                .difference(&reached_peers)
                .cloned()
                .collect::<HashSet<_>>();

            // Early exit if there are no unreached peers.
            if unreached_peers.is_empty() {
                return crate::Ok(());
            }

            // Create a channel to receive the vote responses.
            let (vote_tx, mut vote_rx) = mpsc::channel::<RequestVoteResponse>(max(peers.len(), 1));

            // Send RequestVote RPC to all unreached peers.
            for peer in unreached_peers {
                let vote_tx = vote_tx.clone();
                let node = Arc::clone(&session.node);

                // Send the RequestVote RPC in a separate task.
                tokio::spawn(async move {
                    node.send_request_vote_rpc(peer, vote_tx).await?;
                    crate::Ok(())
                });
            }

            // Drop the vote_tx so that we can wait for all the vote responses.
            drop(vote_tx);

            // Wait for all the vote responses.
            while let Some(vote) = vote_rx.recv().await {
                if vote.vote_granted {
                    session.granted_votes.fetch_add(1, Ordering::SeqCst);
                }

                // We short-circuit if we have enough votes to become the leader.
                let cluster_size = session.node.peers.lock().await.len() + 1;
                if session.granted_votes.load(Ordering::SeqCst) > cluster_size / 2 {
                    session
                        .vote_result_tx
                        .lock()
                        .await
                        .send(VoteResult::Granted)
                        .await?;

                    return crate::Ok(());
                }
            }

            session
                .vote_result_tx
                .lock()
                .await
                .send(VoteResult::NotGranted)
                .await?;

            crate::Ok(())
        });
    }
}
