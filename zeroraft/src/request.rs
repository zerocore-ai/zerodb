use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    AppendEntriesResponse, ClientResponse, LogEntry, NodeId, Request, RequestVoteResponse, Response,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `ClientRequest` is a struct representing a request from a client to the Raft consensus algorithm.
pub struct ClientRequest<R, P>(pub R, pub mpsc::Sender<ClientResponse<P>>)
where
    R: Request,
    P: Response;

/// `AppendEntriesRequest` is a struct representing a request to append entries to the log in the Raft consensus algorithm.
///
/// This struct is used in the `AppendEntries` RPC, which is invoked by the leader to replicate log entries and to provide a form of heartbeat to other nodes.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppendEntriesRequest<R>
where
    R: Request,
{
    /// The term of the request. This is the number of the current term the leader is in,
    /// used to identify obsolete requests.
    pub term: u64,

    /// The id of the leader.
    ///
    /// This is used to redirect clients to the current leader.
    pub leader_id: NodeId,

    /// Last commit index of the leader.
    ///
    /// This is used to update the commit index of the follower.
    pub last_commit_index: u64,

    /// Previous log index.
    ///
    /// This is the index of the log entry immediately preceding the new ones.
    ///
    /// The leader can decrement this in an attempt to get to a point where its log matches a follower.
    pub prev_log_index: u64,

    /// Previous log term.
    ///
    /// This is the term of the prev_log_index entry.
    pub prev_log_term: u64,

    /// Log entries to store.
    ///
    /// These are the entries the leader picked up that are to be saved.
    /// It is empty if the leader is sending a heartbeat.
    pub entries: Vec<LogEntry<R>>,
}

/// `RequestVoteRequest` is a struct representing a request for votes in the Raft consensus algorithm.
///
/// This struct is used in the `RequestVote` RPC, which is invoked by candidates during elections to gather votes from other nodes.
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestVoteRequest {
    /// The term of the request.
    pub term: u64,

    /// The ID of the candidate requesting the vote.
    pub candidate_id: NodeId,

    /// The index of the candidate's last log entry.
    pub last_log_index: u64,

    /// The term of the candidate's last log entry.
    pub last_log_term: u64,
}

// TODO(appcypher): Document
#[derive(Debug, Serialize, Deserialize)]
pub struct InstallSnapshotRequest {
    /// The term of the configuration.
    pub term: u64,

    /// The ID of the leader.
    pub leader_id: NodeId,

    /// The last commit index.
    pub last_commit_index: u64,

    /// The previous log index.
    pub prev_log_index: u64,

    /// The previous log term.
    pub prev_log_term: u64,
    // TODO(appcypher): ...
}

/// `ConfigRequest` is a struct representing a request to change the configuration in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigRequest {
    /// The term of the configuration.
    pub term: u64,

    /// The ID of the leader.
    pub leader_id: NodeId,

    /// The last commit index.
    pub last_commit_index: u64,

    /// The previous log index.
    pub prev_log_index: u64,

    /// The previous log term.
    pub prev_log_term: u64,

    /// The membership configuration.
    pub config: MembershipConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MembershipConfig {
    /// Represents the state of a configuration in the Raft consensus algorithm.
    Single(Vec<NodeId>),

    /// Represents the transition between multiple configuration states in the Raft consensus algorithm.
    Joint(Vec<NodeId>, Vec<NodeId>),
}

/// `PeerRpc` is an enum representing the different types of RPCs (Remote Procedure Calls) that can be sent between peers in the Raft consensus algorithm.
///
/// `AppendEntries` is used by the leader to replicate log entries and provide a form of heartbeat. `RequestVote` is used by candidates during elections to gather votes.
pub enum PeerRpc<R>
where
    R: Request,
{
    /// Append entries to the log.
    AppendEntries(AppendEntriesRequest<R>, mpsc::Sender<AppendEntriesResponse>),

    /// Request votes from other nodes.
    RequestVote(RequestVoteRequest, mpsc::Sender<RequestVoteResponse>),

    /// Install a snapshot.
    InstallSnapshot(InstallSnapshotRequest, mpsc::Sender<()>),

    /// A configuration command.
    Config(ConfigRequest, mpsc::Sender<()>),
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<R> Clone for AppendEntriesRequest<R>
where
    R: Request + Clone,
{
    fn clone(&self) -> Self {
        Self {
            term: self.term,
            leader_id: self.leader_id,
            last_commit_index: self.last_commit_index,
            prev_log_index: self.prev_log_index,
            prev_log_term: self.prev_log_term,
            entries: self.entries.clone(),
        }
    }
}
