#![warn(missing_docs)]
//! `zerodb` is a multi-model database query engine for multi-tenant applications

mod command;
mod error;
mod init;
mod node;
mod server;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod config;
pub mod utils;

pub use command::*;
pub use error::*;
pub use init::*;
pub use node::*;
pub use server::*;
