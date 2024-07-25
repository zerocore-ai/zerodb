//! Parser module for zeroql compiler.

mod capture;
mod combinator;
mod errors;
mod parse_basic;
mod parse_keywords;
mod parse_operators;
mod parser;
#[cfg(test)]
mod tests;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use capture::*;
pub use combinator::*;
pub use errors::*;
pub use parser::*;
