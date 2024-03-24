//! # Task

mod role_candidate;
mod role_follower;
mod role_leader;
mod role_nonvoter;
mod state;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod common;

pub(crate) use role_candidate::*;
pub(crate) use role_follower::*;
pub(crate) use role_leader::*;
// pub(crate) use role_nonvoter::*;
pub(crate) use state::*;
