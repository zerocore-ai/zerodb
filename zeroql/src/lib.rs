//! `zeroql` is a multi-paradigm query language for multi-model databases

#![warn(missing_docs)]
#![allow(clippy::module_inception)]

mod compiler;
mod error;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use compiler::*;
pub use error::*;
