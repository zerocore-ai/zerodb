use zeroql_macros::{backtrack, memoize};

use crate::{
    ast::{Ast, AstKind},
    lexer::{
        Token,
        TokenKind::{self, *},
    },
    parse,
};

use super::{Parser, ParserResult};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[backtrack(state = self.lexer.cursor, condition = |r| matches!(r, Ok(None)))]
#[memoize(cache = self.cache, salt = self.lexer.cursor)]
impl<'a> Parser<'a> {
    /// Parses a token of the given kind.
    pub fn parse_tok(&mut self, token_kind: TokenKind) -> ParserResult<Option<Ast<'a>>> {
        let token = self.eat_token()?;
        if let Some(Token { span, kind }) = token {
            if kind == token_kind {
                return Ok(Some(Ast::new(span, AstKind::Temp)));
            }
        }

        Ok(None)
    }

    /// Parses an operator that is a multiplication operator.
    ///
    /// ```txt
    /// op_mul =
    ///     | op_mul_lexer
    ///     | op_star
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_mul(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpMulLexer)
            (arg parse_tok OpStar)
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is a logical AND operator.
    ///
    /// ```txt
    /// op_and =
    ///     | op_and_lexer
    ///     | plain_identifier["and"]
    ///     | plain_identifier["AND"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_and(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpAndLexer)
            (arg parse_kw "and")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is a logical OR operator.
    ///
    /// ```txt
    /// op_or =
    ///     | op_or_lexer
    ///     | plain_identifier["or"]
    ///     | plain_identifier["OR"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_or(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpOrLexer)
            (arg parse_kw "or")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is an IS operator.
    ///
    /// ```txt
    /// op_is =
    ///     | op_is_lexer
    ///     | plain_identifier["is"]
    ///     | plain_identifier["IS"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_is(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpIsLexer)
            (arg parse_kw "is")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is an IS operator.
    ///
    /// ```txt
    /// op_is_not =
    ///     | op_is_not_lexer
    ///     | plain_identifier["is"]
    ///     | plain_identifier["IS"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_is_not(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpIsNotLexer)
            (arg parse_kw2 "is" "not")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }
    /// Parses an operator that is a NOT operator.
    ///
    /// ```txt
    /// op_not =
    ///     | op_not_lexer
    ///     | plain_identifier["not"]
    ///     | plain_identifier["NOT"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_not(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpNotLexer)
            (arg parse_kw "not")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is an IN operator.
    ///
    /// ```txt
    /// op_in =
    ///     | plain_identifier["in"]
    ///     | plain_identifier["IN"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_in(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("in")
    }

    /// Parses an operator that is a NOT IN operator.
    ///
    /// ```txt
    /// op_not_in =
    ///     | plain_identifier["not"] plain_identifier["in"]
    ///     | plain_identifier["not"] plain_identifier["IN"]
    ///     | plain_identifier["NOT"] plain_identifier["IN"]
    ///     | plain_identifier["NOT"] plain_identifier["in"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_not_in(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw2("not", "in")
    }

    /// Parses an operator that is a CONTAINS operator.
    ///
    /// ```txt
    /// op_contains =
    ///     | op_contains_lexer
    ///     | plain_identifier["contains"]
    ///     | plain_identifier["CONTAINS"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_contains(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpContainsLexer)
            (arg parse_kw "contains")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is a NOT CONTAINS operator.
    ///
    /// ```txt
    /// op_not_contains =
    ///     | op_not_contains_lexer
    ///     | plain_identifier["not"] plain_identifier["contains"]
    ///     | plain_identifier["not"] plain_identifier["CONTAINS"]
    ///     | plain_identifier["NOT"] plain_identifier["CONTAINS"]
    ///     | plain_identifier["NOT"] plain_identifier["contains"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_not_contains(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpNotContainsLexer)
            (arg parse_kw2 "not" "contains")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is a CONTAINS NONE operator.
    ///
    /// ```txt
    /// op_contains_none =
    ///     | op_contains_none_lexer
    ///     | plain_identifier["contains"] plain_identifier["none"]
    ///     | plain_identifier["contains"] plain_identifier["NONE"]
    ///     | plain_identifier["CONTAINS"] plain_identifier["NONE"]
    ///     | plain_identifier["CONTAINS"] plain_identifier["none"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_contains_none(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpContainsNoneLexer)
            (arg parse_kw2 "contains" "none")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is a CONTAINS ALL operator.
    ///
    /// ```txt
    /// op_contains_all =
    ///     | op_contains_all_lexer
    ///     | plain_identifier["contains"] plain_identifier["all"]
    ///     | plain_identifier["contains"] plain_identifier["ALL"]
    ///     | plain_identifier["CONTAINS"] plain_identifier["ALL"]
    ///     | plain_identifier["CONTAINS"] plain_identifier["all"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_contains_all(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpContainsAllLexer)
            (arg parse_kw2 "contains" "all")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is a CONTAINS ANY operator.
    ///
    /// ```txt
    /// op_contains_any =
    ///     | op_contains_any_lexer
    ///     | plain_identifier["contains"] plain_identifier["any"]
    ///     | plain_identifier["contains"] plain_identifier["ANY"]
    ///     | plain_identifier["CONTAINS"] plain_identifier["ANY"]
    ///     | plain_identifier["CONTAINS"] plain_identifier["any"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_contains_any(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpContainsAnyLexer)
            (arg parse_kw2 "contains" "any")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is a MATCH operator.
    ///
    /// ```txt
    /// op_match =
    ///     | op_match_lexer
    ///     | plain_identifier["match"]
    ///     | plain_identifier["MATCH"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_match(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpMatchLexer)
            (arg parse_kw "match")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parses an operator that is a NOT MATCH operator.
    ///
    /// ```txt
    /// op_not_match =
    ///     | op_not_match_lexer
    ///     | plain_identifier["not"] plain_identifier["match"]
    ///     | plain_identifier["not"] plain_identifier["MATCH"]
    ///     | plain_identifier["NOT"] plain_identifier["MATCH"]
    ///     | plain_identifier["NOT"] plain_identifier["match"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_op_not_match(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpNotMatchLexer)
            (arg parse_kw2 "not" "match")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }
}
