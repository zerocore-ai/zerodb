#![warn(missing_docs)]
//! `zeroql` is a multi-paradigm query language for multi-model databases

mod errors;
mod lexer;
mod parser;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use errors::*;
pub use lexer::*;
pub use parser::*;
