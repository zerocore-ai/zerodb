use std::num::NonZeroUsize;

use lru::LruCache;
use zeroql_macros::{anykey::AnyKey, backtrack, memoize};

use crate::{Ast, Lexer, Token};

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
    cache: LruCache<Box<dyn AnyKey>, Option<Ast>>,

    /// The lexer that produces tokens from the input stream.
    // lexer: Lexer<'a, Token<'a>>,
    lexer: Lexer<'a>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[backtrack(state = self.lexer.cursor)]
#[memoize(cache = self.cache, salt = self.lexer.cursor)]
impl<'a> Parser<'a> {
    /// Creates a new parser for the given input.
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::from(input);
        let cache = LruCache::new(NonZeroUsize::new(1024).unwrap());
        Self { cache, lexer }
    }

    /// Eat a token from the lexer.
    pub fn eat_token(&mut self) -> Option<Token<'a>> {
        // TODO: Handle error?
        self.lexer.next().map(|result| result.unwrap())
    }

    /// TODO: Implement the parser.
    #[backtrack]
    #[memoize]
    pub fn parse_list_literal(&mut self) -> Option<Ast> {
        todo!("Implement the parser")
    }
}
