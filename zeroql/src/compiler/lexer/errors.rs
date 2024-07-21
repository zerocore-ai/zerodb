use thiserror::Error;

use super::Bracket;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result type for the lexer.
pub type LexerResult<T> = Result<T, LexerError>;

/// An error that occurred during lexing.
#[derive(Error, Debug, Clone)]
pub enum LexerError {
    /// An unexpected character was encountered.
    #[error("Unexpected character at position {}: {}", span.start, character)]
    UnexpectedCharacter {
        /// The span of the unexpected character.
        span: std::ops::Range<usize>,

        /// The unexpected character.
        character: char,
    },

    /// A mismatched bracket was encountered.
    #[error("Mismatched bracket at position {}: expected {:?}, found {}", span.start, expected, found)]
    MismatchedBracket {
        /// The span of the mismatched bracket.
        span: std::ops::Range<usize>,

        /// The expected bracket.
        expected: Option<Bracket>,

        /// The found bracket.
        found: Bracket,
    },

    /// An error that occurred while lexing a module block.
    #[error("Unable to lex module block at position {}, Check if it is missing an 'END' keyword", span.start)]
    UnableToLexModuleBlock {
        /// The span of the module block.
        span: std::ops::Range<usize>,
    },
}
