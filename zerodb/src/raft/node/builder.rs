use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc,
    },
    time::Instant,
};

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    config::DEFAULT_ELECTION_TIMEOUT_RANGE, raft::node::task::TaskState, Log, NodeId, RaftNode,
    RaftNodeInner, RaftSideChannels, Request, Response,
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
    pub(super) current_term: AtomicU64,
    pub(super) voted_for: Mutex<Option<NodeId>>,
    pub(super) log: L,
    pub(super) current_state: Mutex<TaskState>,
    pub(super) last_heartbeat: Option<Instant>,
    pub(super) election_timeout_range: (u64, u64),
    pub(super) running: AtomicBool,
    pub(super) channels: Channels,
    pub(super) peers: Vec<NodeId>,
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

    /// Sets the current term of the Raft node.
    pub fn current_term(self, current_term: u64) -> Self {
        RaftNodeBuilder {
            current_term: AtomicU64::new(current_term),
            ..self
        }
    }

    /// Sets the ID of the node that the Raft node voted for in the current term.
    pub fn voted_for(self, voted_for: NodeId) -> Self {
        RaftNodeBuilder {
            voted_for: Mutex::new(Some(voted_for)),
            ..self
        }
    }

    /// Sets the log of the Raft node.
    pub fn log(self, log: L) -> Self {
        RaftNodeBuilder { log, ..self }
    }

    /// Sets the current state of the Raft node.
    pub fn current_state(self, current_state: TaskState) -> Self {
        RaftNodeBuilder {
            current_state: Mutex::new(current_state),
            ..self
        }
    }

    /// Sets the election timeout range of the Raft node.
    pub fn election_timeout_range(self, election_timeout_range: (u64, u64)) -> Self {
        RaftNodeBuilder {
            election_timeout_range,
            ..self
        }
    }

    /// Add a peer to the Raft node.
    pub fn add_peer(self, peer: NodeId) -> Self {
        let mut peers = self.peers;
        peers.push(peer);
        RaftNodeBuilder { peers, ..self }
    }

    /// Add peers to the Raft node.
    pub fn peers(self, peers: Vec<NodeId>) -> Self {
        RaftNodeBuilder { peers, ..self }
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
            current_term: self.current_term,
            voted_for: self.voted_for,
            log: self.log,
            current_state: self.current_state,
            last_heartbeat: self.last_heartbeat,
            election_timeout_range: self.election_timeout_range,
            running: self.running,
            peers: self.peers,
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
    pub fn build(self) -> RaftNode<L, R, P> {
        let inner = Arc::new(RaftNodeInner {
            id: self.id,
            current_term: self.current_term,
            voted_for: self.voted_for,
            log: self.log,
            current_state: self.current_state,
            last_heartbeat: self.last_heartbeat,
            election_timeout_range: self.election_timeout_range,
            running: self.running,
            channels: self.channels,
            peers: self.peers,
        });

        RaftNode { inner }
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
            current_term: Default::default(),
            voted_for: Default::default(),
            log: Default::default(),
            current_state: Default::default(),
            last_heartbeat: None,
            election_timeout_range: DEFAULT_ELECTION_TIMEOUT_RANGE,
            running: Default::default(),
            channels: Default::default(),
            peers: Default::default(),
        }
    }
}
