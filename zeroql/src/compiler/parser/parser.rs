use std::num::NonZeroUsize;

use lru::LruCache;
use zeroql_macros::{anykey::AnyKey, backtrack, memoize};

use crate::{
    ast::Ast,
    lexer::{Lexer, Token},
    parser::ParserResult,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A packrat parser for the `zeroql` language.
///
/// It is essentially a recursive descent parser that memoizes the results of parsing subexpressions,
/// which allows it to parse any context-free grammar in linear time.
///
/// In addition, the parser also uses state backtracking to handle ambiguous grammars.
///
/// It is based on the grammar defined in the `./parser.grammar` file.
pub struct Parser<'a> {
    /// This caches results of parsing subexpressions.
    cache: LruCache<Box<dyn AnyKey>, ParserResult<Option<Ast<'a>>>>,

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

    /// Parses a symbol literal.
    #[backtrack]
    #[memoize]
    pub fn parse_symbol_literal(&mut self) -> ParserResult<Option<Ast<'a>>> {
        todo!("parse_symbol_literal");
    }
}

// Need macros to help with the boilerplate, opt(?), many_0(*), many_1(+), seq(,), alt(|)
/*
seq![p:expression, t:OpDot, t:Identifier]
let state = self.lexer.cursor;
let mut result: Option<(Ast, Token, Token)> = None;

if let Some(__expression @ ...) = self.parse_expression() {
   if let Some(__OpDot @ ...) = self.eat_token() {
       if let Some(__Identifier @ ...) = self.eat_token() {
           result = Some((__expression, __OpDot, __Identifier));
       }
   }
}

if result.is_none() {
   self.lexer.cursor = state;
   return Ok(None);
}
*/

/*
alt![p:expression, t:OpDot, t:Identifier]
let state = self.lexer.cursor;

let mut result: Option<(Ast,)> = None;
if let Some(__expression @ ...) = self.parse_expression() {
    result = Some((__expression,));
}

if result.is_none() {
    let mut result: Option<(Token,)> = None;
    if let Some(__OpDot @ ...) = self.eat_token() {
        result = Some((__OpDot,));
    }

    if result.is_none() {
        let mut result: Option<(Token,)> = None;
        if let Some(__Identifier @ ...) = self.eat_token() {
            result = Some((__Identifier,));
        }

        if result.is_none() {
            self.lexer.cursor = state;
            return Ok(None);
        }
    }
}



*/

/*
opt![p:expression, t:OpDot, t:Identifier]
let state = self.lexer.cursor;
let mut vars: Option<(Ast, Token, Token)> = None;

if let Some(__expression @ ...) = self.parse_expression() {
   if let Some(__OpDot @ ...) = self.eat_token() {
       if let Some(__Identifier @ ...) = self.eat_token() {
           vars = Some((__expression, __OpDot, __Identifier));
       }
   }
}

if result.is_none() {
   self.lexer.cursor = state;
}
*/

/*
many_0![p:expression, t:OpDot, t:Identifier]
let mut result: Vec<(Ast, Token, Token)> = vec![];

loop {
    let state = self.lexer.cursor;
    let len = result.len();

    if let Some(__expression @ ...) = self.parse_expression() {
        if let Some(__OpDot @ ...) = self.eat_token() {
            if let Some(__Identifier @ ...) = self.eat_token() {
                vars.push((__expression, __OpDot, __Identifier));
            }
        }
    }

    if result.len() == len {
        self.lexer.cursor = state;
        break;
    }
}
*/

/*
many_1![p:expression, t:OpDot, t:Identifier]
let mut result: Vec<(Ast, Token, Token)> = vec![];
let mut count = 0;

loop {
    let state = self.lexer.cursor;
    let len = result.len();

    if let Some(__expression @ ...) = self.parse_expression() {
        if let Some(__OpDot @ ...) = self.eat_token() {
            if let Some(__Identifier @ ...) = self.eat_token() {
                vars.push((__expression, __OpDot, __Identifier));
            }
        }
    }

    if result.len() == len {
        self.lexer.cursor = state;
        break;
    }

    count += 1;
}

if count == 0 {
    return Ok(None);
}
*/
