use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};

use zeroraft::{LogEntry, NodeId, Request, Snapshot, Store};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `MemoryStore` is a struct representing the log and memory of a Raft node stored in memory.
///
/// It contains a vector of `LogEntry` instances. Each `LogEntry` contains a term number and a command.
/// The `MemoryStore` struct is parameterized over a type `C` that implements the `Request` trait, allowing for flexibility in the specific commands that can be included in a log entry.
#[derive(Debug)]
pub struct MemoryStore<R>
where
    R: Request,
{
    /// The log entries.
    entries: Vec<LogEntry<R>>,

    // TODO(appcypher): Set internally.
    /// Membership of the cluster.
    membership: HashMap<NodeId, SocketAddr>,

    /// Commit index.
    commit_index: u64,

    // TODO(appcypher): Set internally.
    /// Applied index.
    applied_index: u64,

    // TODO(appcypher): Set internally.
    /// The current term.
    ///
    /// A term of 0 means that the node has not seen a candidate or leader yet.
    current_term: u64,

    /// The leader voted for in the current term.
    voted_for: Option<NodeId>,
}

/// The `MemorySnapshot` struct represents an in-memory snapshot of the state machine and the log at a certain point in time.
/// It is used for log compaction in the Raft consensus algorithm.
pub struct MemorySnapshot {
    /// The index of the last log entry included in this snapshot.
    _last_included_index: u64,

    /// The term of the last log entry included in this snapshot.
    _last_included_term: u64,

    /// The membership configuration at the time of this snapshot.
    _membership: HashSet<NodeId>,

    /// The serialized state machine data at the time of this snapshot.
    _data: Vec<u8>,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Snapshot for MemorySnapshot {}

impl<R> Store<R> for MemoryStore<R>
where
    R: Request + Send + Sync,
{
    type Snapshot = MemorySnapshot;

    fn append_entries(&mut self, entries: Vec<LogEntry<R>>) -> anyhow::Result<()> {
        for e in entries {
            self.entries.push(e);
        }

        Ok(())
    }

    fn remove_entries_after(&mut self, index: u64) -> anyhow::Result<()> {
        self.entries.truncate(index as usize);
        Ok(())
    }

    fn get_entry(&self, index: u64) -> Option<&LogEntry<R>> {
        self.entries.get(index as usize - 1)
    }

    fn get_entries<'a>(&'a self, start: u64) -> Box<dyn Iterator<Item = &LogEntry<R>> + 'a> {
        Box::new(self.entries.iter().skip(start as usize - 1))
    }

    fn get_last_index(&self) -> u64 {
        self.entries.len() as u64
    }

    fn get_last_term(&self) -> u64 {
        self.entries.last().map(|e| e.term).unwrap_or(0)
    }

    fn get_last_commit_index(&self) -> u64 {
        self.commit_index
    }

    fn get_last_applied_index(&self) -> u64 {
        self.applied_index
    }

    fn get_membership(&self) -> &HashMap<NodeId, SocketAddr> {
        &self.membership
    }

    fn set_initial_membership(
        &mut self,
        membership: HashMap<NodeId, SocketAddr>,
    ) -> anyhow::Result<()> {
        self.membership = membership;
        Ok(())
    }

    fn set_last_commit_index(&mut self, index: u64) -> anyhow::Result<()> {
        self.commit_index = index;
        Ok(())
    }

    fn get_snapshot(&self) -> Option<&Self::Snapshot> {
        unimplemented!("get_snapshot not implemented")
    }

    fn load_voted_for(&self) -> Option<NodeId> {
        self.voted_for
    }

    fn load_current_term(&self) -> u64 {
        self.current_term
    }

    fn store_voted_for(&mut self, voted_for: NodeId) -> anyhow::Result<()> {
        self.voted_for = Some(voted_for);
        Ok(())
    }

    fn store_current_term(&mut self, term: u64) -> anyhow::Result<()> {
        self.current_term = term;
        Ok(())
    }
}

impl<R> Default for MemoryStore<R>
where
    R: Request,
{
    fn default() -> Self {
        Self {
            entries: vec![],
            membership: HashMap::new(),
            commit_index: 0,
            applied_index: 0,
            current_term: 0,
            voted_for: None,
        }
    }
}
