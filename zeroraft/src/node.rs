use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

use tokio::{
    sync::{mpsc, RwLock},
    task::JoinHandle,
};
use uuid::Uuid;

use crate::{
    roles::TaskState, AppendEntriesRequest, AppendEntriesResponse, PeerRpc, RaftNodeBuilder,
    RaftSideChannels, Request, RequestVoteRequest, RequestVoteResponse, Response, Result, Store,
    Timeout,
};

use super::roles::{CandidateRole, FollowerRole, LeaderRole};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Node ID.
pub type NodeId = Uuid;

// TODO(appcypher): RPC retries
/// A `RaftNode` represents a single node in a Raft consensus cluster.
///
/// Each node has a unique ID, a current term, a voted_for field to keep track of the node it has voted for,
/// a store for entries, and a role which can be either `Follower`, `Candidate`, or `Leader`.
/// The role determines how the node responds to incoming requests.
pub struct RaftNode<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    pub(super) inner: Arc<RaftNodeInner<S, R, P>>,
}

// TODO(appcypher): We need to persist some fields to disk.
/// The inner state of a Raft node.
pub(crate) struct RaftNodeInner<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    /// The unique ID of the node.
    pub(crate) id: NodeId,

    /// The latest term the node has seen.
    pub(crate) current_term: AtomicU64,

    /// The ID of the node that the current node voted for in the current term.
    pub(crate) voted_for: RwLock<Option<NodeId>>,

    /// The store for node's log and state.
    pub(crate) store: RwLock<S>,

    /// The communication channels for the node.
    pub(crate) channels: RaftSideChannels<R, P>,

    /// The current state of the node.
    pub(crate) current_state: RwLock<TaskState>,

    /// Election timeout range.
    pub(crate) election_timeout_range: (u64, u64),

    /// Heartbeat interval.
    pub(crate) heartbeat_interval: u64,

    /// The current leader id.
    pub(crate) leader_id: RwLock<Option<NodeId>>,

    /// Last time the node heard from the leader.
    /// Used to prevent unnecessary voting when there is a stable leader.
    pub(crate) last_heard_from_leader: RwLock<Option<Instant>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S, R, P> RaftNode<S, R, P>
where
    S: Store<R> + Send + Sync + 'static,
    R: Request + Clone + Send + Sync + 'static,
    P: Response + Send + Sync + 'static,
{
    /// Starts the Raft node.
    pub fn start(&self) -> JoinHandle<Result<()>> {
        let node = self.clone();
        tokio::spawn(async move {
            loop {
                let current_state = node.get_current_state().await;

                tracing::debug!(
                    id = node.get_id().to_string(),
                    "State changed: {current_state:?}"
                );

                let node = node.clone();
                match current_state {
                    TaskState::Follower => FollowerRole::start(node).await?,
                    TaskState::Candidate => CandidateRole::start(node).await?,
                    TaskState::Leader => LeaderRole::start(node).await?,
                    TaskState::NonVoter => {
                        todo!("Implement NonVotingMember state") // TODO(appcypher): Implement NonVotingMember state
                    }
                    TaskState::Shutdown => {
                        break;
                    }
                }
            }

            Ok(())
        })
    }
}

