use std::num::NonZeroUsize;

use lru::LruCache;
use zeroql_macros::{anykey::AnyKey, backtrack, memoize};

use crate::{Ast, Lexer, ParserResult, Token};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A packrat parser for the zeroql language.
///
/// It is essentially a recursive descent parser that memoizes the results of parsing subexpressions,
/// which allows it to parse any context-free grammar in linear time. In addition to that, the parser
/// also uses state backtracking to handle ambiguous grammars.
///
/// It is based on the grammar defined in the `parser.grammar` file.
pub struct Parser<'a> {
    /// This caches results of parsing subexpressions.
    cache: LruCache<Box<dyn AnyKey>, ParserResult<Option<Ast>>>,

    /// The lexer that produces tokens from the input stream.
    lexer: Lexer<'a>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[backtrack(state = self.lexer.cursor, condition = |r| matches!(r, Ok(None)))]
#[memoize(cache = self.cache, salt = self.lexer.cursor)]
impl<'a> Parser<'a> {
    /// Creates a new parser for the given input.
    pub fn new(input: &'a str, cache_size: usize) -> Self {
        let lexer = Lexer::from(input);
        let cache = LruCache::new(NonZeroUsize::new(cache_size).unwrap());
        Self { cache, lexer }
    }

    /// Eat a token from the lexer.
    pub fn eat_token(&mut self) -> ParserResult<Option<Token<'a>>> {
        Ok(self.lexer.next().transpose()?)
    }

    /// TODO: Implement the parser.
    #[backtrack]
    #[memoize]
    pub fn parse_ident(&mut self) -> ParserResult<Option<Ast>> {
        todo!("Implement the parser")
    }
}
