#![warn(missing_docs)]
//! `zerodb` is a multi-model database query engine for multi-tenant applications

mod client;
mod error;
mod init;
mod message;
mod node;
mod raft;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod config;
pub mod utils;

pub(crate) use client::*;
pub use error::*;
pub use init::*;
pub(crate) use message::*;
pub use node::*;
pub(crate) use raft::*;