impl<S, R, P> RaftNode<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    /// Returns a new `RaftNodeBuilder` instance.
    ///
    /// This lets you configure the Raft node before starting it.
    pub fn builder() -> RaftNodeBuilder<S, R, P>
    where
        S: Default,
    {
        RaftNodeBuilder::default()
    }

    /// Changes the state of the node to `Follower`.
    pub(super) async fn change_to_follower_state(&self) {
        *self.inner.current_state.write().await = TaskState::Follower;
    }

    /// Changes the state of the node to `Candidate`.
    pub(super) async fn change_to_candidate_state(&self) {
        *self.inner.current_state.write().await = TaskState::Candidate;
    }

    /// Changes the state of the node to `Leader`.
    pub(super) async fn change_to_leader_state(&self) {
        *self.inner.current_state.write().await = TaskState::Leader;
    }

    /// Changes the state of the node to `Shutdown`.
    pub(super) async fn change_to_shutdown_state(&self) {
        *self.inner.current_state.write().await = TaskState::Shutdown;
    }

    /// Increments the current term of the node.
    pub(super) async fn increment_term(&self) {
        tracing::debug!(
            id = self.inner.id.to_string(),
            "Incrementing term: {}",
            self.inner.current_term.load(Ordering::SeqCst),
        );

        // TODO: We need to persist this to disk.
        self.inner.current_term.fetch_add(1, Ordering::SeqCst);
    }

    /// Votes for the current node.
    pub(super) async fn vote_for_self(&self) {
        // TODO(appcypher): We need to persist this to disk.
        self.inner.voted_for.write().await.replace(self.inner.id);
    }

    /// Updates the current term and the ID of the node that the current node voted for in the current term.
    pub(super) async fn update_current_term_and_voted_for(
        &self,
        term: u64,
        candidate_id: NodeId,
    ) -> Result<()> {
        // Persist values to disk first.
        self.inner.store.write().await.store_current_term(term)?;
        self.inner
            .store
            .write()
            .await
            .store_voted_for(candidate_id)?;

        // Update in-memory values.
        self.inner.current_term.store(term, Ordering::SeqCst);
        self.inner.voted_for.write().await.replace(candidate_id);

        Ok(())
    }

    /// Updates the current term of the node.
    pub(super) async fn update_current_term(&self, term: u64) -> Result<()> {
        // Persist value to disk first.
        self.inner.store.write().await.store_current_term(term)?;

        // Update in-memory value.
        self.inner.current_term.store(term, Ordering::SeqCst);

        Ok(())
    }

    /// Update the last time the node heard from the leader.
    pub(super) async fn update_last_heard_from_leader(&self) {
        *self.inner.last_heard_from_leader.write().await = Some(Instant::now());
    }

    /// Updates the leader id.
    pub(super) async fn update_leader_id(&self, leader_id: NodeId) {
        *self.inner.leader_id.write().await = Some(leader_id);
    }

    /// Checks if the current state of the node is `Follower`.
    pub async fn is_follower_state(&self) -> bool {
        matches!(*self.inner.current_state.read().await, TaskState::Follower)
    }

    /// Checks if the current state of the node is `Candidate`.
    pub async fn is_candidate_state(&self) -> bool {
        matches!(*self.inner.current_state.read().await, TaskState::Candidate)
    }

    /// Checks if the current state of the node is `Shutdown`.
    pub async fn is_shutdown_state(&self) -> bool {
        matches!(*self.inner.current_state.read().await, TaskState::Shutdown)
    }

    /// Checks if the current state of the node is `Leader`.
    pub async fn is_leader_state(&self) -> bool {
        matches!(*self.inner.current_state.read().await, TaskState::Leader)
    }

    /// Returns the current term of the node.
    pub fn get_current_term(&self) -> u64 {
        self.inner.current_term.load(Ordering::SeqCst)
    }

    /// Returns the ID of the node.
    pub fn get_id(&self) -> NodeId {
        self.inner.id
    }

    /// Returns the communication channels for the node.
    pub fn get_channels(&self) -> &RaftSideChannels<R, P> {
        &self.inner.channels
    }

    /// Returns the ID of the node that the current node voted for in the current term.
    pub async fn get_voted_for(&self) -> Option<NodeId> {
        *self.inner.voted_for.write().await
    }

    /// Returns the election timeout range.
    pub fn get_election_timeout_range(&self) -> (u64, u64) {
        self.inner.election_timeout_range
    }

    /// Returns the heartbeat interval.
    pub fn get_heartbeat_interval(&self) -> u64 {
        self.inner.heartbeat_interval
    }

    /// Returns the current leader id.
    pub async fn get_leader_id(&self) -> Option<NodeId> {
        *self.inner.leader_id.write().await
    }

    /// Returns the current state of the node.
    pub async fn get_current_state(&self) -> TaskState {
        self.inner.current_state.read().await.clone()
    }

    /// Fetches the peer address of a node in the cluster.
    pub async fn get_peer(&self, id: &NodeId) -> Option<SocketAddr> {
        self.inner
            .store
            .read()
            .await
            .get_membership()
            .get(id)
            .cloned()
    }

    /// Creates a new election timeout.
    pub(super) fn new_election_timeout(&self) -> Timeout {
        let timeout = Timeout::start_range(self.inner.election_timeout_range);

        tracing::debug!(
            id = self.inner.id.to_string(),
            "Starting election timeout: {:?}",
            timeout.get_interval()
        );

        timeout
    }

    /// Sends a request vote RPC to a peer.
    pub(super) async fn send_request_vote_rpc(
        &self,
        peer: NodeId,
        vote_tx: mpsc::UnboundedSender<RequestVoteResponse>,
    ) -> Result<()> {
        let last_log_index = self.inner.store.read().await.get_last_index();
        let last_log_term = self.inner.store.read().await.get_last_term();

        // Create request
        let request = RequestVoteRequest {
            term: self.inner.current_term.load(Ordering::SeqCst),
            candidate_id: self.inner.id,
            last_log_index,
            last_log_term,
        };

        // Response channel.
        let (response_tx, mut response_rx) = mpsc::channel(1);

        let start = Instant::now(); // Start timer

        // Send request
        self.inner
            .channels
            .out_rpc_tx
            .send((peer, PeerRpc::RequestVote(request, response_tx)))?;

        // Wait for response
        let response = response_rx.recv().await.unwrap(); // TODO: Handle error.

        tracing::debug!(
            id = self.inner.id.to_string(),
            term = self.get_current_term(),
            "Request Vote RPC took {:?} roundtrip to: {}, vote: ({}, {})",
            start.elapsed(), // End timer
            peer,
            response.term,
            response.vote_granted
        );

        // Send response
        vote_tx.send(response)?;

        Ok(())
    }

    /// Sends an append entries RPC to a peer.
    pub(super) async fn send_append_entries_rpc(
        &self,
        request: AppendEntriesRequest<R>,
        peer: NodeId,
        append_entries_tx: mpsc::UnboundedSender<AppendEntriesResponse>,
    ) -> Result<()> {
        // Response channel.
        let (response_tx, mut response_rx) = mpsc::channel(1);

        let start = Instant::now(); // Start timer

        // Send request
        self.inner
            .channels
            .out_rpc_tx
            .send((peer, PeerRpc::AppendEntries(request, response_tx)))?;

        // Wait for response
        let response = response_rx.recv().await.unwrap(); // TODO: Handle error.

        tracing::debug!(
            id = self.inner.id.to_string(),
            term = self.get_current_term(),
            "Append Entries RPC took {:?} roundtrip to: {}, entries len: {}",
            start.elapsed(), // End timer
            peer,
            response.len
        );

        // Send response
        append_entries_tx.send(response)?;

        Ok(())
    }

    /// Shuts down the Raft node.
    pub async fn shutdown(&self) -> Result<()> {
        Ok(self.inner.channels.shutdown_tx.send(()).await?)
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S, R, P> Clone for RaftNode<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
