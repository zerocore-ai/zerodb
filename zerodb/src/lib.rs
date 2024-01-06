#![warn(missing_docs)]
//! `zerodb` is a multi-model database query engine for multi-tenant applications

mod db;
mod error;
mod init;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod client;
pub mod config;
pub mod peer;
pub mod util;

pub use db::*;
pub use error::*;
pub use init::*;
