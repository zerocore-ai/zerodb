mod reversible;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod ast;
pub mod lexer;
pub mod parser;

/// A span mostly used to represent the start and end of a token in the input string.
pub type Span = std::ops::Range<usize>;
