use std::{
    cmp::max,
    collections::HashSet,
    ops::Deref,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use tokio::sync::{mpsc, Mutex};

use crate::{
    roles::common, AppendEntriesResponse, AppendEntriesResponseReason, ClientRequest,
    ClientResponse, ClientResponseReason, NodeId, PeerRpc, RaftNode, Request, RequestVoteResponse,
    Response, Result, Store, Timeout,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tasks that a candidate performs.
#[derive(Debug)]
pub(crate) struct CandidateRole;

/// The result of a vote request.
pub(crate) enum VoteResult {
    NoPeers,
    Granted,
    NotGranted,
}

/// TODO: Add documentation
pub struct VoteSession<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    inner: Arc<VoteSessionInner<S, R, P>>,
}

/// TODO: Add documentation
pub struct VoteSessionInner<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    ack_peers: Mutex<HashSet<NodeId>>,
    node: RaftNode<S, R, P>,
    vote_result_tx: mpsc::Sender<VoteResult>,
    granted_votes: AtomicUsize,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl CandidateRole {
    /// Starts the candidate tasks.
    pub(crate) async fn start<S, R, P>(node: RaftNode<S, R, P>) -> Result<()>
    where
        S: Store<R> + Sync + Send + 'static,
        R: Request + Sync + Send + 'static,
        P: Response + Send + 'static,
    {
        let node_id = node.get_id().to_string();

        // Create a election and retry timeouts.
        let mut election_timeout = node.new_election_timeout();
        let mut retry_timeout = Timeout::start(node.get_heartbeat_interval());

        // Start vote session
        let (mut vote_session, mut vote_result_rx) = VoteSession::initialize(node.clone()).await;
        vote_session.request_votes();

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
                    node.change_to_shutdown_state().await;
                },
                Some(result) = vote_result_rx.recv() => match result {
                    VoteResult::NoPeers => {
                        tracing::debug!(id = node_id, "No peers to ask for votes so we become the leader");
                        node.change_to_leader_state().await;
                    },
                    VoteResult::Granted => {
                        tracing::debug!(id = node_id, "Received enough votes to become leader");
                        node.change_to_leader_state().await;
                    },
                    VoteResult::NotGranted => {
                        tracing::debug!(id = node_id, "Did not receive enough votes to become leader");
                        // Do nothing
                    }
                },
                Some(request) = in_rpc_rx.recv() => match request {
                    PeerRpc::AppendEntries(request, response_tx) => {
                        common::respond_to_append_entries(node.clone(), request, response_tx, |node, _, response_tx| Box::pin(async move {
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
                        common::respond_to_request_vote(node.clone(), request, response_tx).await?;
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
                    if let Some(leader_id) = node.get_leader_id().await { // Check if there is a leader.
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
                _ = retry_timeout.continuation() => {
                    retry_timeout.reset();
                    vote_session.request_votes();
                }
                _ = election_timeout.continuation()  => {
                    // Reset timeouts
                    election_timeout.reset();
                    retry_timeout.reset();

                    // Restart vote session.
                    (vote_session, vote_result_rx) = VoteSession::initialize(node.clone()).await;
                    vote_session.request_votes();
                }
            }
        }

        Ok(())
    }
}

impl<S, R, P> VoteSession<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    pub async fn initialize(node: RaftNode<S, R, P>) -> (Self, mpsc::Receiver<VoteResult>) {
        node.increment_term().await;
        node.vote_for_self().await;

        let (vote_result_tx, vote_result_rx) = mpsc::channel(1);

        let session = Self {
            inner: Arc::new(VoteSessionInner {
                ack_peers: Mutex::new(HashSet::new()),
                node,
                granted_votes: AtomicUsize::new(1),
                vote_result_tx,
            }),
        };

        (session, vote_result_rx)
    }

    /// TODO: Document this.
    pub fn request_votes(&self)
    where
        S: Send + 'static,
        R: Send + 'static,
        P: Send + 'static,
    {
        let session = self.clone();
        tokio::spawn(async move {
            let peers_len = session.node.inner.store.lock().await.get_membership().len();
            let Some(unack_peers) = session.unack_peers().await? else {
                return Ok(());
            };

            // Create a channel to receive the vote responses.
            let (vote_tx, mut vote_rx) = mpsc::channel::<RequestVoteResponse>(max(peers_len, 1));

            // Send the RequestVote RPC in a separate task.
            session.send_to_peers(unack_peers, vote_tx);

            // Wait for all the vote responses.
            while let Some(vote) = vote_rx.recv().await {
                if vote.vote_granted {
                    session.granted_votes.fetch_add(1, Ordering::SeqCst);
                }

                // We short-circuit if we have enough votes to become the leader.
                let cluster_size = peers_len + 1;
                if session.granted_votes.load(Ordering::SeqCst) > cluster_size / 2 {
                    session.vote_result_tx.send(VoteResult::Granted).await?;
                    return crate::Ok(());
                }
            }

            session.vote_result_tx.send(VoteResult::NotGranted).await?;

            crate::Ok(())
        });
    }

    /// TODO: Document this.
    async fn unack_peers(&self) -> Result<Option<HashSet<NodeId>>> {
        let peers = self.node.inner.store.lock().await;
        let peers = peers
            .get_membership()
            .keys()
            .filter(|id| **id != self.node.get_id())
            .collect::<HashSet<_>>();

        // Early exit to if there are no peers.
        if peers.is_empty() {
            self.vote_result_tx.send(VoteResult::NoPeers).await?;
            return Ok(None);
        }

        let ack_peers = self.ack_peers.lock().await;
        let ack_peers = ack_peers.iter().collect();
        let unack_peers = peers
            .difference(&ack_peers)
            .map(|peer| **peer)
            .collect::<HashSet<_>>();

        // Early exit if there are no unreached peers.
        if unack_peers.is_empty() {
            return Ok(None);
        }

        Ok(Some(unack_peers))
    }

    /// TODO: Document this.
    fn send_to_peers(
        &self,
        unack_peers: HashSet<NodeId>,
        vote_tx: mpsc::Sender<RequestVoteResponse>,
    ) where
        S: Send + 'static,
        R: Send + 'static,
        P: Send + 'static,
    {
        // Send RequestVote RPC to all unreached peers.
        for peer in unack_peers {
            let vote_tx = vote_tx.clone();
            let node = self.node.clone();

            // Send the RequestVote RPC in a separate task.
            tokio::spawn(async move { node.send_request_vote_rpc(peer, vote_tx).await });
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S, R, P> Deref for VoteSession<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    type Target = VoteSessionInner<S, R, P>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S, R, P> Clone for VoteSession<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
