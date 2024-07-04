//! Parser module for zeroql compiler.

mod errors;
mod parser;
#[cfg(test)]
mod tests;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use errors::*;
pub use parser::*;
