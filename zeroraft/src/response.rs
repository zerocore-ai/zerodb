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

/// `RequestVoteResponse` is a struct representing a response to a request for votes in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestVoteResponse {
    /// The term of the response.
    pub term: u64,
    /// A boolean indicating whether the vote was granted.
    pub vote_granted: bool,
    /// The id of the node that sent the response.
    pub id: NodeId,
}

/// `AppendEntriesResponse` is a struct representing a response to a request to append entries to the log in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    pub term: u64,
    pub success: bool, // TODO(appcypher): Fix fields
    pub id: NodeId,
}

/// `ClientResponse` is a struct representing a response to a client request in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientResponse<P> {
    // pub success: bool,
    // pub message: Message,
    // pub value: Option<P>,
    _phantom: std::marker::PhantomData<P>, // TODO(appcypher): Remove this.
}
