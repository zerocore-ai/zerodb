use std::num::{ParseFloatError, ParseIntError};

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

    /// An error parsing u128 integer literal.
    #[error("Invalid integer literal: {0}, value = {1}")]
    InvalidIntegerLiteral(ParseIntError, String),

    /// An error parsing f64 float literal.
    #[error("Invalid float literal: {0}, value = {1}")]
    InvalidFloatLiteral(ParseFloatError, String),
}
