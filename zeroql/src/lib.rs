//! `zeroql` is a multi-paradigm query language for multi-model databases

#![warn(missing_docs)]
#![allow(clippy::module_inception)]
#![recursion_limit = "256"]

mod compiler;
mod error;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use compiler::*;
pub use error::*;
