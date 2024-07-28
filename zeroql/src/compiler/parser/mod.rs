//! Parser module for zeroql compiler.

mod combinator;
mod errors;
mod parse_keywords;
mod parse_literals;
mod parse_operations;
mod parse_operators;
mod parser;
#[cfg(test)]
mod tests;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use combinator::*;
pub use errors::*;
pub use parser::*;
