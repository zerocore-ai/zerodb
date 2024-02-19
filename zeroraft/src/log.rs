use std::collections::HashSet;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{Command, MemorySnapshot, NodeId, Snapshot};

use super::Request;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// `Log` is a trait that defines the log component of a Raft consensus algorithm.
///
/// It provides methods for managing log entries and metadata.
#[async_trait]
pub trait Log<R>
where
    R: Request,
{
    type Snapshot: Snapshot;

    //----------------- ENTRIES -----------------------

    /// Appends new entries to the log.
    async fn append_entries(&mut self, entry: Vec<LogEntry<R>>) -> anyhow::Result<()>;

    /// Removes all entries from the log following the given index if there are any.
    async fn remove_entries_after(&mut self, index: u64) -> anyhow::Result<()>;

    /// Returns the log entry at the given index, if it exists.
    async fn get_entry(&self, index: u64) -> Option<&LogEntry<R>>;

    /// Returns an iterator over the log entries within the given range.
    async fn get_entries(&self, start: u64, end: u64) -> Box<dyn Iterator<Item = &LogEntry<R>>>;

    /// Returns the index of the last log entry.
    async fn get_last_index(&self) -> u64;

    /// Returns the term of the last log entry.
    async fn get_last_term(&self) -> u64;

    /// Returns the index of the last committed log entry.
    async fn get_last_commit_index(&self) -> u64;

    /// Returns the index of the last applied log entry.
    async fn get_last_applied_index(&self) -> u64;

    /// Returns the current membership configuration.
    async fn get_membership(&self) -> &HashSet<NodeId>;

    /// Sets the initial membership configuration.
    async fn set_initial_membership(&mut self, membership: HashSet<NodeId>) -> anyhow::Result<()>;

    /// Sets the index of the last committed log entry and applies it to the state machine.
    async fn set_last_commit_index(&mut self, index: u64) -> anyhow::Result<()>;

    //----------------- SNAPSHOT -----------------------

    /// Returns the latest snapshot, if it exists.
    async fn get_snapshot(&self) -> Option<&Self::Snapshot>;

    //------------------- VOTE ------------------------

    /// Returns the ID of the node that this node has voted for in the current term, if it exists.
    async fn load_voted_for(&self) -> Option<NodeId>;

    /// Returns the current term.
    async fn load_current_term(&self) -> u64;

    /// Stores the ID of the node that this node has voted for in the current term.
    async fn store_voted_for(&mut self, voted_for: NodeId) -> anyhow::Result<()>;

    /// Stores the current term.
    async fn store_current_term(&mut self, term: u64) -> anyhow::Result<()>;
}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `LogEntry` is a struct representing an entry in the log of a Raft consensus protocol node.
///
/// Each `LogEntry` contains a term number and a command. The term number is a non-negative integer that increases over time,
/// representing the term in which the entry was created. The command is a specific action that the Raft node needs to execute.
///
/// The `LogEntry` struct is parameterized over a type `C` that implements the `Request` trait, allowing for flexibility in the specific commands that can be included in a log entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry<R>
where
    R: Request,
{
    /// The term of the log entry.
    pub term: u64,

    /// The command of the log entry.
    pub command: Command<R>,
}

/// `MemoryLog` is a struct representing the log of a Raft consensus protocol node stored in memory.
///
/// It contains a vector of `LogEntry` instances. Each `LogEntry` contains a term number and a command.
/// The `MemoryLog` struct is parameterized over a type `C` that implements the `Request` trait, allowing for flexibility in the specific commands that can be included in a log entry.
#[derive(Debug)]
pub struct MemoryLog<R>
where
    R: Request,
{
    /// The log entries.
    entries: Vec<LogEntry<R>>,

    /// Membership of the cluster.
    membership: Vec<NodeId>,

    /// The current term.
    current_term: u64,

    /// The leader voted for in the current term.
    voted_for: Option<NodeId>,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

#[async_trait]
impl<R> Log<R> for MemoryLog<R>
where
    R: Request + Send + Sync,
{
    type Snapshot = MemorySnapshot;

    async fn append_entries(&mut self, entry: Vec<LogEntry<R>>) -> anyhow::Result<()> {
        todo!()
    }

    async fn remove_entries_after(&mut self, index: u64) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_entry(&self, index: u64) -> Option<&LogEntry<R>> {
        todo!()
    }

    async fn get_entries(&self, start: u64, end: u64) -> Box<dyn Iterator<Item = &LogEntry<R>>> {
        todo!()
    }

    async fn get_last_index(&self) -> u64 {
        todo!()
    }

    async fn get_last_term(&self) -> u64 {
        todo!()
    }

    async fn get_last_commit_index(&self) -> u64 {
        todo!()
    }

    async fn get_last_applied_index(&self) -> u64 {
        todo!()
    }

    async fn get_membership(&self) -> &HashSet<NodeId> {
        todo!()
    }

    async fn set_initial_membership(&mut self, membership: HashSet<NodeId>) -> anyhow::Result<()> {
        todo!()
    }

    async fn set_last_commit_index(&mut self, index: u64) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_snapshot(&self) -> Option<&Self::Snapshot> {
        todo!()
    }

    async fn load_voted_for(&self) -> Option<NodeId> {
        todo!()
    }

    async fn load_current_term(&self) -> u64 {
        todo!()
    }

    async fn store_voted_for(&mut self, voted_for: NodeId) -> anyhow::Result<()> {
        todo!()
    }

    async fn store_current_term(&mut self, term: u64) -> anyhow::Result<()> {
        todo!()
    }
}

impl<R> Default for MemoryLog<R>
where
    R: Request,
{
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            membership: Vec::new(),
            current_term: 0,
            voted_for: None,
        }
    }
}
