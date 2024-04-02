use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{atomic::AtomicU64, Arc},
};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    roles::TaskState, NodeId, RaftNode, RaftNodeInner, RaftSideChannels, Request, Response, Result,
    Store, DEFAULT_ELECTION_TIMEOUT_RANGE, DEFAULT_HEARTBEAT_INTERVAL,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Builder for a Raft node.
pub struct RaftNodeBuilder<S, R, P, Channels = ()>
where
    S: Store<R>,
    R: Request,
{
    _s: std::marker::PhantomData<S>,
    _r: std::marker::PhantomData<R>,
    _p: std::marker::PhantomData<P>,
    id: NodeId,
    store: S,
    seeds: HashMap<NodeId, SocketAddr>,
    channels: Channels,
    election_timeout_range: (u64, u64),
    heartbeat_interval: u64,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S, R, P, Channels> RaftNodeBuilder<S, R, P, Channels>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    /// Sets the ID of the Raft node.
    pub fn id(mut self, id: NodeId) -> Self {
        self.id = id;
        self
    }

    /// Sets the store of the Raft node.
    pub fn store(mut self, store: S) -> Self {
        self.store = store;
        self
    }

    /// Sets the seed peers of the Raft node only if the store does not have a membership yet.
    pub fn seeds(mut self, seeds: HashMap<NodeId, SocketAddr>) -> Self {
        self.seeds = seeds;
        self
    }

    /// Sets the election timeout range of the Raft node.
    pub fn election_timeout_range(mut self, election_timeout_range: (u64, u64)) -> Self {
        self.election_timeout_range = election_timeout_range;
        self
    }

    /// Sets the heartbeat interval of the Raft node.
    pub fn heartbeat_interval(mut self, heartbeat_interval: u64) -> Self {
        self.heartbeat_interval = heartbeat_interval;
        self
    }

    /// Sets the communication channels for the Raft node.
    pub fn channels(
        self,
        channels: RaftSideChannels<R, P>,
    ) -> RaftNodeBuilder<S, R, P, RaftSideChannels<R, P>> {
        RaftNodeBuilder {
            _s: self._s,
            _r: self._r,
            _p: self._p,
            id: self.id,
            store: self.store,
            seeds: self.seeds,
            election_timeout_range: self.election_timeout_range,
            heartbeat_interval: self.heartbeat_interval,
            channels,
        }
    }
}

impl<S, R, P> RaftNodeBuilder<S, R, P, RaftSideChannels<R, P>>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    /// Builds the Raft node.
    pub fn build(mut self) -> Result<RaftNode<S, R, P>> {
        // Load the current term, voted for from the store.
        let current_term = AtomicU64::new(self.store.load_current_term());
        let voted_for = RwLock::new(self.store.load_voted_for());

        // We check if there is no membership yet, in which case we use the provided seeds.
        if self.store.get_membership().is_empty() {
            self.store.set_initial_membership(self.seeds)?;
        }

        let inner = Arc::new(RaftNodeInner {
            id: self.id,
            current_term,
            voted_for,
            store: RwLock::new(self.store),
            channels: self.channels,
            current_state: RwLock::new(TaskState::Follower),
            election_timeout_range: self.election_timeout_range,
            heartbeat_interval: self.heartbeat_interval,
            leader_id: RwLock::new(None),
            last_heard_from_leader: RwLock::new(None),
        });

        Ok(RaftNode { inner })
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S, R, P, Channels> Default for RaftNodeBuilder<S, R, P, Channels>
where
    S: Store<R> + Default,
    R: Request,
    Channels: Default,
{
    fn default() -> Self {
        Self {
            _s: std::marker::PhantomData,
            _r: std::marker::PhantomData,
            _p: std::marker::PhantomData,
            id: Uuid::new_v4(),
            store: Default::default(),
            seeds: Default::default(),
            election_timeout_range: DEFAULT_ELECTION_TIMEOUT_RANGE,
            heartbeat_interval: DEFAULT_HEARTBEAT_INTERVAL,
            channels: Default::default(),
        }
    }
}
