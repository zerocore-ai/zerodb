use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::NodeId;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// `Request` is a trait representing a custom command in the Raft consensus protocol.
///
/// This trait is used to allow for flexibility in the specific commands that can be included in a log entry.
/// It requires the implementing type to support serialization, debugging, and cloning.
pub trait Request: Serialize {}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `Command` is an enum representing a command in the Raft consensus protocol.
///
/// This enum is parameterized over a type `R` that implements the `Request` trait, allowing for flexibility in the specific commands that can be included in a log entry.
///
/// It has three variants: `Config`, `JointConfig`, and `ClientRequest`.
///
/// - `Config` represents a configuration command that includes a `MembershipConfig`.
/// - `JointConfig` represents a joint configuration command that includes two `MembershipConfig`.
/// - `ClientRequest` represents a custom command defined by the user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Command<R>
where
    R: Request,
{
    /// A configuration state.
    SingleConfig(Vec<NodeId>),

    /// A transition between multiple configuration states.
    JointConfig(Vec<NodeId>, Vec<NodeId>),

    /// A custom command defined by the user.
    ClientRequest(R),
}
