use zeroql_macros::{backtrack, memoize};

use crate::ast::Ast;

use super::{Parser, ParserResult};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[memoize(cache = self.cache, state = self.lexer.state)]
#[backtrack(state = self.lexer.state, condition = |r| matches!(r, Ok(None)))]
impl<'a> Parser<'a> {
    /// TODO
    #[memoize]
    #[backtrack]
    pub fn parse_statement(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_identifier() // TODO
    }
}
