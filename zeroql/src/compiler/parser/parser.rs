use std::num::NonZeroUsize;

use lru::LruCache;
use zeroql_macros::{anykey::AnyKey, backtrack, memoize};

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
/// This parser employs a recursive descent approach with memoization for subexpression results,
/// enabling it to parse any context-free grammar in linear time. It also utilizes state backtracking
/// to manage ambiguous grammars effectively.
///
/// The grammar rules are defined in the [`./parser.grammar`](./parser.grammar) file.
///
/// ## Important
///
/// Due to its recursive descent nature, this parser is not tail-recursive and may cause stack overflows
/// with large inputs. This limitation is known and there are no immediate plans to address it. To mitigate
/// this risk, it is recommended to run the parser in a separate thread to isolate potential faults.
///
/// [packrat]: https://en.wikipedia.org/wiki/Packrat_parser
pub struct Parser<'a> {
    /// This caches results of parsing subexpressions.
    pub(crate) cache: LruCache<Box<dyn AnyKey>, CacheValue<'a>>,

    /// The lexer that produces tokens from the input stream.
    pub(crate) lexer: Lexer<'a>,
}

/// The value stored in the cache.
type CacheValue<'a> = (ParserResult<Option<Ast<'a>>>, LexerState);

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

    /// Parse program.
    /// TODO
    #[memoize(cache = self.cache, state = self.lexer.state)]
    #[backtrack(state = self.lexer.state, condition = |r| matches!(r, Ok(None)))]
    pub fn parse_program(&mut self) -> ParserResult<Option<Ast<'a>>> {
        Ok(None) // TODO: Implement
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
