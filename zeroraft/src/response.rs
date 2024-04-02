use serde::{Deserialize, Serialize};

use crate::NodeId;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// `Response` is a trait representing a client response in the Raft consensus protocol.
pub trait Response: Serialize {}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `AppendEntriesResponse` is a struct representing a response to a request to append entries to the log in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    /// The term of the response.
    pub term: u64,

    /// A boolean indicating whether the append entries request was successful.
    pub success: bool,

    /// The id of the node that sent the response.
    pub id: NodeId,

    /// Length of entries appended.
    pub len: u64,

    /// The reason for the response.
    pub reason: AppendEntriesResponseReason,
}

/// `AppendEntriesResponseReason` is an enum representing the reason for a response to a request to append entries to the log in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppendEntriesResponseReason {
    /// The term in the append entries request is older than the current term.
    StaleTerm,

    /// The recipient's log is the same as the leader's log.
    LogMismatch,

    /// Append entries request was successful.
    Ok,
}

/// `RequestVoteResponse` is a struct representing a response to a request for votes in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestVoteResponse {
    /// The term of the response.
    pub term: u64,

    /// A boolean indicating whether the vote was granted.
    pub vote_granted: bool,

    /// The id of the node that sent the response.
    pub id: NodeId,

    /// The reason for the response.
    pub reason: RequestVoteResponseReason,
}

/// `RequestVoteResponseReason` is an enum representing the reason for a response to a request for votes in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestVoteResponseReason {
    /// The term in the vote request is older than the current term.
    StaleTerm,

    /// The recipient has already voted in this term.
    AlreadyVoted,

    /// The candidate's log is not at least as up-to-date as the recipient's log.
    IncompleteLog,

    /// The vote was granted.
    Ok,
}

// TODO(appcypher): Document
pub struct InstallSnapshotResponse {
    // ...
}

/// The `ClientResponse` struct represents a response from the Raft server to a client request.
///
/// It includes information about the success of the request, the reason for failure (if any),
/// the ID of the leader node (if known), and an optional payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientResponse<P> {
    /// Indicates whether the client request was successful.
    pub success: bool,

    /// Provides more information about the outcome of the client request.
    /// This is particularly useful when the request was not successful.
    pub reason: ClientResponseReason,

    /// The ID of the leader node, if known.
    /// This is useful when the client request was redirected or failed because the recipient was not the leader.
    pub leader_id: Option<NodeId>,

    /// An optional payload that contains the response data.
    /// The type and content of the payload depend on the specific client request.
    pub payload: Option<P>,
}

/// `ClientResponseReason` is an enum representing the reason for a response to a client request in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientResponseReason {
    /// There is currently no leader in the Raft cluster.
    NoLeaderYet,

    /// The client request was redirected to another node.
    Redirect,

    /// The client request was successful.
    Ok,
}
