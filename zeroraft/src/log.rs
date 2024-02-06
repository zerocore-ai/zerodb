use super::Request;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// `Log` is a trait that defines the log component of a Raft consensus algorithm.
///
/// It provides methods for appending entries, committing entries, retrieving entries,
/// truncating the log, and managing snapshots. The log stores `Request` objects,
/// which represent the state changes that the distributed system can make.
///
/// Implementations of this trait are used by Raft nodes to manage their logs,
/// which are a critical part of the Raft consensus algorithm's state.
pub trait Log<R>
where
    R: Request,
{
    // fn append(&mut self, entry: LogEntry<R>);
    // fn commit(&mut self, index: u64);
    // fn get_last_index(&self) -> u64;
    // fn get_last_term(&self) -> u64;
    // fn get_entry(&self, index: u64) -> Option<&LogEntry<R>>;
    // fn get_entries(&self, start: u64, end: u64) -> Option<&[LogEntry<R>]>; // Why not iterator?
    // fn truncate(&mut self, index: u64);
    // fn snapshot(&self) -> Option<&Snapshot>;
    // fn set_snapshot(&mut self, snapshot: Snapshot);
    // fn get_snapshot_last_index(&self) -> u64;
    // ----------------------------------------------
    // fn persist_vote(&mut self, term: u64, vote: Option<NodeId>);
    // fn persist_term(&mut self, term: u64);
    // fn get_vote(&self) -> Option<NodeId>;
    // fn get_term(&self) -> u64;
}

// pub trait Snapshot {
//     fn get_last_index(&self) -> u64;
//     fn get_last_term(&self) -> u64;
//     fn get_data(&self) -> &[u8];
// }

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `LogEntry` is a struct representing an entry in the log of a Raft consensus protocol node.
///
/// Each `LogEntry` contains a term number and a command. The term number is a non-negative integer that increases over time,
/// representing the term in which the entry was created. The command is a specific action that the Raft node needs to execute.
///
/// The `LogEntry` struct is parameterized over a type `C` that implements the `Request` trait, allowing for flexibility in the specific commands that can be included in a log entry.
#[derive(Debug)]
pub struct LogEntry<R>
where
    R: Request,
{
    // /// The term of the log entry.
    // term: u64,
    // /// The command of the log entry.
    // command: Command<R>,
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug)]
/// `MemoryLog` is a struct representing the log of a Raft consensus protocol node stored in memory.
///
/// It contains a vector of `LogEntry` instances. Each `LogEntry` contains a term number and a command.
/// The `MemoryLog` struct is parameterized over a type `C` that implements the `Request` trait, allowing for flexibility in the specific commands that can be included in a log entry.
pub struct MemoryLog<R>
where
    R: Request,
{
    // /// The log entries.
    // entries: Vec<LogEntry<R>>,
    _phantom: std::marker::PhantomData<R>,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<R> Log<R> for MemoryLog<R> where R: Request {}

impl<R> Default for MemoryLog<R>
where
    R: Request,
{
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}
