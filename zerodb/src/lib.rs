#![warn(missing_docs)]
//! `zerodb` is a multi-model database query engine for multi-tenant applications

mod errors;
mod init;
mod node;
mod store;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod configs;
pub mod utils;

pub use errors::*;
pub use init::*;
pub use node::*;

//--------------------------------------------------------------------------------------------------
// Re-exports
//--------------------------------------------------------------------------------------------------

pub use zeroraft::{self, NodeId, NodeIdError};
