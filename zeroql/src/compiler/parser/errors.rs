use thiserror::Error;

use crate::lexer::LexerError;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a parser operation.
pub type ParserResult<T> = Result<T, ParserError>;

/// An error that occurred during parsing.
#[derive(Error, Debug, Clone)]
pub enum ParserError {
    /// An error that occurred during lexing.
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError),
}
