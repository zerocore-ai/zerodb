//! Lexer module for zeroql compiler.

mod errors;
#[allow(clippy::module_inception)]
mod lexer;
#[cfg(test)]
mod tests;
mod token;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use errors::*;
pub use lexer::*;
pub use token::*;
