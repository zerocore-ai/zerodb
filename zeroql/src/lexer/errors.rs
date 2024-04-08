use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result type for the lexer.
pub type LexerResult<T> = Result<T, LexerError>;

/// An error that occurred during lexing.
#[derive(Error, Debug)]
pub enum LexerError {
    /// An unexpected character was encountered.
    #[error("Unexpected character at position {}: {}", span.start, character)]
    UnexpectedCharacter {
        /// The span of the unexpected character.
        span: std::ops::Range<usize>,

        /// The unexpected character.
        character: char,
    },
}
