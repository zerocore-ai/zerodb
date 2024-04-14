use thiserror::Error;

use crate::LexerError;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a parser operation.
pub type ParserResult<T> = Result<T, ParserError>;

/// An error that occurred during parsing.
#[derive(Error, Debug)]
pub enum ParserError {
    /// An error that occurred during lexing.
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError),
}
