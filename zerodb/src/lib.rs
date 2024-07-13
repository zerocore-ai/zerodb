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

/// Re-exports for `zeroraft` types and traits.
pub mod raft {
    pub use zeroraft::*;
}

/// Re-exports for `zeroutils_config` types and traits.
pub mod common_config {
    pub use zeroutils_config::*;
}
