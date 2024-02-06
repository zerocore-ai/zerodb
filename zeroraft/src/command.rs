use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{CombinedConfigStates, SingleConfigState};

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
/// This enum is parameterized over a type `C` that implements the `CustomCommand` trait, allowing for flexibility in the specific commands that can be included in a log entry.
/// It has three variants: `Config`, `JointConfig`, and `Custom`.
/// `Config` represents a configuration command that includes a `MembershipConfig`.
/// `JointConfig` represents a joint configuration command that includes two `MembershipConfig`.
/// `Custom` represents a custom command defined by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command<R>
where
    R: Request,
{
    /// A configuration state.
    SingleConfigState(SingleConfigState),
    /// A transition between multiple configuration states.
    CombinedConfigStates(CombinedConfigStates),
    /// A custom command defined by the user.
    Client(R),
}
