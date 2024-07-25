use zeroql_macros::{backtrack, memoize};

use crate::{
    ast::{Ast, AstKind},
    lexer::{
        Token,
        TokenKind::{self, *},
    },
    parse,
};

use super::{Parser, ParserError, ParserResult};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[backtrack(state = self.lexer.cursor, condition = |r| matches!(r, Ok(None)))]
#[memoize(cache = self.cache, salt = self.lexer.cursor)]
impl<'a> Parser<'a> {
    /// Parse an identifier.
    ///
    /// ```txt
    /// identifier =
    ///     | plain_identifier
    ///     | escaped_identifier
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_identifier(&mut self) -> ParserResult<Option<Ast<'a>>> {
        if let Some(Token { span, kind }) = self.eat_token()? {
            match kind {
                TokenKind::PlainIdentifier(ident) => {
                    return Ok(Some(Ast::new(span, AstKind::Identifier(ident))));
                }
                TokenKind::EscapedIdentifier(ident) => {
                    return Ok(Some(Ast::new(span, AstKind::Identifier(ident))));
                }
                _ => {}
            }
        }

        Ok(None)
    }

    /// Parse a boolean literal.
    ///
    /// ```txt
    /// boolean_lit =
    ///     | (plain_identifier["true"] | plain_identifier["TRUE"])
    ///     | (plain_identifier["false"] | plain_identifier["FALSE"])
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_boolean_lit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_kw "true")
            (arg parse_kw "false")
        ));

        Ok(result.map(|x| x.unwrap_solo()))
    }

    /// Parse a none literal.
    ///
    /// ```txt
    /// none_lit =
    ///     | plain_identifier["none"]
    ///     | plain_identifier["NONE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_none_lit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("none")
    }

    /// Parse a raw literal.
    ///
    /// ```txt
    /// raw_lit =
    ///     | integer_lit
    ///     | float_lit
    ///     | string_lit
    ///     | regex_lit
    ///     | byte_string_lit
    ///     | boolean_lit
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_raw_lit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        if let Some(Token { span, kind }) = self.eat_token()? {
            match kind {
                TokenKind::DecIntegerLiteral(lit) => {
                    let int = lit
                        .parse::<u128>()
                        .map_err(ParserError::InvalidIntegerLiteral)?;
                    return Ok(Some(Ast::new(span, AstKind::IntegerLiteral(int))));
                }
                TokenKind::FloatLiteral(lit) => {
                    return Ok(Some(Ast::new(span, AstKind::FloatLiteral(lit))));
                }
                TokenKind::HexIntegerLiteral(lit) => {
                    let int = lit
                        .parse::<u128>()
                        .map_err(ParserError::InvalidIntegerLiteral)?;
                    return Ok(Some(Ast::new(span, AstKind::IntegerLiteral(int))));
                }
                TokenKind::BinIntegerLiteral(lit) => {
                    let int = lit
                        .parse::<u128>()
                        .map_err(ParserError::InvalidIntegerLiteral)?;
                    return Ok(Some(Ast::new(span, AstKind::IntegerLiteral(int))));
                }
                TokenKind::OctIntegerLiteral(lit) => {
                    let int = lit
                        .parse::<u128>()
                        .map_err(ParserError::InvalidIntegerLiteral)?;
                    return Ok(Some(Ast::new(span, AstKind::IntegerLiteral(int))));
                }
                TokenKind::StringLiteral(lit) => {
                    return Ok(Some(Ast::new(span, AstKind::StringLiteral(lit))));
                }
                TokenKind::RegexLiteral(lit, flags) => {
                    return Ok(Some(Ast::new(span, AstKind::RegexLiteral(lit, flags))));
                }
                TokenKind::ByteStringLiteral(lit) => {
                    return Ok(Some(Ast::new(span, AstKind::ByteStringLiteral(lit))));
                }
                _ => {
                    self.lexer.cursor -= 1; // Backtrack
                    if let Some(ast) = self.parse_boolean_lit()? {
                        return Ok(Some(ast));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Parse a list literal.
    ///
    /// ```txt
    /// list_lit =
    ///     | "[" "]"
    ///     | "[" op ("," op)* ","? "]"
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_list_lit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq (arg parse_tok OpOpenBrace) (arg parse_tok OpCloseBrace))
            (seq
                (arg parse_tok OpOpenBrace)
                parse_raw_lit // TODO
                (many_0 (seq (arg parse_tok OpComma) parse_raw_lit)) // TODO
                (opt (arg parse_tok OpComma))
                (arg parse_tok OpCloseBrace)
            )
        ));

        println!("result: {:?}", result);

        // TODO

        Ok(None)
    }

    // list_lit =
    //     | "[" "]"
    //     | "[" op ("," op)* ","? "]"

    // object_lit =
    //     | "{" identifier ":" op ("," identifier ":" op)* ","? "}"

    // tuple_lit =
    //     | "(" op ("," op)* ","? ")"

    // lit =
    //     | raw_lit
    //     | list_lit
    //     | object_lit
    //     | tuple_lit
}
