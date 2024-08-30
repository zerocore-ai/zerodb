use zeroql_macros::{backtrack, memoize};

use crate::{
    ast::{Ast, AstKind::*, Field, TypeSig},
    lexer::TokenKind::*,
    parse,
    parser::parse_expressions::{extract_opt_partial_on_database, extract_partial_type_sig},
};

use super::{
    parse_expressions::{extract_opt_partial_on_namespace, extract_partial_on_table},
    Choice, Combinator, Parser, ParserResult,
};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[memoize(cache = self.cache, state = self.lexer.state)]
#[backtrack(state = self.lexer.state, condition = |r| matches!(r, Ok(None)))]
impl<'a> Parser<'a> {
    /// Parses partial `partial_if_not_exists` syntax.
    ///
    /// ```txt
    /// partial_if_not_exists =
    ///     | kw_if kw_not (kw_exists | kw_exist)
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_if_not_exists(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_if
            parse_kw_not
            (alt parse_kw_exists parse_kw_exist)
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a `DEFINE NAMESPACE` statement
    ///
    /// ```txt
    /// define_namespace_stmt =
    ///     | kw_define (kw_namespace | kw_ns) << partial_if_not_exists? identifier >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_define_namespace_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_define
            (alt parse_kw_namespace parse_kw_ns)
            (perm
                (opt parse_partial_if_not_exists)
                parse_identifier
            )
        ));

        let ast = result.map(|x| {
            let (kw_define, _, perm) = x.unwrap_seq3();
            let (opt_parse_partial_if_not_exists, ident) = perm.unwrap_seq2();

            let kw_define = kw_define.unwrap_single();
            let ident = ident.unwrap_indexed().1.unwrap_single();

            let span_start = kw_define.span.start;
            let mut span_end = ident.span.end;

            let if_not_exists =
                match extract_opt_partial_if_not_exists(*opt_parse_partial_if_not_exists) {
                    Some(end) => {
                        span_end = usize::max(span_end, end);
                        true
                    }
                    None => false,
                };

            Ast::new(
                span_start..span_end,
                DefineNamespace {
                    name: Box::new(ident),
                    if_not_exists,
                },
            )
        });

        Ok(ast)
    }

    /// Parses a `DEFINE DATABASE` statement
    ///
    /// ```txt
    /// define_database_stmt =
    ///     | kw_define (kw_database | kw_db) partial_if_not_exists identifier partial_on_namespace?
    ///     | kw_define (kw_database | kw_db) identifier << partial_if_not_exists? partial_on_namespace? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_define_database_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_define
                (alt parse_kw_database parse_kw_db)
                parse_partial_if_not_exists
                parse_identifier
                (opt parse_partial_on_namespace)
            )
            (seq
                parse_kw_define
                (alt parse_kw_database parse_kw_db)
                parse_identifier
                (perm
                    (opt parse_partial_if_not_exists)
                    (opt parse_partial_on_namespace)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_define, _, partial_if_not_exists, ident, opt_partial_on_namespace) =
                    x.unwrap_seq5();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let if_not_exists = match extract_opt_partial_if_not_exists(*partial_if_not_exists)
                {
                    Some(end) => {
                        span_end = end;
                        true
                    }
                    None => false,
                };

                let namespace = match extract_opt_partial_on_namespace(*opt_partial_on_namespace) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                Ast::new(
                    span_start..span_end,
                    DefineDatabase {
                        name: Box::new(ident),
                        if_not_exists,
                        namespace,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_define, _, ident, perm) = x.unwrap_seq4();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_not_exists, opt_partial_on_namespace) = perm.unwrap_seq2();

                let if_not_exists =
                    match extract_opt_partial_if_not_exists(*opt_partial_if_not_exists) {
                        Some(end) => {
                            span_end = end;
                            true
                        }
                        None => false,
                    };

                let namespace = match extract_opt_partial_on_namespace(*opt_partial_on_namespace) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                Ast::new(
                    span_start..span_end,
                    DefineDatabase {
                        name: Box::new(ident),
                        if_not_exists,
                        namespace,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a partial `field_type` syntax.
    ///
    /// ```txt
    /// partial_field_type =
    ///     | kw_type partial_type_sig
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_field_type(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_type
            parse_partial_type_sig
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a partial `field_value` syntax.
    ///
    /// ```txt
    /// partial_field_value =
    ///     | kw_value range_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_field_value(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_value
            parse_range_op
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a partial `table_field_assert` syntax.
    ///
    /// ```txt
    /// partial_table_field_assert =
    ///     | kw_assert range_op
    #[memoize]
    #[backtrack]
    pub fn parse_partial_table_field_assert(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_assert
            parse_range_op
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a partial `table_field` syntax.
    ///
    /// ```txt
    /// partial_table_field =
    ///     | identifier << partial_field_type partial_field_value? partial_table_field_assert* kw_readonly? kw_unique? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_table_field(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_identifier
            (perm
                parse_partial_field_type
                (opt parse_partial_field_value)
                (many_0 parse_partial_table_field_assert)
                (opt parse_kw_readonly)
                (opt parse_kw_unique)
            )
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a partial `table_fields` syntax.
    ///
    /// ```txt
    /// partial_table_fields =
    ///     | kw_fields partial_table_field ("," partial_table_field)*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_table_fields(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_fields
            parse_partial_table_field
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_partial_table_field
            ))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a `DEFINE TABLE` statement.
    ///
    /// ```txt
    /// define_table_stmt =
    ///     | kw_define kw_table partial_if_not_exists identifier << partial_on_database? partial_table_fields? >>
    ///     | kw_define kw_table identifier << partial_if_not_exists? partial_on_database? partial_table_fields? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_define_table_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_define
                parse_kw_table
                parse_partial_if_not_exists
                parse_identifier
                (perm
                    (opt parse_partial_on_database)
                    (opt parse_partial_table_fields)
                )
            )
            (seq
                parse_kw_define
                parse_kw_table
                parse_identifier
                (perm
                    (opt parse_partial_if_not_exists)
                    (opt parse_partial_on_database)
                    (opt parse_partial_table_fields)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_define, _, partial_if_not_exists, ident, perm) = x.unwrap_seq5();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let if_not_exists =
                    extract_opt_partial_if_not_exists(*partial_if_not_exists).is_some();

                let (opt_partial_on_database, opt_partial_table_fields) = perm.unwrap_seq2();

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = ident.span.end;
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let fields = match extract_opt_partial_table_fields(*opt_partial_table_fields) {
                    Some((fields, end)) => {
                        span_end = usize::max(span_end, end);
                        fields
                    }
                    None => vec![],
                };

                Ast::new(
                    span_start..span_end,
                    DefineTable {
                        name: Box::new(ident),
                        if_not_exists,
                        database,
                        fields,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_define, _, ident, perm) = x.unwrap_seq4();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_not_exists, opt_partial_on_database, opt_partial_table_fields) =
                    perm.unwrap_seq3();

                let if_not_exists =
                    match extract_opt_partial_if_not_exists(*opt_partial_if_not_exists) {
                        Some(end) => {
                            span_end = end;
                            true
                        }
                        None => false,
                    };

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let fields = match extract_opt_partial_table_fields(*opt_partial_table_fields) {
                    Some((fields, end)) => {
                        span_end = usize::max(span_end, end);
                        fields
                    }
                    None => vec![],
                };

                Ast::new(
                    span_start..span_end,
                    DefineTable {
                        name: Box::new(ident),
                        if_not_exists,
                        database,
                        fields,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a `DEFINE EDGE` statement.
    ///
    /// ```txt
    /// define_edge_stmt =
    ///     | kw_define kw_edge partial_if_not_exists identifier << partial_on_database? partial_table_fields? >>
    ///     | kw_define kw_edge identifier << partial_if_not_exists? partial_on_database? partial_table_fields? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_define_edge_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_define
                parse_kw_edge
                parse_partial_if_not_exists
                parse_identifier
                (perm
                    (opt parse_partial_on_database)
                    (opt parse_partial_table_fields)
                )
            )
            (seq
                parse_kw_define
                parse_kw_edge
                parse_identifier
                (perm
                    (opt parse_partial_if_not_exists)
                    (opt parse_partial_on_database)
                    (opt parse_partial_table_fields)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_define, _, partial_if_not_exists, ident, perm) = x.unwrap_seq5();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let if_not_exists =
                    extract_opt_partial_if_not_exists(*partial_if_not_exists).is_some();

                let (opt_partial_on_database, opt_partial_table_fields) = perm.unwrap_seq2();

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = ident.span.end;
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let fields = match extract_opt_partial_table_fields(*opt_partial_table_fields) {
                    Some((fields, end)) => {
                        span_end = usize::max(span_end, end);
                        fields
                    }
                    None => vec![],
                };

                Ast::new(
                    span_start..span_end,
                    DefineEdge {
                        name: Box::new(ident),
                        if_not_exists,
                        database,
                        fields,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_define, _, ident, perm) = x.unwrap_seq4();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_not_exists, opt_partial_on_database, opt_partial_table_fields) =
                    perm.unwrap_seq3();

                let if_not_exists =
                    match extract_opt_partial_if_not_exists(*opt_partial_if_not_exists) {
                        Some(end) => {
                            span_end = end;
                            true
                        }
                        None => false,
                    };

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let fields = match extract_opt_partial_table_fields(*opt_partial_table_fields) {
                    Some((fields, end)) => {
                        span_end = usize::max(span_end, end);
                        fields
                    }
                    None => vec![],
                };

                Ast::new(
                    span_start..span_end,
                    DefineEdge {
                        name: Box::new(ident),
                        if_not_exists,
                        database,
                        fields,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a partial `type_field` syntax.
    ///
    /// ```txt
    /// partial_type_field =
    ///     | identifier partial_field_type
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_type_field(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_identifier
            parse_partial_field_type
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a partial `type_fields` syntax.
    ///
    /// ```txt
    /// partial_type_fields =
    ///     | kw_fields partial_type_field ("," partial_type_field)*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_type_fields(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_fields
            parse_partial_type_field
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_partial_type_field
            ))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a `DEFINE TYPE` statement.
    ///
    /// ```txt
    /// define_type_stmt =
    ///     | kw_define kw_type partial_if_not_exists identifier << partial_on_database? partial_type_fields? >>
    ///     | kw_define kw_type identifier << partial_if_not_exists? partial_on_database? partial_type_fields? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_define_type_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_define
                parse_kw_type
                parse_partial_if_not_exists
                parse_identifier
                (perm
                    (opt parse_partial_on_database)
                    (opt parse_partial_type_fields)
                )
            )
            (seq
                parse_kw_define
                parse_kw_type
                parse_identifier
                (perm
                    (opt parse_partial_if_not_exists)
                    (opt parse_partial_on_database)
                    (opt parse_partial_type_fields)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_define, _, partial_if_not_exists, ident, perm) = x.unwrap_seq5();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let if_not_exists =
                    extract_opt_partial_if_not_exists(*partial_if_not_exists).is_some();

                let (opt_partial_on_database, opt_partial_type_fields) = perm.unwrap_seq2();

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let fields = match extract_opt_partial_type_fields(*opt_partial_type_fields) {
                    Some((fields, end)) => {
                        span_end = usize::max(span_end, end);
                        fields
                    }
                    None => vec![],
                };

                Ast::new(
                    span_start..span_end,
                    DefineType {
                        name: Box::new(ident),
                        if_not_exists,
                        database,
                        fields,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_define, _, ident, perm) = x.unwrap_seq4();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_not_exists, opt_partial_on_database, opt_partial_type_fields) =
                    perm.unwrap_seq3();

                let if_not_exists =
                    match extract_opt_partial_if_not_exists(*opt_partial_if_not_exists) {
                        Some(end) => {
                            span_end = end;
                            true
                        }
                        None => false,
                    };

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let fields = match extract_opt_partial_type_fields(*opt_partial_type_fields) {
                    Some((fields, end)) => {
                        span_end = usize::max(span_end, end);
                        fields
                    }
                    None => vec![],
                };

                Ast::new(
                    span_start..span_end,
                    DefineType {
                        name: Box::new(ident),
                        if_not_exists,
                        database,
                        fields,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a partial `enum_variants` syntax.
    ///
    /// ```txt
    /// partial_enum_variants =
    ///     | kw_variants identifier ("," identifier)*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_enum_variants(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_variants
            parse_identifier
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_identifier
            ))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a `DEFINE ENUM` statement.
    ///
    /// ```txt
    /// define_enum_stmt =
    ///     | kw_define kw_enum partial_if_not_exists identifier << partial_on_database? partial_enum_variants? >>
    ///     | kw_define kw_enum identifier << partial_if_not_exists? partial_on_database? partial_enum_variants? >>
    #[memoize]
    #[backtrack]
    pub fn parse_define_enum_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_define
                parse_kw_enum
                parse_partial_if_not_exists
                parse_identifier
                (perm
                    (opt parse_partial_on_database)
                    (opt parse_partial_enum_variants)
                )
            )
            (seq
                parse_kw_define
                parse_kw_enum
                parse_identifier
                (perm
                    (opt parse_partial_if_not_exists)
                    (opt parse_partial_on_database)
                    (opt parse_partial_enum_variants)
                )
            )
        ));

        let ast =
            result.map(|x| match x.unwrap_choice() {
                Choice::A(x) => {
                    let (kw_define, _, partial_if_not_exists, ident, perm) = x.unwrap_seq5();

                    let kw_define = kw_define.unwrap_single();
                    let ident = ident.unwrap_single();

                    let span_start = kw_define.span.start;
                    let mut span_end = ident.span.end;

                    let if_not_exists =
                        extract_opt_partial_if_not_exists(*partial_if_not_exists).is_some();

                    let (opt_partial_on_database, opt_partial_enum_variants) = perm.unwrap_seq2();

                    let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                        Some(ident) => {
                            span_end = usize::max(span_end, ident.span.end);
                            Some(Box::new(ident))
                        }
                        None => None,
                    };

                    let variants =
                        match extract_opt_partial_enum_variants(*opt_partial_enum_variants) {
                            Some((variants, end)) => {
                                span_end = usize::max(span_end, end);
                                variants
                            }
                            None => vec![],
                        };

                    Ast::new(
                        span_start..span_end,
                        DefineEnum {
                            name: Box::new(ident),
                            if_not_exists,
                            database,
                            variants,
                        },
                    )
                }
                Choice::B(x) => {
                    let (kw_define, _, ident, perm) = x.unwrap_seq4();

                    let kw_define = kw_define.unwrap_single();
                    let ident = ident.unwrap_single();

                    let span_start = kw_define.span.start;
                    let mut span_end = ident.span.end;

                    let (
                        opt_partial_if_not_exists,
                        opt_partial_on_database,
                        opt_partial_enum_variants,
                    ) = perm.unwrap_seq3();

                    let if_not_exists =
                        match extract_opt_partial_if_not_exists(*opt_partial_if_not_exists) {
                            Some(end) => {
                                span_end = end;
                                true
                            }
                            None => false,
                        };

                    let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                        Some(ident) => {
                            span_end = usize::max(span_end, ident.span.end);
                            Some(Box::new(ident))
                        }
                        None => None,
                    };

                    let variants =
                        match extract_opt_partial_enum_variants(*opt_partial_enum_variants) {
                            Some((variants, end)) => {
                                span_end = usize::max(span_end, end);
                                variants
                            }
                            None => vec![],
                        };

                    Ast::new(
                        span_start..span_end,
                        DefineEnum {
                            name: Box::new(ident),
                            if_not_exists,
                            database,
                            variants,
                        },
                    )
                }
                _ => unreachable!(),
            });

        Ok(ast)
    }

    /// Parses a partial `index_fields` syntax.
    ///
    /// ```txt
    /// partial_index_fields =
    ///     | kw_fields identifier ("," identifier)*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_index_fields(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_fields
            parse_identifier
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_identifier
            ))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a partial `index_with` syntax.
    ///
    /// ```txt
    /// partial_index_with =
    ///     | kw_with function_call_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_index_with(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_with
            parse_function_call_op
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a `DEFINE INDEX` statement.
    ///
    /// ```txt
    /// define_index_stmt =
    ///     | kw_define kw_index partial_if_not_exists identifier << partial_on_database? partial_on_table partial_index_fields partial_index_with? >>
    ///     | kw_define kw_index identifier << partial_if_not_exists? partial_on_database? partial_on_table partial_index_fields partial_index_with? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_define_index_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_define
                parse_kw_index
                parse_partial_if_not_exists
                parse_identifier
                (perm
                    (opt parse_partial_on_database)
                    parse_partial_on_table
                    parse_partial_index_fields
                    (opt parse_partial_index_with)
                )
            )
            (seq
                parse_kw_define
                parse_kw_index
                parse_identifier
                (perm
                    (opt parse_partial_if_not_exists)
                    (opt parse_partial_on_database)
                    parse_partial_on_table
                    parse_partial_index_fields
                    (opt parse_partial_index_with)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_define, _, partial_if_not_exists, ident, perm) = x.unwrap_seq5();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let if_not_exists =
                    extract_opt_partial_if_not_exists(*partial_if_not_exists).is_some();

                let (
                    opt_partial_on_database,
                    partial_on_table,
                    partial_index_fields,
                    opt_partial_index_with,
                ) = perm.unwrap_seq4();

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let table = extract_partial_on_table(*partial_on_table);
                span_end = usize::max(span_end, table.span.end);

                let columns = match extract_partial_index_fields(*partial_index_fields) {
                    Some((columns, end)) => {
                        span_end = usize::max(span_end, end);
                        columns
                    }
                    None => vec![],
                };

                let function = match extract_opt_partial_index_with(*opt_partial_index_with) {
                    Some(function_call_op) => {
                        span_end = usize::max(span_end, function_call_op.span.end);
                        Some(Box::new(function_call_op))
                    }
                    None => None,
                };

                Ast::new(
                    span_start..span_end,
                    DefineIndex {
                        name: Box::new(ident),
                        table: Box::new(table),
                        if_not_exists,
                        database,
                        columns,
                        function,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_define, _, ident, perm) = x.unwrap_seq4();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let (
                    opt_partial_if_not_exists,
                    opt_partial_on_database,
                    partial_on_table,
                    partial_index_fields,
                    opt_partial_index_with,
                ) = perm.unwrap_seq5();

                let if_not_exists =
                    match extract_opt_partial_if_not_exists(*opt_partial_if_not_exists) {
                        Some(end) => {
                            span_end = end;
                            true
                        }
                        None => false,
                    };

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let table = extract_partial_on_table(*partial_on_table);
                span_end = usize::max(span_end, table.span.end);

                let columns = match extract_partial_index_fields(*partial_index_fields) {
                    Some((columns, end)) => {
                        span_end = usize::max(span_end, end);
                        columns
                    }
                    None => vec![],
                };

                let function = match extract_opt_partial_index_with(*opt_partial_index_with) {
                    Some(function_call_op) => {
                        span_end = usize::max(span_end, function_call_op.span.end);
                        Some(Box::new(function_call_op))
                    }
                    None => None,
                };

                Ast::new(
                    span_start..span_end,
                    DefineIndex {
                        name: Box::new(ident),
                        table: Box::new(table),
                        if_not_exists,
                        database,
                        columns,
                        function,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a partial `module_block` syntax.
    ///
    /// ```txt
    /// partial_module_block =
    ///     | kw_with module_block kw_end
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_module_block(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_with
            parse_module_block
            parse_kw_end
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses a `DEFINE MODULE` statement.
    ///
    /// ```txt
    /// define_module_stmt =
    ///     | kw_define (kw_module | kw_mod) partial_if_not_exists identifier << partial_module_block partial_on_database? >>
    ///     | kw_define (kw_module | kw_mod) identifier << partial_if_not_exists? partial_module_block partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_define_module_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_define
                (alt parse_kw_module parse_kw_mod)
                parse_partial_if_not_exists
                parse_identifier
                (perm
                    parse_partial_module_block
                    (opt parse_partial_on_database)
                )
            )
            (seq
                parse_kw_define
                (alt parse_kw_module parse_kw_mod)
                parse_identifier
                (perm
                    (opt parse_partial_if_not_exists)
                    parse_partial_module_block
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_define, _, partial_if_not_exists, ident, perm) = x.unwrap_seq5();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let if_not_exists =
                    extract_opt_partial_if_not_exists(*partial_if_not_exists).is_some();

                let (partial_module_block, opt_partial_on_database) = perm.unwrap_seq2();

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let (block, end) =
                    extract_partial_module_block(*partial_module_block.unwrap_indexed().1);
                span_end = usize::max(span_end, end);

                Ast::new(
                    span_start..span_end,
                    DefineModule {
                        name: Box::new(ident),
                        block: Box::new(block),
                        if_not_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_define, _, ident, perm) = x.unwrap_seq4();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_not_exists, partial_module_block, opt_partial_on_database) =
                    perm.unwrap_seq3();

                let if_not_exists =
                    match extract_opt_partial_if_not_exists(*opt_partial_if_not_exists) {
                        Some(end) => {
                            span_end = end;
                            true
                        }
                        None => false,
                    };

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let (block, end) =
                    extract_partial_module_block(*partial_module_block.unwrap_indexed().1);
                span_end = usize::max(span_end, end);

                Ast::new(
                    span_start..span_end,
                    DefineModule {
                        name: Box::new(ident),
                        block: Box::new(block),
                        if_not_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a `DEFINE PARAM` statement.
    ///
    /// ```txt
    /// define_param_stmt =
    ///     | kw_define kw_param partial_if_not_exists identifier << partial_on_database? partial_field_type? partial_field_value >>
    ///     | kw_define kw_param identifier << partial_if_not_exists? partial_on_database? partial_field_type? partial_field_value >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_define_param_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_define
                parse_kw_param
                parse_partial_if_not_exists
                parse_identifier
                (perm
                    (opt parse_partial_on_database)
                    (opt parse_partial_field_type)
                    parse_partial_field_value
                )
            )
            (seq
                parse_kw_define
                parse_kw_param
                parse_identifier
                (perm
                    (opt parse_partial_if_not_exists)
                    (opt parse_partial_on_database)
                    (opt parse_partial_field_type)
                    parse_partial_field_value
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_define, _, partial_if_not_exists, ident, perm) = x.unwrap_seq5();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let if_not_exists =
                    extract_opt_partial_if_not_exists(*partial_if_not_exists).is_some();

                let (opt_partial_on_database, opt_partial_field_type, partial_field_value) =
                    perm.unwrap_seq3();

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let r#type = match extract_opt_partial_field_type(*opt_partial_field_type) {
                    Some((r#type, end)) => {
                        span_end = usize::max(span_end, end);
                        Some(r#type)
                    }
                    None => None,
                };

                let (value, end) = extract_opt_partial_field_value(*partial_field_value).unwrap();
                span_end = usize::max(span_end, end);

                Ast::new(
                    span_start..span_end,
                    DefineParam {
                        name: Box::new(ident),
                        value: Box::new(value),
                        if_not_exists,
                        database,
                        r#type,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_define, _, ident, perm) = x.unwrap_seq4();

                let kw_define = kw_define.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_define.span.start;
                let mut span_end = ident.span.end;

                let (
                    opt_partial_if_not_exists,
                    opt_partial_on_database,
                    opt_partial_field_type,
                    partial_field_value,
                ) = perm.unwrap_seq4();

                let if_not_exists =
                    match extract_opt_partial_if_not_exists(*opt_partial_if_not_exists) {
                        Some(end) => {
                            span_end = end;
                            true
                        }
                        None => false,
                    };

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                let r#type = match extract_opt_partial_field_type(*opt_partial_field_type) {
                    Some((r#type, end)) => {
                        span_end = usize::max(span_end, end);
                        Some(r#type)
                    }
                    None => None,
                };

                let (value, end) = extract_opt_partial_field_value(*partial_field_value).unwrap();
                span_end = usize::max(span_end, end);

                Ast::new(
                    span_start..span_end,
                    DefineParam {
                        name: Box::new(ident),
                        value: Box::new(value),
                        if_not_exists,
                        database,
                        r#type,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a `DEFINE` statement.
    ///
    /// ```txt
    /// define_stmt =
    ///     | define_namespace_stmt
    ///     | define_database_stmt
    ///     | define_table_stmt
    ///     | define_edge_stmt
    ///     | define_type_stmt
    ///     | define_enum_stmt
    ///     | define_index_stmt
    ///     | define_module_stmt
    ///     | define_param_stmt
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_define_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_define_namespace_stmt
            parse_define_database_stmt
            parse_define_table_stmt
            parse_define_edge_stmt
            parse_define_type_stmt
            parse_define_enum_stmt
            parse_define_index_stmt
            parse_define_module_stmt
            parse_define_param_stmt
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => x.unwrap_single(),
            Choice::B(x) => x.unwrap_single(),
            Choice::C(x) => x.unwrap_single(),
            Choice::D(x) => x.unwrap_single(),
            Choice::E(x) => x.unwrap_single(),
            Choice::F(x) => x.unwrap_single(),
            Choice::G(x) => x.unwrap_single(),
            Choice::H(x) => x.unwrap_single(),
            Choice::I(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a `USE` statement.
    ///
    /// ```txt
    /// use_stmt =
    ///     | kw_use (kw_database | kw_db) identifier
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_use_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_use
            (alt parse_kw_database parse_kw_db)
            parse_identifier
        ));

        let ast = result.map(|x| {
            let (kw_use, _, ident) = x.unwrap_seq3();

            let kw_use = kw_use.unwrap_single();
            let ident = ident.unwrap_single();

            Ast::new(
                kw_use.span.start..ident.span.end,
                Use {
                    database: Box::new(ident),
                },
            )
        });

        Ok(ast)
    }

    /// Parses a statement.
    ///
    /// ```txt
    /// stmt =
    ///     | define_stmt
    ///     | use_stmt
    ///     | kw_break
    ///     | kw_continue
    ///     | kw_return
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_stmt(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_define_stmt
            parse_use_stmt
            parse_kw_break
            parse_kw_continue
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => x.unwrap_single(),
            Choice::B(x) => x.unwrap_single(),
            Choice::C(x) => {
                let break_stmt = x.unwrap_single();
                Ast::new(break_stmt.span, Break)
            }
            Choice::D(x) => {
                let continue_stmt = x.unwrap_single();
                Ast::new(continue_stmt.span, Continue)
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn extract_opt_partial_if_not_exists(comb: Combinator<Ast<'_>>) -> Option<usize> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_if_not_exists) => {
            let (_, _, kw_exists) = partial_if_not_exists
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();

            match kw_exists.unwrap_choice() {
                Choice::A(kw_exists) => Some(kw_exists.unwrap_single().span.end),
                Choice::B(kw_exist) => Some(kw_exist.unwrap_single().span.end),
                _ => unreachable!(),
            }
        }
        Combinator::Single(partial_if_not_exists) => {
            let (_, _, kw_exists) = partial_if_not_exists.unwrap_temp().unwrap_seq3();

            match kw_exists.unwrap_choice() {
                Choice::A(kw_exists) => Some(kw_exists.unwrap_single().span.end),
                Choice::B(kw_exists) => Some(kw_exists.unwrap_single().span.end),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_table_fields(
    comb: Combinator<Ast<'_>>,
) -> Option<(Vec<Field<'_>>, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_table_fields) => {
            let (_, partial_table_field, partial_table_fields) = partial_table_fields
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();

            let (field, mut span_end) = extract_partial_table_field(*partial_table_field);
            let mut fields = vec![field];
            for partial_table_field in partial_table_fields.unwrap_many() {
                let (_, partial_table_field) = partial_table_field.unwrap_seq2();
                let (field, end) = extract_partial_table_field(*partial_table_field);
                fields.push(field);
                span_end = end;
            }

            Some((fields, span_end))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_partial_table_field(comb: Combinator<Ast<'_>>) -> (Field<'_>, usize) {
    let (ident, perm) = comb.unwrap_single().unwrap_temp().unwrap_seq2();
    let (
        partial_field_type,
        opt_partial_field_value,
        partial_field_asserts,
        opt_kw_readonly,
        opt_kw_unique,
    ) = perm.unwrap_seq5();

    let ident = ident.unwrap_single();

    let (r#type, mut span_end) = extract_opt_partial_field_type(*partial_field_type).unwrap();

    let default = match extract_opt_partial_field_value(*opt_partial_field_value) {
        Some((value, end)) => {
            span_end = usize::max(span_end, end);
            Some(Box::new(value))
        }
        None => None,
    };

    let mut assertions = vec![];
    if let Combinator::Indexed(_, partial_field_asserts) = *partial_field_asserts {
        for partial_field_assert in partial_field_asserts.unwrap_many() {
            let (assert, end) = extract_partial_field_assert(partial_field_assert);
            span_end = usize::max(span_end, end);
            assertions.push(assert);
        }
    }

    let readonly = match *opt_kw_readonly {
        Combinator::Void => false,
        Combinator::Indexed(_, kw_readonly) => {
            let kw_readonly = kw_readonly.unwrap_single();
            span_end = usize::max(span_end, kw_readonly.span.end);
            true
        }
        Combinator::Single(kw_readonly) => {
            span_end = usize::max(span_end, kw_readonly.span.end);
            true
        }
        _ => unreachable!(),
    };

    let unique = match *opt_kw_unique {
        Combinator::Void => false,
        Combinator::Indexed(_, kw_unique) => {
            let kw_unique = kw_unique.unwrap_single();
            span_end = usize::max(span_end, kw_unique.span.end);
            true
        }
        _ => unreachable!(),
    };

    let field = Field {
        name: Box::new(ident),
        r#type,
        default,
        assertions,
        readonly,
        unique,
    };

    (field, span_end)
}

pub(crate) fn extract_opt_partial_field_type(
    comb: Combinator<Ast<'_>>,
) -> Option<(TypeSig<'_>, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_field_type) => {
            let (_, partial_type_sig) = partial_field_type
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq2();
            Some(extract_partial_type_sig(*partial_type_sig))
        }
        Combinator::Single(partial_field_type) => {
            let (_, partial_type_sig) = partial_field_type.unwrap_temp().unwrap_seq2();
            Some(extract_partial_type_sig(*partial_type_sig))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_field_value(
    comb: Combinator<Ast<'_>>,
) -> Option<(Ast<'_>, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_field_value) => {
            let (_, range_op) = partial_field_value
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq2();
            let range_op = range_op.unwrap_single();
            let span_end = range_op.span.end;
            Some((range_op, span_end))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_partial_field_assert(comb: Combinator<Ast<'_>>) -> (Ast<'_>, usize) {
    let (_, range_op) = comb.unwrap_single().unwrap_temp().unwrap_seq2();

    let range_op = range_op.unwrap_single();
    let span_end = range_op.span.end;

    (range_op, span_end)
}

pub(crate) fn extract_opt_partial_type_fields(
    comb: Combinator<Ast<'_>>,
) -> Option<(Vec<(Ast<'_>, TypeSig<'_>)>, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_type_fields) => {
            let (_, partial_type_field, partial_type_fields) = partial_type_fields
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();

            let (field, mut span_end) = extract_partial_type_field(*partial_type_field);
            let mut fields = vec![field];
            for partial_type_field in partial_type_fields.unwrap_many() {
                let (_, partial_type_field) = partial_type_field.unwrap_seq2();
                let (field, end) = extract_partial_type_field(*partial_type_field);
                fields.push(field);
                span_end = end;
            }

            Some((fields, span_end))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_partial_type_field(
    comb: Combinator<Ast<'_>>,
) -> ((Ast<'_>, TypeSig<'_>), usize) {
    let (ident, partial_field_type) = comb.unwrap_single().unwrap_temp().unwrap_seq2();

    let ident = ident.unwrap_single();
    let (r#type, span_end) = extract_opt_partial_field_type(*partial_field_type).unwrap();

    ((ident, r#type), span_end)
}

pub(crate) fn extract_opt_partial_enum_variants(
    comb: Combinator<Ast<'_>>,
) -> Option<(Vec<Ast<'_>>, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_enum_variants) => {
            let (_, ident, rest) = partial_enum_variants
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();

            let ident = ident.unwrap_single();
            let mut span_end = ident.span.end;

            let mut ident_asts = vec![ident];
            for x in rest.unwrap_many() {
                let (_, ident) = x.unwrap_seq2();
                let ident = ident.unwrap_single();
                span_end = ident.span.end;
                ident_asts.push(ident);
            }

            Some((ident_asts, span_end))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_partial_index_fields(
    comb: Combinator<Ast<'_>>,
) -> Option<(Vec<Ast<'_>>, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_index_fields) => {
            let (_, ident, rest) = partial_index_fields
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();

            let ident = ident.unwrap_single();
            let mut span_end = ident.span.end;

            let mut ident_asts = vec![ident];
            for x in rest.unwrap_many() {
                let (_, ident) = x.unwrap_seq2();
                let ident = ident.unwrap_single();
                span_end = ident.span.end;
                ident_asts.push(ident);
            }

            Some((ident_asts, span_end))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_index_with(comb: Combinator<Ast<'_>>) -> Option<Ast<'_>> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_index_fields) => {
            let (_, function_call_op) = partial_index_fields
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq2();

            Some(function_call_op.unwrap_single())
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_partial_module_block(comb: Combinator<Ast<'_>>) -> (Ast<'_>, usize) {
    let (_, module_block, end) = comb.unwrap_single().unwrap_temp().unwrap_seq3();

    let module_block = module_block.unwrap_single();
    let span_end = end.unwrap_single().span.end;

    (module_block, span_end)
}
