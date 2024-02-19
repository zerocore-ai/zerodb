use std::{
    collections::HashSet,
    sync::{atomic::AtomicU64, Arc},
};

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    task::TaskState, Log, NodeId, RaftNode, RaftNodeInner, RaftSideChannels, Request, Response,
    Result, DEFAULT_ELECTION_TIMEOUT_RANGE, DEFAULT_HEARTBEAT_INTERVAL,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Builder for a Raft node.
pub struct RaftNodeBuilder<L, R, P, Channels = ()>
where
    L: Log<R>,
    R: Request,
{
    pub(super) _l: std::marker::PhantomData<L>,
    pub(super) _r: std::marker::PhantomData<R>,
    pub(super) _p: std::marker::PhantomData<P>,
    pub(super) id: NodeId,
    pub(super) log: L,
    pub(super) peers: HashSet<NodeId>,
    pub(super) channels: Channels,
    pub(super) election_timeout_range: (u64, u64),
    pub(super) heartbeat_interval: u64,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<L, R, P, Channels> RaftNodeBuilder<L, R, P, Channels>
where
    L: Log<R>,
    R: Request,
    P: Response,
{
    /// Sets the ID of the Raft node.
    pub fn id(self, id: NodeId) -> Self {
        RaftNodeBuilder { id, ..self }
    }

    /// Sets the log of the Raft node.
    pub fn log(self, log: L) -> Self {
        RaftNodeBuilder { log, ..self }
    }

    /// Sets the peers of the Raft node only if the log does not have a membership yet.
    pub fn peers(self, peers: HashSet<NodeId>) -> Self {
        RaftNodeBuilder { peers, ..self }
    }

    /// Sets the election timeout range of the Raft node.
    pub fn election_timeout_range(self, election_timeout_range: (u64, u64)) -> Self {
        RaftNodeBuilder {
            election_timeout_range,
            ..self
        }
    }

    /// Sets the heartbeat interval of the Raft node.
    pub fn heartbeat_interval(self, heartbeat_interval: u64) -> Self {
        RaftNodeBuilder {
            heartbeat_interval,
            ..self
        }
    }

    /// Sets the communication channels for the Raft node.
    pub fn channels(
        self,
        channels: RaftSideChannels<R, P>,
    ) -> RaftNodeBuilder<L, R, P, RaftSideChannels<R, P>> {
        RaftNodeBuilder {
            _l: self._l,
            _r: self._r,
            _p: self._p,
            id: self.id,
            log: self.log,
            peers: self.peers,
            election_timeout_range: self.election_timeout_range,
            heartbeat_interval: self.heartbeat_interval,
            channels,
        }
    }
}

impl<L, R, P> RaftNodeBuilder<L, R, P, RaftSideChannels<R, P>>
where
    L: Log<R>,
    R: Request,
    P: Response,
{
    /// Builds the Raft node.
    pub async fn build(mut self) -> Result<RaftNode<L, R, P>> {
        // Load the current term, voted for from the log.
        let current_term = AtomicU64::new(self.log.load_current_term().await);
        let voted_for = Mutex::new(self.log.load_voted_for().await);

        // Load the membership and filter out the node's own ID.
        // But first, we check if there is no membership yet, in which case we use the provided peers.
        let membership = self.log.get_membership().await;
        let filtered_membership = if membership.is_empty() {
            self.log.set_initial_membership(self.peers.clone()).await?;
            HashSet::from_iter(self.peers.iter().cloned().filter(|id| id != &self.id))
        } else {
            HashSet::from_iter(membership.iter().cloned().filter(|id| id != &self.id))
        };
        let peers = Mutex::new(filtered_membership);

        let inner = Arc::new(RaftNodeInner {
            id: self.id,
            current_term,
            voted_for,
            log: Mutex::new(self.log),
            channels: self.channels,
            current_state: Mutex::new(TaskState::Follower),
            election_timeout_range: self.election_timeout_range,
            heartbeat_interval: self.heartbeat_interval,
            leader_id: Mutex::new(None),
            peers,
            last_heard_from_leader: Mutex::new(None),
        });

        Ok(RaftNode { inner })
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<L, R, P, Channels> Default for RaftNodeBuilder<L, R, P, Channels>
where
    L: Log<R> + Default,
    R: Request,
    Channels: Default,
{
    fn default() -> Self {
        Self {
            _l: std::marker::PhantomData,
            _r: std::marker::PhantomData,
            _p: std::marker::PhantomData,
            id: Uuid::new_v4(),
            log: Default::default(),
            peers: Default::default(),
            election_timeout_range: DEFAULT_ELECTION_TIMEOUT_RANGE,
            heartbeat_interval: DEFAULT_HEARTBEAT_INTERVAL,
            channels: Default::default(),
        }
    }
}
