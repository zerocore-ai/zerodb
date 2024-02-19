//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The running state of a Raft node.
#[derive(Clone, Debug, Default)]
pub enum TaskState {
    /// The node is a follower.
    #[default]
    Follower,

    /// The node is a candidate.
    Candidate,

    /// The node is a leader.
    Leader,

    /// Non-voting member.
    NonVoter,

    /// The node is shutting down.
    Shutdown,
}
