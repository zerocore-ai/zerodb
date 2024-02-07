use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    AppendEntriesResponse, ClientResponse, NodeId, Request, RequestVoteResponse, Response,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `AppendEntriesRequest` is a struct representing a request to append entries to the log in the Raft consensus algorithm.
///
/// This struct is used in the `AppendEntries` RPC, which is invoked by the leader to replicate log entries and to provide a form of heartbeat to other nodes.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppendEntriesRequest {
    /// The term of the request.
    pub term: u64,

    /// The ID of the leader.
    pub leader_id: NodeId,
    // pub prev_log_index: u64,
    // pub prev_log_term: u64,
    // pub entries: Vec<u8>,
    // pub leader_commit: u64,
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
    // pub last_log_index: u64,
    // pub last_log_term: u64,
}

/// `SingleConfigState` is a struct representing the state of a configuration in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleConfigState {
    // pub term: u64,
    // pub leader_id: u64,
    // pub entries: Vec<NodeId>,
}

/// `CombinedConfigStates` is a struct representing the transition between multiple configuration states in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedConfigStates {
    // pub term: u64,
    // pub leader_id: u64,
    // pub old_entries: Vec<NodeId>,
    // pub new_entries: Vec<NodeId>,
}

/// `PeerRpc` is an enum representing the different types of RPCs (Remote Procedure Calls) that can be sent between peers in the Raft consensus algorithm.
///
/// `AppendEntries` is used by the leader to replicate log entries and provide a form of heartbeat. `RequestVote` is used by candidates during elections to gather votes.
pub enum PeerRpc {
    /// Append entries to the log.
    AppendEntries(AppendEntriesRequest, mpsc::Sender<AppendEntriesResponse>),
    /// Request votes from other nodes.
    RequestVote(RequestVoteRequest, mpsc::Sender<RequestVoteResponse>),
}

/// `ClientRequest` is a struct representing a request from a client to the Raft consensus algorithm.
pub struct ClientRequest<R, P>(pub R, pub mpsc::Sender<ClientResponse<P>>)
where
    R: Request,
    P: Response;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------
