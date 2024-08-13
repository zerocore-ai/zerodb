use zeroql_macros::{backtrack, memoize};

use crate::{
    ast::{Ast, AstKind},
    lexer::{Token, TokenKind},
};

use super::{Parser, ParserResult};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[memoize(cache = self.cache, state = self.lexer.state)]
#[backtrack(state = self.lexer.state, condition = |r| matches!(r, Ok(None)))]
impl<'a> Parser<'a> {
    /// Parses a terminator.
    ///
    /// ```txt
    /// terminator =
    ///     | terminator
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_terminator(&mut self) -> ParserResult<Option<Ast<'a>>> {
        if let Some(Token {
            span,
            kind: TokenKind::Terminator,
        }) = self.eat_token()?
        {
            return Ok(Some(Ast::new(span, AstKind::Temp(None))));
        }

        Ok(None)
    }
}
