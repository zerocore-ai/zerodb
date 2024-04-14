#![warn(missing_docs)]
//! `zerodb` is a multi-model database query engine for multi-tenant applications

mod errors;
mod init;
mod query;
mod service;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod configs;
pub mod stores;
pub mod utils;

pub use errors::*;
pub use init::*;
pub use query::*;
pub use service::*;
pub use stores::*;

//--------------------------------------------------------------------------------------------------
// Re-exports
//--------------------------------------------------------------------------------------------------

pub use zeroraft::{self, NodeId, NodeIdError};
