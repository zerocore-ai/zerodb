#![warn(missing_docs)]
//! `zerodb` is a multi-model database query engine for multi-tenant applications

mod error;
mod init;
mod query;
mod service;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod config;
pub mod store;
pub mod utils;

pub use error::*;
pub use init::*;
pub use query::*;
pub use service::*;
pub use store::*;

//--------------------------------------------------------------------------------------------------
// Re-exports
//--------------------------------------------------------------------------------------------------

pub use zeroraft::{self, NodeId, NodeIdError};
