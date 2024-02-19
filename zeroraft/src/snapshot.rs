use std::collections::HashSet;

use crate::NodeId;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

pub trait Snapshot {}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The `MemorySnapshot` struct represents an in-memory snapshot of the state machine and the log at a certain point in time.
/// It is used for log compaction in the Raft consensus algorithm.
pub struct MemorySnapshot {
    /// The index of the last log entry included in this snapshot.
    last_included_index: u64,

    /// The term of the last log entry included in this snapshot.
    last_included_term: u64,

    /// The membership configuration at the time of this snapshot.
    membership: HashSet<NodeId>,

    /// The serialized state machine data at the time of this snapshot.
    data: Vec<u8>,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Snapshot for MemorySnapshot {}
