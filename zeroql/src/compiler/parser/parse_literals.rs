use itertools::Either;
use zeroql_macros::{backtrack, memoize};

use crate::{
    ast::{Ast, AstKind},
    compiler::reversible::Reversible,
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

#[backtrack(state = self.lexer.state, condition = |r| matches!(r, Ok(None)))]
#[memoize(cache = self.cache, salt = self.lexer.state)]
impl<'a> Parser<'a> {
    /// Parses an identifier.
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

    /// Parses a boolean literal.
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

        let ast = result.map(|x| match x.unwrap_choice() {
            Either::Left(x) => {
                Ast::new(x.unwrap_single().get_span(), AstKind::BooleanLiteral(true))
            }
            Either::Right(x) => {
                Ast::new(x.unwrap_single().get_span(), AstKind::BooleanLiteral(false))
            }
        });

        Ok(ast)
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
        if let Some(ast) = self.parse_kw("none")? {
            return Ok(Some(Ast::new(ast.get_span(), AstKind::NoneLiteral)));
        }

        Ok(None)
    }

    /// Parses a raw literal.
    ///
    /// ```txt
    /// raw_lit =
    ///     | integer_lit
    ///     | float_lit
    ///     | string_lit
    ///     | regex_lit
    ///     | byte_string_lit
    ///     | boolean_lit
    ///     | none_lit
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_raw_lit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let state = self.get_state();
        if let Some(Token { span, kind }) = self.eat_token()? {
            match kind {
                TokenKind::DecIntegerLiteral(lit) => {
                    let int = convert_string_to_int(lit, 10)?;
                    return Ok(Some(Ast::new(span, AstKind::IntegerLiteral(int))));
                }
                TokenKind::FloatLiteral(lit) => {
                    let float = convert_string_to_float(lit)?;
                    return Ok(Some(Ast::new(span, AstKind::FloatLiteral(float))));
                }
                TokenKind::HexIntegerLiteral(lit) => {
                    let int = convert_string_to_int(lit, 16)?;
                    return Ok(Some(Ast::new(span, AstKind::IntegerLiteral(int))));
                }
                TokenKind::BinIntegerLiteral(lit) => {
                    let int = convert_string_to_int(lit, 2)?;
                    return Ok(Some(Ast::new(span, AstKind::IntegerLiteral(int))));
                }
                TokenKind::OctIntegerLiteral(lit) => {
                    let int = convert_string_to_int(lit, 8)?;
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
                    self.set_state(state);
                    if let Some(ast) = self.parse_boolean_lit()? {
                        return Ok(Some(ast));
                    } else if let Some(ast) = self.parse_none_lit()? {
                        return Ok(Some(ast));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Parses a list literal.
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
            (seq (arg parse_tok OpOpenSquareBracket) (arg parse_tok OpCloseSquareBracket))
            (seq
                (arg parse_tok OpOpenSquareBracket)
                parse_op
                (many_0 (seq (arg parse_tok OpComma) parse_op))
                (opt (arg parse_tok OpComma))
                (arg parse_tok OpCloseSquareBracket)
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Either::Left(x) => {
                let (open, close) = x.unwrap_seq2();
                Ast::new(
                    open.unwrap_single().get_span().start..close.unwrap_single().get_span().end,
                    AstKind::ListLiteral(vec![]),
                )
            }
            Either::Right(x) => {
                let (open, op0, ops, _, close) = x.unwrap_seq5();
                let mut op_asts = vec![op0.unwrap_single()];
                for op in ops.unwrap_many() {
                    let (_, op) = op.unwrap_seq2();
                    op_asts.push(op.unwrap_single());
                }

                Ast::new(
                    open.unwrap_single().get_span().start..close.unwrap_single().get_span().end,
                    AstKind::ListLiteral(op_asts),
                )
            }
        });

        Ok(ast)
    }

    /// Parses an object literal.
    ///
    /// ```txt
    /// object_lit =
    ///     | "{" "}"
    ///     | "{" identifier ":" op ("," identifier ":" op)* ","? "}"
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_object_lit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq (arg parse_tok OpOpenBrace) (arg parse_tok OpCloseBrace))
            (seq
                (arg parse_tok OpOpenBrace)
                parse_identifier
                (arg parse_tok OpColon)
                parse_op
                (many_0 (seq (arg parse_tok OpComma) parse_identifier (arg parse_tok OpColon) parse_op))
                (opt (arg parse_tok OpComma))
                (arg parse_tok OpCloseBrace)
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Either::Left(x) => {
                let (open, close) = x.unwrap_seq2();
                Ast::new(
                    open.unwrap_single().get_span().start..close.unwrap_single().get_span().end,
                    AstKind::ObjectLiteral(vec![]),
                )
            }
            Either::Right(x) => {
                let (open, k0, _, v0, kvs, _, close) = x.unwrap_seq7();
                let mut op_asts = vec![(k0.unwrap_single(), v0.unwrap_single())];
                for op in kvs.unwrap_many() {
                    let (_, identifier, _, op) = op.unwrap_seq4();
                    op_asts.push((identifier.unwrap_single(), op.unwrap_single()));
                }

                Ast::new(
                    open.unwrap_single().get_span().start..close.unwrap_single().get_span().end,
                    AstKind::ObjectLiteral(op_asts),
                )
            }
        });

        Ok(ast)
    }

    /// Parses a tuple literal.
    ///
    /// ```txt
    /// tuple_lit =
    ///     | "(" ")"
    ///     | "(" op ("," op)* ","? ")"
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_tuple_lit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq (arg parse_tok OpOpenParen) (arg parse_tok OpCloseParen))
            (seq
                (arg parse_tok OpOpenParen)
                parse_op
                (many_0 (seq (arg parse_tok OpComma) parse_op))
                (opt (arg parse_tok OpComma))
                (arg parse_tok OpCloseParen)
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Either::Left(x) => {
                let (open, close) = x.unwrap_seq2();
                Ast::new(
                    open.unwrap_single().get_span().start..close.unwrap_single().get_span().end,
                    AstKind::TupleLiteral(vec![]),
                )
            }
            Either::Right(x) => {
                let (open, op0, ops, _, close) = x.unwrap_seq5();
                let mut op_asts = vec![op0.unwrap_single()];
                for op in ops.unwrap_many() {
                    let (_, op) = op.unwrap_seq2();
                    op_asts.push(op.unwrap_single());
                }

                Ast::new(
                    open.unwrap_single().get_span().start..close.unwrap_single().get_span().end,
                    AstKind::TupleLiteral(op_asts),
                )
            }
        });

        Ok(ast)
    }

    /// Parses a literal.
    ///
    /// ```txt
    /// lit =
    ///     | raw_lit
    ///     | list_lit
    ///     | object_lit
    ///     | tuple_lit
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_lit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_raw_lit
            (alt
                parse_list_lit
                (alt
                    parse_object_lit
                    parse_tuple_lit
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Either::Left(x) => x.unwrap_single(),
            Either::Right(x) => match x.unwrap_choice() {
                Either::Left(x) => x.unwrap_single(),
                Either::Right(x) => match x.unwrap_choice() {
                    Either::Left(x) => x.unwrap_single(),
                    Either::Right(x) => x.unwrap_single(),
                },
            },
        });

        Ok(ast)
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn convert_string_to_int(str: &str, radix: u32) -> Result<u128, ParserError> {
    let cleaned = str.trim_start_matches('0').replace('_', "");
    let int = u128::from_str_radix(&cleaned, radix)
        .map_err(|e| ParserError::InvalidIntegerLiteral(e, cleaned))?;
    Ok(int)
}

fn convert_string_to_float(str: &str) -> Result<f64, ParserError> {
    let cleaned = str.replace('_', "");
    let float = cleaned
        .parse::<f64>()
        .map_err(|e| ParserError::InvalidFloatLiteral(e, cleaned))?;
    Ok(float)
}
