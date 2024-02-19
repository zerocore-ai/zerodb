use std::{
    collections::HashSet,
    ops::Deref,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

use tokio::{
    sync::{mpsc, Mutex},
    task::JoinHandle,
};
use uuid::Uuid;

use crate::{
    task::TaskState, AppendEntriesRequest, AppendEntriesResponse, Countdown, Log, MemoryLog,
    PeerRpc, RaftNodeBuilder, RaftSideChannels, Request, RequestVoteRequest, RequestVoteResponse,
    Response, Result,
};

use super::task::{CandidateTasks, FollowerTasks, LeaderTasks};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Node ID.
pub type NodeId = Uuid;

// TODO(appcypher): RPC retries
/// A `RaftNode` represents a single node in a Raft consensus cluster.
///
/// Each node has a unique ID, a current term, a voted_for field to keep track of the node it has voted for, a log to store entries, and a role which can be either `Follower`, `Candidate`, or `Leader`. The role determines how the node responds to incoming requests.
pub struct RaftNode<L, R, P>
where
    L: Log<R>,
    R: Request,
    P: Response,
{
    pub(super) inner: Arc<RaftNodeInner<L, R, P>>,
}

// TODO(appcypher): We need to persist some fields to disk.
/// The inner state of a Raft node.
pub struct RaftNodeInner<L, R, P>
where
    L: Log<R>,
    R: Request,
    P: Response,
{
    /// The unique ID of the node.
    pub(crate) id: NodeId,

    /// The latest term the node has seen.
    pub(crate) current_term: AtomicU64,

    /// The ID of the node that the current node voted for in the current term.
    pub(crate) voted_for: Mutex<Option<NodeId>>,

    /// The log of the node.
    pub(crate) log: Mutex<L>,

    /// The communication channels for the node.
    pub(crate) channels: RaftSideChannels<R, P>,

    /// The current state of the node.
    pub(crate) current_state: Mutex<TaskState>,

    /// Election timeout range.
    pub(crate) election_timeout_range: (u64, u64),

    /// Heartbeat interval.
    pub(crate) heartbeat_interval: u64,

    /// The current leader id.
    pub(crate) leader_id: Mutex<Option<NodeId>>,

    /// The peers in the cluster from the last membership update.
    ///
    /// Current node is filtered out of the membership.
    pub(crate) peers: Mutex<HashSet<NodeId>>,

    /// Last time the node heard from the leader.
    /// Used to prevent unnecessary voting when there is a stable leader.
    pub(crate) last_heard_from_leader: Mutex<Option<Instant>>,
}

/// This is a convenience type alias for a Raft node with an in-memory log.
///
/// When using this type, the log will be stored in memory and will not be persisted to disk.
pub type MemRaftNode<R, P> = RaftNode<MemoryLog<R>, R, P>;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<L, R, P> RaftNode<L, R, P>
where
    L: Log<R> + Send + Sync + 'static,
    R: Request + Send + Sync + 'static,
    P: Response + Send + Sync + 'static,
{
    /// Starts the Raft node.
    pub fn start(&self) -> JoinHandle<Result<()>> {
        let inner = Arc::clone(&self.inner);
        tokio::spawn(async move {
            loop {
                let current_state = inner.current_state.lock().await.clone();

                tracing::debug!(
                    id = inner.id.to_string(),
                    "State changed: {current_state:?}"
                );

                let inner = Arc::clone(&inner);

                match current_state {
                    TaskState::Follower => FollowerTasks::start(inner).await?,
                    TaskState::Candidate => CandidateTasks::start(inner).await?,
                    TaskState::Leader => LeaderTasks::start(inner).await?,
                    TaskState::NonVoter => {
                        // TODO(appcypher): Implement NonVotingMember state
                        todo!("Implement NonVotingMember state")
                    }
                    TaskState::Shutdown => {
                        break;
                    }
                }
            }

            Ok(())
        })
    }

    /// Returns a new `RaftNodeBuilder` instance.
    ///
    /// This lets you configure the Raft node before starting it.
    pub fn builder() -> RaftNodeBuilder<L, R, P>
    where
        L: Default,
    {
        RaftNodeBuilder::default()
    }
}

impl<L, R, P> RaftNodeInner<L, R, P>
where
    L: Log<R>,
    R: Request,
    P: Response,
{
    /// Creates a new election countdown.
    pub(super) fn new_election_countdown(&self) -> Countdown {
        let countdown = Countdown::start_range(self.election_timeout_range);

        tracing::debug!(
            id = self.id.to_string(),
            "Starting election countdown: {:?}",
            countdown.get_interval()
        );

        countdown
    }

    /// Changes the state of the node to `Follower`.
    pub(super) async fn change_to_follower_state(&self) {
        *self.current_state.lock().await = TaskState::Follower;
    }

    /// Changes the state of the node to `Candidate`.
    pub(super) async fn change_to_candidate_state(&self) {
        *self.current_state.lock().await = TaskState::Candidate;
    }

    /// Changes the state of the node to `Leader`.
    pub(super) async fn change_to_leader_state(&self) {
        *self.current_state.lock().await = TaskState::Leader;
    }

    /// Changes the state of the node to `Shutdown`.
    pub(super) async fn change_to_shutdown_state(&self) {
        *self.current_state.lock().await = TaskState::Shutdown;
    }

    /// Increments the current term of the node.
    pub(super) async fn increment_term(&self) {
        tracing::debug!(
            id = self.id.to_string(),
            "Incrementing term: {}",
            self.current_term.load(Ordering::SeqCst),
        );

        // TODO: We need to persist this to disk.
        self.current_term.fetch_add(1, Ordering::SeqCst);
    }

    /// Votes for the current node.
    pub(super) async fn vote_for_self(&self) {
        // TODO(appcypher): We need to persist this to disk.
        self.voted_for.lock().await.replace(self.id);
    }

    /// Updates the current term and the ID of the node that the current node voted for in the current term.
    pub(super) async fn update_current_term_and_voted_for(
        &self,
        term: u64,
        candidate_id: NodeId,
    ) -> Result<()> {
        // Persist values to disk first.
        self.log.lock().await.store_current_term(term).await?;
        self.log.lock().await.store_voted_for(candidate_id).await?;

        // Update in-memory values.
        self.current_term.store(term, Ordering::SeqCst);
        self.voted_for.lock().await.replace(candidate_id);

        Ok(())
    }

    /// Updates the current term of the node.
    pub(super) async fn update_current_term(&self, term: u64) -> Result<()> {
        // Persist value to disk first.
        self.log.lock().await.store_current_term(term).await?;

        // Update in-memory value.
        self.current_term.store(term, Ordering::SeqCst);

        Ok(())
    }

    /// Update the last time the node heard from the leader.
    pub(super) async fn update_last_heard_from_leader(&self) {
        *self.last_heard_from_leader.lock().await = Some(Instant::now());
    }

    /// Updates the leader id.
    pub(super) async fn update_leader_id(&self, leader_id: NodeId) {
        *self.leader_id.lock().await = Some(leader_id);
    }

    /// Checks if the current state of the node is `Follower`.
    pub async fn is_follower_state(&self) -> bool {
        matches!(*self.current_state.lock().await, TaskState::Follower)
    }

    /// Checks if the current state of the node is `Candidate`.
    pub async fn is_candidate_state(&self) -> bool {
        matches!(*self.current_state.lock().await, TaskState::Candidate)
    }

    /// Checks if the current state of the node is `Shutdown`.
    pub async fn is_shutdown_state(&self) -> bool {
        matches!(*self.current_state.lock().await, TaskState::Shutdown)
    }

    /// Checks if the current state of the node is `Leader`.
    pub async fn is_leader_state(&self) -> bool {
        matches!(*self.current_state.lock().await, TaskState::Leader)
    }

    /// Returns the current term of the node.
    pub fn get_current_term(&self) -> u64 {
        self.current_term.load(Ordering::SeqCst)
    }

    /// Returns the ID of the node.
    pub fn get_id(&self) -> NodeId {
        self.id
    }

    /// Returns the communication channels for the node.
    pub fn get_channels(&self) -> &RaftSideChannels<R, P> {
        &self.channels
    }

    /// Returns the ID of the node that the current node voted for in the current term.
    pub async fn get_voted_for(&self) -> Option<NodeId> {
        *self.voted_for.lock().await
    }

    /// Returns the election timeout range.
    pub fn get_election_timeout_range(&self) -> (u64, u64) {
        self.election_timeout_range
    }

    /// Returns the heartbeat interval.
    pub fn get_heartbeat_interval(&self) -> u64 {
        self.heartbeat_interval
    }

    /// Returns the current leader id.
    pub async fn get_leader_id(&self) -> Option<NodeId> {
        *self.leader_id.lock().await
    }

    /// Returns the current state of the node.
    pub async fn get_current_state(&self) -> TaskState {
        self.current_state.lock().await.clone()
    }

    /// Sends a request vote RPC to a peer.
    pub(super) async fn send_request_vote_rpc(
        &self,
        peer: NodeId,
        vote_tx: mpsc::Sender<RequestVoteResponse>,
    ) -> Result<()> {
        // Create request
        let request = RequestVoteRequest {
            term: self.current_term.load(Ordering::SeqCst),
            candidate_id: self.id,
            last_log_index: self.log.lock().await.get_last_index().await,
            last_log_term: self.log.lock().await.get_last_term().await,
        };

        // Response channel.
        let (response_tx, mut response_rx) = mpsc::channel(1);

        let start = Instant::now();

        // Send request
        self.channels
            .out_rpc_tx
            .send((peer, PeerRpc::RequestVote(request, response_tx)))?;

        // Wait for response
        let response = response_rx.recv().await.unwrap();

        tracing::debug!(
            id = self.id.to_string(),
            term = self.get_current_term(),
            "Request Vote RPC took {:?} roundtrip to: {}, vote: ({}, {})",
            start.elapsed(),
            peer,
            response.term,
            response.vote_granted
        );

        // Send response
        vote_tx.send(response).await?;

        Ok(())
    }

    /// Sends an append entries RPC to a peer.
    pub(super) async fn send_append_entries_rpc(
        &self,
        request: AppendEntriesRequest<R>,
        peer: NodeId,
        append_entries_tx: mpsc::Sender<AppendEntriesResponse>,
    ) -> Result<()> {
        // Response channel.
        let (response_tx, mut response_rx) = mpsc::channel(1);

        let start = Instant::now();

        // Send request
        self.channels
            .out_rpc_tx
            .send((peer, PeerRpc::AppendEntries(request, response_tx)))?;

        // Wait for response
        let response = response_rx.recv().await.unwrap();

        tracing::debug!(
            id = self.id.to_string(),
            term = self.get_current_term(),
            "Append Entries RPC took {:?} roundtrip to: {}",
            start.elapsed(),
            peer
        );

        // Send response
        append_entries_tx.send(response).await?;

        Ok(())
    }

    /// Shuts down the Raft node.
    pub async fn shutdown(&self) -> Result<()> {
        Ok(self.channels.shutdown_tx.send(()).await?)
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<L, R, P> Deref for RaftNode<L, R, P>
where
    L: Log<R>,
    R: Request,
    P: Response,
{
    type Target = RaftNodeInner<L, R, P>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
