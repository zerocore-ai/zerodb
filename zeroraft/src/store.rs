use std::{collections::HashMap, net::SocketAddr};

use crate::{Command, NodeId, Result, Snapshot};
use serde::{Deserialize, Serialize};

use super::Request;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// `Store` is a trait that manages the log and state of a Raft consensus protocol node.
pub trait Store<R>
where
    R: Request,
{
    type Snapshot: Snapshot;

    //----------------- LOG ENTRIES ------------------

    /// Appends new entries to the log.
    fn append_entries(&mut self, entries: Vec<LogEntry<R>>) -> Result<()>;

    /// Removes all entries from the log following the given index if there are any.
    fn remove_entries_after(&mut self, index: u64) -> Result<()>;

    /// Returns the log entry at the given index, if it exists.
    fn get_entry(&self, index: u64) -> Option<&LogEntry<R>>;

    /// Returns an iterator over the log entries within the given range.
    fn get_entries<'a>(
        &'a self,
        start: u64,
        limit: Option<u64>,
    ) -> Box<dyn Iterator<Item = &LogEntry<R>> + 'a>;

    /// Returns the index of the last log entry.
    ///
    /// Note that entries start at index 1.
    fn get_last_index(&self) -> u64;

    /// Returns the term of the last log entry.
    ///
    /// Note that terms start at index 1.
    fn get_last_term(&self) -> u64;

    /// Returns the index of the last committed log entry.
    fn get_last_commit_index(&self) -> u64;

    /// Returns the index of the last applied log entry.
    fn get_last_applied_index(&self) -> u64;

    /// Returns the current membership configuration.
    fn get_membership(&self) -> &HashMap<NodeId, SocketAddr>;

    /// Sets the initial membership configuration.
    fn set_initial_membership(&mut self, membership: HashMap<NodeId, SocketAddr>) -> Result<()>;

    /// Sets the index of the last committed log entry and applies it to the state machine.
    fn set_last_commit_index(&mut self, index: u64) -> Result<()>;

    //----------------- SNAPSHOT -----------------------

    /// Returns the latest snapshot, if it exists.
    fn get_snapshot(&self) -> Option<&Self::Snapshot>;

    //--------------- VOTE STATE ------------------------

    /// Returns the ID of the node that this node has voted for in the current term, if it exists.
    fn load_voted_for(&self) -> Option<NodeId>;

    /// Returns the current term.
    fn load_current_term(&self) -> u64;

    /// Stores the ID of the node that this node has voted for in the current term.
    fn store_voted_for(&mut self, voted_for: NodeId) -> Result<()>;

    /// Stores the current term.
    fn store_current_term(&mut self, term: u64) -> Result<()>;
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

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<R> PartialEq for LogEntry<R>
where
    R: Request + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.term == other.term && self.command == other.command
    }
}

impl<R> Clone for LogEntry<R>
where
    R: Request + Clone,
{
    fn clone(&self) -> Self {
        Self {
            term: self.term,
            command: self.command.clone(),
        }
    }
}
