use zeroql_macros::{backtrack, memoize};

use crate::ast::Ast;

use super::{Parser, ParserResult};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[backtrack(state = self.lexer.state, condition = |r| matches!(r, Ok(None)))]
#[memoize(cache = self.cache, salt = self.lexer.state)]
impl<'a> Parser<'a> {
    /// TODO
    #[backtrack]
    #[memoize]
    pub fn parse_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_raw_lit() // TODO
    }
}
