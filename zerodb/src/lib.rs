#![warn(missing_docs)]
//! `zerodb` is a multi-model database query engine for multi-tenant applications

mod errors;
mod init;
mod node;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod compiler;
pub mod configs;
pub mod stores;
pub mod utils;

pub use errors::*;
pub use init::*;
pub use node::*;
pub use stores::*;

//--------------------------------------------------------------------------------------------------
// Re-exports
//--------------------------------------------------------------------------------------------------

pub use zeroraft::{self, NodeId, NodeIdError};
