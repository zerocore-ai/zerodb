//! # Task

mod candidate;
mod follower;
mod leader;
mod state;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod common;

pub(crate) use candidate::*;
pub(crate) use follower::*;
pub(crate) use leader::*;
pub(crate) use state::*;
