use std::{
    collections::HashSet,
    ops::Deref,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use futures::Future;
use tokio::{
    sync::{mpsc, Mutex},
    time,
};

use crate::{
    roles::common, ClientRequest, ClientResponse, ClientResponseReason, NodeId, PeerRpc, RaftNode,
    Request, RequestVoteResponse, RequestVoteResponseReason, Response, Result, Store, Timeout,
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
}

/// A vote session.
pub struct VoteSession<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    inner: Arc<VoteSessionInner<S, R, P>>,
}

/// The inner state of a vote session.
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
        let (mut session, mut vote_result_rx) = VoteSession::initialize(node.clone()).await;
        session.request_votes(retry_timeout.clone(), election_timeout.clone());

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
                    }
                },
                Some(request) = in_rpc_rx.recv() => match request {
                    PeerRpc::AppendEntries(request, response_tx) => {
                        common::respond_to_append_entries(node.clone(), request, response_tx).await?;
                    },
                    PeerRpc::RequestVote(request, response_tx) => {
                        common::respond_to_request_vote(node.clone(), request, response_tx).await?;
                    },
                    PeerRpc::Config(_, _) => {
                        unimplemented!("Config RPC not implemented") // TODO(appcypher): Implement Config RPC.
                    },
                    PeerRpc::InstallSnapshot(_, _) => {
                        unimplemented!("InstallSnapshot RPC not implemented") // TODO(appcypher): Implement InstallSnapshot RPC.
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
                    session.request_votes(retry_timeout.clone(), election_timeout.clone());
                }
                _ = election_timeout.continuation()  => {
                    // Reset timeouts
                    election_timeout.reset();
                    retry_timeout.reset();

                    // Restart vote session.
                    (session, vote_result_rx) = VoteSession::initialize(node.clone()).await;
                    session.request_votes(retry_timeout.clone(), election_timeout.clone());
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
    /// Initializes a new vote session instance.
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

    /// Sends a RequestVote RPC to peers.
    pub fn request_votes(&self, retry: Timeout, election: Timeout)
    where
        S: Send + Sync + 'static,
        R: Send + 'static,
        P: Send + 'static,
    {
        let session = self.clone();
        tokio::spawn(timeouts(retry.clone(), election.clone(), async move {
            // NOTE: times out with retry and election
            let peers_len = session.node.inner.store.read().await.get_membership().len();
            let Some(unack_peers) = session.get_unack_peers().await? else {
                return Ok(());
            };

            // Create a channel to receive the vote responses.
            let (vote_tx, mut vote_rx) = mpsc::unbounded_channel::<RequestVoteResponse>();

            // Send the RequestVote RPC in a separate task.
            session.send_to_peers(unack_peers, vote_tx, retry, election);

            // Wait for all the vote responses.
            while let Some(response) = vote_rx.recv().await {
                match response.reason {
                    RequestVoteResponseReason::StaleTerm
                    | RequestVoteResponseReason::IncompleteLog => {
                        session.node.update_current_term(response.term).await?; // Update term
                        session.node.change_to_follower_state().await; // Steps down to follower.
                        break;
                    }
                    RequestVoteResponseReason::AlreadyVoted | RequestVoteResponseReason::Ok => {}
                }

                session.ack_peers.lock().await.insert(response.id); // Add the peer to the list of acknowledged peers.

                if !response.vote_granted {
                    continue; // Skip if the vote was not granted.
                }

                session.granted_votes.fetch_add(1, Ordering::SeqCst); // Increment the granted votes.

                // We short-circuit if we have enough votes to become the leader.
                let cluster_size = peers_len + 1;
                if session.granted_votes.load(Ordering::SeqCst) > cluster_size / 2 {
                    session.vote_result_tx.send(VoteResult::Granted).await?;
                    return crate::Ok(());
                }
            }

            crate::Ok(())
        }));
    }

    /// Sends vote requests to peers.
    fn send_to_peers(
        &self,
        unack_peers: HashSet<NodeId>,
        vote_tx: mpsc::UnboundedSender<RequestVoteResponse>,
        retry: Timeout,
        election: Timeout,
    ) where
        S: Send + Sync + 'static,
        R: Send + 'static,
        P: Send + 'static,
    {
        // Send RequestVote RPC to all unreached peers.
        for peer in unack_peers {
            let vote_tx = vote_tx.clone();
            let node = self.node.clone();

            tokio::spawn(timeouts(retry.clone(), election.clone(), async move {
                // NOTE: times out with retry and election
                node.send_request_vote_rpc(peer, vote_tx).await
            }));
        }
    }

    /// Gets the peers that have not acknowledged the vote request.
    async fn get_unack_peers(&self) -> Result<Option<HashSet<NodeId>>> {
        // TODO: This part can be done once in initialize. Need a peers field.
        let peers = self.node.inner.store.read().await;
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
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

#[inline(always)]
fn timeouts<F, O>(retry: Timeout, election: Timeout, future: F) -> time::Timeout<time::Timeout<F>>
where
    F: Future<Output = O>,
{
    time::timeout(
        retry.get_remaining(),
        time::timeout(election.get_remaining(), future),
    )
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
