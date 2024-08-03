use std::num::NonZeroUsize;

use lru::LruCache;
use regex::Regex;
use zeroql_macros::{anykey::AnyKey, backtrack, memoize};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

struct Parser<'a> {
    cache: LruCache<Box<dyn AnyKey>, (Option<Ast>, usize)>,
    cursor: usize,
    tokens: &'a [&'a str],
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    AddExpr(Box<Ast>, Box<Ast>),
    Integer(String),
}

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() {
    // === Memoize ===
    std::panic::catch_unwind(|| {
        let mut parser = Parser::new(&["1"]);
        parser.parse_integer(); // Eats the first token
        parser.parse_integer(); // Tries to eats the next token but panics
    })
    .unwrap_err();

    let mut parser = Parser::new(&["1"]);
    let ast = parser.parse_integer(); // Eats the first token
    parser.tokens = &["a"]; // Change the tokens
    parser.cursor = 0; // And reset the cursor
    assert_eq!(parser.parse_integer(), ast); // We should get cached value regardless of the tokens

    // === Backtrack ===
    let mut parser = Parser::new(&["1", "+", "2"]);
    let ast = parser.parse_add_expr(); // Parses the expression successfully
    assert_eq!(
        // Returns the expected AST
        ast,
        Some(Ast::AddExpr(
            Box::new(Ast::Integer("1".to_string())),
            Box::new(Ast::Integer("2".to_string()))
        ))
    );
    assert_eq!(parser.cursor, 3); // Cursor is at the end

    let mut parser = Parser::new(&["1", "+", "a"]);
    let ast = parser.parse_add_expr(); // Tries to parse the expression but fails
    assert_eq!(ast, None); // Returns None
    assert_eq!(parser.cursor, 0); // State is reverted
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[memoize(cache = self.cache, state = self.cursor)]
#[backtrack(state = self.cursor, condition = |r| r.is_none())]
impl<'a> Parser<'a> {
    fn new(tokens: &'a [&'a str]) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(10).unwrap()),
            cursor: 0,
            tokens,
        }
    }

    fn eat_token(&mut self) -> &'a str {
        let token = self.tokens[self.cursor];
        self.cursor += 1;
        token
    }

    #[backtrack]
    #[memoize]
    fn parse_add_expr(&mut self) -> Option<Ast> {
        let left = self.parse_integer()?;
        if self.eat_token() != "+" {
            return None;
        }
        let right = self.parse_integer()?;

        Some(Ast::AddExpr(Box::new(left), Box::new(right)))
    }

    #[backtrack]
    #[memoize]
    fn parse_integer(&mut self) -> Option<Ast> {
        let token = self.eat_token();
        Regex::new(r"\d+")
            .unwrap()
            .find(token)
            .map(|m| Ast::Integer(m.as_str().to_string()))
    }
}
