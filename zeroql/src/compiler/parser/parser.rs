use std::num::NonZeroUsize;

use lru::LruCache;
use zeroql_macros::anykey::AnyKey;

use crate::{
    ast::Ast,
    compiler::reversible::Reversible,
    lexer::{Lexer, LexerState, Token},
    parser::ParserResult,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A [packrat parser][packrat] for the `zeroql` language.
///
/// It is essentially a recursive descent parser that memoizes the results of parsing subexpressions,
/// which allows it to parse any context-free grammar in linear time.
///
/// In addition, the parser also uses state backtracking to handle ambiguous grammars.
///
/// It is based on the grammar defined in the [`./parser.grammar`](./parser.grammar) file.
///
/// [packrat]: https://en.wikipedia.org/wiki/Packrat_parser
pub struct Parser<'a> {
    /// This caches results of parsing subexpressions.
    pub(crate) cache: LruCache<Box<dyn AnyKey>, ParserResult<Option<Ast<'a>>>>,

    /// The lexer that produces tokens from the input stream.
    pub(crate) lexer: Lexer<'a>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a> Parser<'a> {
    /// Creates a new parser for the given input.
    pub fn new(input: &'a str, cache_size: usize) -> Self {
        let lexer = Lexer::from(input);
        let cache = LruCache::new(NonZeroUsize::new(cache_size).unwrap());
        Self { cache, lexer }
    }

    /// Eats a token from the lexer.
    pub fn eat_token(&mut self) -> ParserResult<Option<Token<'a>>> {
        Ok(self.lexer.next_token()?)
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<'a> Reversible for Parser<'a> {
    type State = LexerState;

    fn get_state(&self) -> Self::State {
        self.lexer.get_state()
    }

    fn set_state(&mut self, state: Self::State) {
        self.lexer.set_state(state);
    }
}
