use logos::Lexer;
use lru::LruCache;

use crate::{Result, Token};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A packrat parser for the ZeroQL language.
///
/// It is essantially a recursive descent parser that memoizes the results of parsing subexpressions,
/// which allows it to parse any context-free grammar in linear time. In addition to that, the parser
/// also uses state backtracking to handle ambiguous grammars.
///
/// It is based on the grammar defined in the `parser.grammar` file.
pub struct Parser<'a> {
    /// This caches results of parsing subexpressions.
    _cache: LruCache<String, ()>, // TODO: Replace `()` with the actual type of the memoized values

    /// The current position in the input stream.
    _cursor: u64,

    /// The lexer that produces tokens from the input stream.
    _lexer: Lexer<'a, Token<'a>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a> Parser<'a> {
    /// TODO: Document this method.
    pub fn eat_token(&mut self) -> Result<Token<'a>> {
        // self.lexer.next().ok_or_else(|| {
        //     let message = format!("Unexpected end of input at position {}", self.cursor);
        //     self.error(message)
        // })
        todo!()
    }
}
