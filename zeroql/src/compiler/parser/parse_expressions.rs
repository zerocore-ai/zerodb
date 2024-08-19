use std::collections::BTreeMap;

use itertools::Itertools;
use zeroql_macros::{backtrack, memoize};

use crate::{
    ast::{
        Ast, AstKind::*, Direction, ElseIfPart, SelectColumn, SelectTransform, TypeSig,
        UpdateAssign::*,
    },
    lexer::TokenKind::*,
    parse,
    parser::Choice,
};

use super::{parse_operations, Combinator, Parser, ParserResult};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[memoize(cache = self.cache, state = self.lexer.state)]
#[backtrack(state = self.lexer.state, condition = |r| matches!(r, Ok(None)))]
impl<'a> Parser<'a> {
    /// Parses partial `set_object` syntax.
    ///
    /// ```txt
    /// partial_set_object =
    ///     | kw_set object_lit
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_set_object(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq parse_kw_set parse_object_lit));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `set_assign` syntax.
    ///
    /// ```txt
    /// partial_set_assign =
    ///     | kw_set identifier op_is_lexer op ("," identifier op_is_lexer op)*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_set_assign(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_set
            parse_identifier
            (arg parse_tok OpIsLexer)
            parse_op
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_identifier
                (arg parse_tok OpIsLexer)
                parse_op
            ))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses `CREATE` expression.
    ///
    /// ```txt
    /// create_exp =
    ///     | kw_create (id_op | identifier) (partial_set_object | partial_set_assign)
    ///     | kw_create identifier kw_set "(" (identifier ("," identifier)* ","?)? ")" kw_values tuple_lit ("," tuple_lit)*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_create_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_create
                (alt parse_id_op parse_identifier)
                (alt parse_partial_set_object parse_partial_set_assign)
            )
            (seq
                parse_kw_create
                parse_identifier
                parse_kw_set
                (arg parse_tok OpOpenParen)
                (opt (seq
                    parse_identifier
                    (many_0 (seq
                        (arg parse_tok OpComma)
                        parse_identifier
                    ))
                    (opt (arg parse_tok OpComma))
                ))
                (arg parse_tok OpCloseParen)
                parse_kw_values
                (seq
                    parse_tuple_lit
                    (many_0 (seq
                        (arg parse_tok OpComma)
                        parse_tuple_lit
                    ))
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_create, subject, partial_set) = x.unwrap_seq3();

                let subject = match subject.unwrap_choice() {
                    Choice::A(x) => x.unwrap_single(),
                    Choice::B(x) => x.unwrap_single(),
                    _ => unreachable!(),
                };

                let (columns, value, span_end) = match partial_set.unwrap_choice() {
                    Choice::A(partial_set_object) => {
                        let object = partial_set_object
                            .unwrap_single()
                            .unwrap_temp()
                            .unwrap_seq2()
                            .1;

                        let (tuples, span_end) =
                            ast_as!(object.unwrap_single(), ObjectLiteral(tuples));

                        let (columns, value) = tuples.into_iter().unzip();

                        (columns, value, span_end)
                    }
                    Choice::B(partial_set_assign) => {
                        let (_, column, _, value, kvs) = partial_set_assign
                            .unwrap_single()
                            .unwrap_temp()
                            .unwrap_seq5();

                        let column = column.unwrap_single();
                        let value = value.unwrap_single();
                        let mut span_end = value.span.end;

                        let mut columns = vec![column];
                        let mut value_tuple = vec![value];

                        for kv in kvs.unwrap_many() {
                            let (_, column, _, value) = kv.unwrap_seq4();

                            let column = column.unwrap_single();
                            let value = value.unwrap_single();
                            span_end = value.span.end;

                            columns.push(column);
                            value_tuple.push(value);
                        }

                        (columns, value_tuple, span_end)
                    }
                    _ => unreachable!(),
                };

                Ast::new(
                    kw_create.unwrap_single().span.start..span_end,
                    Create {
                        subject: Box::new(subject),
                        values: vec![value],
                        columns,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_create, identifier, _, _, columns, _, _, values) = x.unwrap_seq8();

                let subject = identifier.unwrap_single();

                let columns = match *columns {
                    Combinator::Seq3(column, columns, _) => {
                        let mut column_asts = vec![column.unwrap_single()];
                        for column in columns.unwrap_many() {
                            let (_, column) = column.unwrap_seq2();
                            column_asts.push(column.unwrap_single());
                        }
                        column_asts
                    }
                    Combinator::Void => vec![],
                    _ => unreachable!(),
                };

                let (value_tuple, value_tuples) = values.unwrap_seq2();
                let (value_tuple, mut span_end) = match value_tuple.unwrap_single() {
                    Ast {
                        kind: TupleLiteral(value_tuple),
                        span,
                    } => (value_tuple, span.end),
                    _ => unreachable!(),
                };

                let mut values = vec![value_tuple];
                for value_tuple in value_tuples.unwrap_many() {
                    let (_, value_tuple) = value_tuple.unwrap_seq2();
                    let value_tuple = match value_tuple.unwrap_single() {
                        Ast {
                            kind: TupleLiteral(value_tuple),
                            span,
                        } => {
                            span_end = span.end;
                            value_tuple
                        }
                        _ => unreachable!(),
                    };

                    values.push(value_tuple);
                }

                Ast::new(
                    kw_create.unwrap_single().span.start..span_end,
                    Create {
                        subject: Box::new(subject),
                        values,
                        columns,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `RELATE` expression.
    ///
    /// ```txt
    /// relate_exp =
    ///     | kw_relate relate_op (partial_set_object | partial_set_assign)?
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_relate_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_relate
            parse_relate_op
            (opt (alt
                parse_partial_set_object
                parse_partial_set_assign
            ))
        ));

        let ast = result.map(|x| {
            let (kw_relate, relate_op, partial_set) = x.unwrap_seq3();
            let relate_op = relate_op.unwrap_single();

            let (columns, value, span_end) = match *partial_set {
                Combinator::Void => (vec![], vec![], relate_op.span.end),
                Combinator::Choice(x) => match x {
                    Choice::A(partial_set_object) => {
                        let object = partial_set_object
                            .unwrap_single()
                            .unwrap_temp()
                            .unwrap_seq2()
                            .1;

                        let (tuples, span_end) =
                            ast_as!(object.unwrap_single(), ObjectLiteral(tuples));

                        let (columns, value) = tuples.into_iter().unzip();

                        (columns, value, span_end)
                    }
                    Choice::B(partial_set_assign) => {
                        let (_, column, _, value, kvs) = partial_set_assign
                            .unwrap_single()
                            .unwrap_temp()
                            .unwrap_seq5();

                        let mut columns = vec![column.unwrap_single()];
                        let mut value_tuple = vec![];

                        let value = value.unwrap_single();
                        let mut span_end = value.span.end;
                        value_tuple.push(value);

                        for kv in kvs.unwrap_many() {
                            let (_, column, _, value) = kv.unwrap_seq4();
                            columns.push(column.unwrap_single());

                            let value = value.unwrap_single();
                            span_end = value.span.end;
                            value_tuple.push(value);
                        }

                        (columns, value_tuple, span_end)
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            Ast::new(
                kw_relate.unwrap_single().span.start..span_end,
                Relate {
                    relate_op: Box::new(relate_op),
                    columns,
                    value,
                },
            )
        });

        Ok(ast)
    }

    /// Parses partial `target` syntax.
    ///
    /// ```txt
    /// partial_target =
    ///     | relate_op
    ///     | id_op
    ///     | identifier
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_target(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_relate_op
            parse_id_op
            parse_identifier
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `where_guard` syntax.
    ///
    /// ```txt
    /// partial_where_guard =
    ///     | kw_where (op | op_star)
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_where_guard(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_where
            (alt
                parse_op
                (arg parse_tok OpStar)
            )
        ));

        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));

        Ok(ast)
    }

    /// Parses `DELETE` expression.
    ///
    /// ```txt
    /// delete_exp =
    ///     | kw_delete partial_target partial_where_guard?
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_delete_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_delete
            parse_partial_target
            (opt parse_partial_where_guard)
        ));

        let ast = result.map(|x| {
            let (kw_delete, partial_target, partial_where_guard) = x.unwrap_seq3();

            let partial_target = match partial_target.unwrap_single().unwrap_temp().unwrap_choice()
            {
                Choice::A(relate_op) => relate_op.unwrap_single(),
                Choice::B(id_op) => id_op.unwrap_single(),
                Choice::C(identifier) => identifier.unwrap_single(),
                _ => unreachable!(),
            };

            let (partial_where_guard, span_end) = match *partial_where_guard {
                Combinator::Void => (None, partial_target.span.end),
                Combinator::Single(partial_where_guard) => {
                    let (_, op) = partial_where_guard.unwrap_temp().unwrap_seq2();

                    let op = match op.unwrap_choice() {
                        Choice::A(op) => op.unwrap_single(),
                        Choice::B(op_star) => op_star.unwrap_single(),
                        _ => unreachable!(),
                    };

                    let span_end = op.span.end;

                    (Some(Box::new(op)), span_end)
                }
                _ => unreachable!(),
            };

            Ast::new(
                kw_delete.unwrap_single().span.start..span_end,
                Delete {
                    target: Box::new(partial_target),
                    where_guard: partial_where_guard,
                },
            )
        });

        Ok(ast)
    }

    /// Parses partial `op_update_assign` syntax.
    ///
    /// ```txt
    /// partial_op_update_assign =
    ///     | op_is_lexer
    ///     | op_assign_plus
    ///     | op_assign_minus
    ///     | op_assign_mul
    ///     | op_assign_div
    ///     | op_assign_mod
    ///     | op_assign_pow
    ///     | op_assign_bit_and
    ///     | op_assign_bit_or
    ///     | op_assign_bit_xor
    ///     | op_assign_bit_not
    ///     | op_assign_shl
    ///     | op_assign_shr
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_op_update_assign(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (arg parse_tok OpIsLexer)
            (arg parse_tok OpAssignPlus)
            (arg parse_tok OpAssignMinus)
            (arg parse_tok OpAssignMul)
            (arg parse_tok OpAssignDiv)
            (arg parse_tok OpAssignMod)
            (arg parse_tok OpAssignPow)
            (arg parse_tok OpAssignBitAnd)
            (arg parse_tok OpAssignBitOr)
            (alt
                (arg parse_tok OpAssignBitXor)
                (arg parse_tok OpAssignBitNot)
                (arg parse_tok OpAssignShl)
                (arg parse_tok OpAssignShr)
            )
        ));

        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));

        Ok(ast)
    }

    /// Parses partial `set_update_assign` syntax.
    ///
    /// ```txt
    /// partial_set_update_assign =
    ///     | kw_set identifier partial_op_update_assign op ("," identifier partial_op_update_assign op)*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_set_update_assign(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_set
            parse_identifier
            parse_partial_op_update_assign
            parse_op
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_identifier
                parse_partial_op_update_assign
                parse_op
            ))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses `UPDATE` expression.
    ///
    /// ```txt
    /// update_exp =
    ///     | kw_update partial_target << partial_where_guard? (partial_set_object | partial_set_update_assign) >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_update_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_update
            parse_partial_target
            (perm
                (opt parse_partial_where_guard)
                (alt
                    parse_partial_set_object
                    parse_partial_set_update_assign
                )
            )
        ));

        let ast = result.map(|x| {
            let (kw_update, partial_target, perm) = x.unwrap_seq3();

            let partial_target = match partial_target.unwrap_single().unwrap_temp().unwrap_choice()
            {
                Choice::A(relate_op) => relate_op.unwrap_single(),
                Choice::B(id_op) => id_op.unwrap_single(),
                Choice::C(identifier) => identifier.unwrap_single(),
                _ => unreachable!(),
            };

            let (opt_partial_where_guard, partial_set) = perm.unwrap_seq2();

            let (partial_where_guard, partial_where_guard_span_end) =
                match extract_opt_partial_where_guard(*opt_partial_where_guard) {
                    Some((_, partial_where_guard, partial_where_guard_span_end)) => {
                        (Some(partial_where_guard), partial_where_guard_span_end)
                    }
                    None => (None, partial_target.span.end),
                };

            let (column_ops, partial_set_span_end) =
                match partial_set.unwrap_indexed().1.unwrap_choice() {
                    Choice::A(partial_set_object) => {
                        let object = partial_set_object
                            .unwrap_single()
                            .unwrap_temp()
                            .unwrap_seq2()
                            .1;

                        let (tuples, span_end) =
                            ast_as!(object.unwrap_single(), ObjectLiteral(tuples));

                        let column_ops = tuples
                            .into_iter()
                            .map(|(column, value)| (column, Direct, value))
                            .collect();

                        (column_ops, span_end)
                    }
                    Choice::B(partial_set_update_assign) => {
                        let (_, column, op, value, kvs) = partial_set_update_assign
                            .unwrap_single()
                            .unwrap_temp()
                            .unwrap_seq5();

                        let column = column.unwrap_single();
                        let op = op.unwrap_single();
                        let value = value.unwrap_single();
                        let mut span_end = value.span.end;

                        let column_op = match op.unwrap_temp().unwrap_choice() {
                            Choice::A(_) => (column, Direct, value),
                            Choice::B(_) => (column, Plus, value),
                            Choice::C(_) => (column, Minus, value),
                            Choice::D(_) => (column, Mul, value),
                            Choice::E(_) => (column, Div, value),
                            Choice::F(_) => (column, Mod, value),
                            Choice::G(_) => (column, Pow, value),
                            Choice::H(_) => (column, BitAnd, value),
                            Choice::I(_) => (column, BitOr, value),
                            Choice::J(rest) => match rest.unwrap_choice() {
                                Choice::A(_) => (column, BitXor, value),
                                Choice::B(_) => (column, BitNot, value),
                                Choice::C(_) => (column, Shl, value),
                                Choice::D(_) => (column, Shr, value),
                                _ => unreachable!(),
                            },
                        };

                        let mut column_ops = vec![column_op];
                        for kv in kvs.unwrap_many() {
                            let (_, column, op, value) = kv.unwrap_seq4();

                            let column = column.unwrap_single();
                            let op = op.unwrap_single();
                            let value = value.unwrap_single();
                            span_end = value.span.end;

                            let column_op = match op.unwrap_temp().unwrap_choice() {
                                Choice::A(_) => (column, Direct, value),
                                Choice::B(_) => (column, Plus, value),
                                Choice::C(_) => (column, Minus, value),
                                Choice::D(_) => (column, Mul, value),
                                Choice::E(_) => (column, Div, value),
                                Choice::F(_) => (column, Mod, value),
                                Choice::G(_) => (column, Pow, value),
                                Choice::H(_) => (column, BitAnd, value),
                                Choice::I(_) => (column, BitOr, value),
                                Choice::J(rest) => match rest.unwrap_choice() {
                                    Choice::A(_) => (column, BitXor, value),
                                    Choice::B(_) => (column, BitNot, value),
                                    Choice::C(_) => (column, Shl, value),
                                    Choice::D(_) => (column, Shr, value),
                                    _ => unreachable!(),
                                },
                            };

                            column_ops.push(column_op);
                        }

                        (column_ops, span_end)
                    }
                    _ => unreachable!(),
                };

            Ast::new(
                kw_update.unwrap_single().span.start
                    ..usize::max(partial_where_guard_span_end, partial_set_span_end),
                Update {
                    target: Box::new(partial_target),
                    where_guard: partial_where_guard,
                    column_ops,
                },
            )
        });

        Ok(ast)
    }

    /// Parses partial `partial_select_field_fold` syntax.
    ///
    /// ```txt
    /// partial_select_field_fold =
    ///     | kw_fold range_op partial_as?
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_select_field_fold(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_fold
            parse_range_op
            (opt parse_partial_as)
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_select_field` syntax.
    ///
    /// ```txt
    /// partial_select_field =
    ///     | partial_select_field_fold
    ///     | op
    ///     | op_star
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_select_field(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_partial_select_field_fold
            parse_op
            (arg parse_tok OpStar)
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_select_omit` syntax
    ///
    /// ```txt
    /// partial_select_omit =
    ///     | kw_omit op ("," op)*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_select_omit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_omit
            parse_op
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_op
            ))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_select_fields` syntax.
    ///
    /// ```txt
    /// partial_select_fields =
    ///     | partial_select_field ("," partial_select_field)* (","? partial_select_omit)?
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_select_fields(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_partial_select_field
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_partial_select_field
            ))
            (opt (seq
                (opt (arg parse_tok OpComma))
                parse_partial_select_omit
            ))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_select_from_table_relate_id_as` syntax.
    ///
    /// ```txt
    /// partial_select_from =
    ///     | kw_from op ("," op)*
    ///     | kw_from op_star
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_select_from(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_from
                parse_op
                (many_0 (seq
                    (arg parse_tok OpComma)
                    parse_op
                ))
            )
            (seq parse_kw_from (arg parse_tok OpStar))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_select_with_indices` syntax.
    ///
    /// ```txt
    /// partial_select_with_indices =
    ///     | kw_with (kw_index | kw_indices | kw_indexes) identifier ("," identifier)*
    ///     | kw_with kw_no kw_index
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_select_with_indices(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_with
                (alt parse_kw_index parse_kw_indices parse_kw_indexes)
                parse_identifier
                (many_0 (seq
                    (arg parse_tok OpComma)
                    parse_identifier
                ))
            )
            (seq
                parse_kw_with
                parse_kw_no
                parse_kw_index
            )
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_select_group_by` syntax.
    ///
    /// ```txt
    /// partial_select_group_by =
    ///     | kw_group kw_by? range_op ("," range_op)*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_select_group_by(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_group
            (opt parse_kw_by)
            parse_range_op
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_range_op
            ))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_select_order_by` syntax.
    ///
    /// ```txt
    /// partial_select_order_by =
    ///     | kw_order kw_by? range_op ("," range_op)* (kw_asc | kw_desc)?
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_select_order_by(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_order
            (opt parse_kw_by)
            parse_range_op
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_range_op
            ))
            (opt (alt parse_kw_asc parse_kw_desc))
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_select_start_at` syntax.
    ///
    /// ```txt
    /// partial_select_start_at =
    ///     | kw_start kw_at? range_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_select_start_at(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_start
            (opt parse_kw_at)
            parse_range_op
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_select_limit_to` syntax.
    ///
    /// ```txt
    /// partial_select_limit_to =
    ///     | kw_limit kw_to? range_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_select_limit_to(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_limit
            (opt parse_kw_to)
            parse_range_op
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `select_exp` syntax.
    ///
    /// ```txt
    /// select_exp =
    ///     | kw_select partial_select_fields partial_select_from << partial_where_guard? partial_select_with_indices? partial_select_group_by? partial_select_order_by? partial_select_start_at? partial_select_limit_to? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_select_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_select
            parse_partial_select_fields
            parse_partial_select_from
            (perm
                (opt parse_partial_where_guard)
                (opt parse_partial_select_with_indices)
                (opt parse_partial_select_group_by)
                (opt parse_partial_select_order_by)
                (opt parse_partial_select_start_at)
                (opt parse_partial_select_limit_to)
            )
        ));

        let ast = result.map(|x| {
            let (select, partial_select_fields, partial_select_from, perm) = x.unwrap_seq4();
            let span_start = select.unwrap_single().span.start;

            let (fields, omit) = extract_partial_select_fields(*partial_select_fields);
            let (from, mut span_end) = extract_partial_select_from(*partial_select_from);
            let (
                opt_partial_where_guard,
                opt_partial_select_with_indices,
                opt_partial_select_group_by,
                opt_partial_select_order_by,
                opt_partial_select_start_at,
                opt_partial_select_limit_to,
            ) = perm.unwrap_seq6();

            let mut transforms = BTreeMap::new();
            if let Some((i, transform, end)) =
                extract_opt_partial_select_where_guard(*opt_partial_where_guard)
            {
                transforms.insert(i, transform);
                span_end = usize::max(span_end, end);
            }

            if let Some((i, transform, end)) =
                extract_opt_partial_select_with_indices(*opt_partial_select_with_indices)
            {
                transforms.insert(i, transform);
                span_end = usize::max(span_end, end);
            }

            if let Some((i, transform, end)) =
                extract_opt_partial_select_group_by(*opt_partial_select_group_by)
            {
                transforms.insert(i, transform);
                span_end = usize::max(span_end, end);
            }

            if let Some((i, transform, end)) =
                extract_opt_partial_select_order_by(*opt_partial_select_order_by)
            {
                transforms.insert(i, transform);
                span_end = usize::max(span_end, end);
            }

            if let Some((i, transform, end)) =
                extract_opt_partial_select_start_at(*opt_partial_select_start_at)
            {
                transforms.insert(i, transform);
                span_end = usize::max(span_end, end);
            }

            if let Some((i, transform, end)) =
                extract_opt_partial_select_limit_to(*opt_partial_select_limit_to)
            {
                transforms.insert(i, transform);
                span_end = usize::max(span_end, end);
            }

            Ast::new(
                span_start..span_end,
                Select {
                    fields,
                    from,
                    omit,
                    transforms: transforms.values().cloned().collect_vec(),
                },
            )
        });

        Ok(ast)
    }

    /// Parses partial `partial_if_exists` syntax.
    ///
    /// ```txt
    /// partial_if_exists =
    ///     | kw_if (kw_exists | kw_exist)
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_if_exists(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_if
            (alt parse_kw_exists parse_kw_exist)
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_on_namespace` syntax.
    ///
    /// ```txt
    /// partial_on_namespace =
    ///     | kw_on (kw_namespace | kw_ns) identifier
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_on_namespace(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_on
            (alt
                parse_kw_namespace
                parse_kw_ns
            )
            parse_identifier
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_on_database` syntax.
    ///
    /// ```txt
    /// partial_on_database =
    ///     | kw_on (kw_database | kw_db) identifier
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_on_database(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_on
            (alt
                parse_kw_database
                parse_kw_db
            )
            parse_identifier
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses partial `partial_on_table` syntax.
    ///
    /// ```txt
    /// partial_on_table =
    ///     | kw_on kw_table identifier
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_on_table(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_on
            parse_kw_table
            parse_identifier
        ));
        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));
        Ok(ast)
    }

    /// Parses `REMOVE NAMESPACE` expression.
    ///
    /// ```txt
    /// remove_namespace_exp =
    ///     | kw_remove (kw_namespace | kw_ns) << partial_if_exists? identifier >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_remove_namespace_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_remove
            (alt
                parse_kw_namespace
                parse_kw_ns
            )
            (perm
                (opt parse_partial_if_exists)
                parse_identifier
            )
        ));

        let ast = result.map(|x| {
            let (kw_remove, _, perm) = x.unwrap_seq3();
            let (opt_partial_if_exists, ident) = perm.unwrap_seq2();

            let kw_remove = kw_remove.unwrap_single();
            let ident = ident.unwrap_indexed().1.unwrap_single();

            let span_start = kw_remove.span.start;
            let mut span_end = ident.span.end;

            let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
                Some(end) => {
                    span_end = usize::max(span_end, end);
                    true
                }
                None => false,
            };

            Ast::new(
                span_start..span_end,
                RemoveNamespace {
                    subject: Box::new(ident),
                    if_exists,
                },
            )
        });

        Ok(ast)
    }

    /// Parses `REMOVE DATABASE` expression.
    ///
    /// ```txt
    /// remove_database_exp =
    ///     | kw_remove (kw_database | kw_db) partial_if_exists identifier partial_on_namespace?
    ///     | kw_remove (kw_database | kw_db) identifier << partial_if_exists? partial_on_namespace? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_remove_database_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_remove
                (alt
                    parse_kw_database
                    parse_kw_db
                )
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_namespace)
            )
            (seq
                parse_kw_remove
                (alt
                    parse_kw_database
                    parse_kw_db
                )
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_namespace)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_remove, _, partial_if_exists, ident, opt_partial_on_namespace) =
                    x.unwrap_seq5();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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
                    RemoveDatabase {
                        subject: Box::new(ident),
                        if_exists,
                        namespace,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_remove, _, ident, perm) = x.unwrap_seq4();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_namespace) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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
                    RemoveDatabase {
                        subject: Box::new(ident),
                        if_exists,
                        namespace,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `REMOVE TABLE` expression.
    ///
    /// ```txt
    /// remove_table_exp =
    ///     | kw_remove kw_table partial_if_exists identifier partial_on_database?
    ///     | kw_remove kw_table identifier << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_remove_table_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_remove
                parse_kw_table
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_remove
                parse_kw_table
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_remove, _, partial_if_exists, ident, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveTable {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_remove, _, ident, perm) = x.unwrap_seq4();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveTable {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `REMOVE EDGE` expression.
    ///
    /// ```txt
    /// remove_edge_exp =
    ///     | kw_remove kw_edge partial_if_exists identifier partial_on_database?
    ///     | kw_remove kw_edge identifier << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_remove_edge_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_remove
                parse_kw_edge
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_remove
                parse_kw_edge
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_remove, _, partial_if_exists, ident, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveEdge {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_remove, _, ident, perm) = x.unwrap_seq4();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveEdge {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `REMOVE TYPE` expression.
    ///
    /// ```txt
    /// remove_type_exp =
    ///     | kw_remove kw_type partial_if_exists identifier partial_on_database?
    ///     | kw_remove kw_type identifier << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_remove_type_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_remove
                parse_kw_type
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_remove
                parse_kw_type
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_remove, _, partial_if_exists, ident, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveType {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_remove, _, ident, perm) = x.unwrap_seq4();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveType {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `REMOVE ENUM` expression.
    ///
    /// ```txt
    /// remove_enum_exp =
    ///     | kw_remove kw_enum partial_if_exists identifier partial_on_database?
    ///     | kw_remove kw_enum identifier << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_remove_enum_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_remove
                parse_kw_enum
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_remove
                parse_kw_enum
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_remove, _, partial_if_exists, ident, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveEnum {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_remove, _, ident, perm) = x.unwrap_seq4();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveEnum {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `REMOVE INDEX` expression.
    ///
    /// ```txt
    /// remove_index_exp =
    ///     | kw_remove kw_index partial_if_exists identifier << partial_on_table partial_on_database? >>
    ///     | kw_remove kw_index identifier << partial_if_exists? partial_on_table partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_remove_index_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_remove
                parse_kw_index
                parse_partial_if_exists
                parse_identifier
                (perm
                    parse_partial_on_table
                    (opt parse_partial_on_database)
                )
            )
            (seq
                parse_kw_remove
                parse_kw_index
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    parse_partial_on_table
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_remove, _, partial_if_exists, ident, perm) = x.unwrap_seq5();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
                    Some(end) => {
                        span_end = end;
                        true
                    }
                    None => false,
                };

                let (partial_on_table, opt_partial_on_database) = perm.unwrap_seq2();

                let table = extract_partial_on_table(*partial_on_table);
                span_end = usize::max(span_end, table.span.end);

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                Ast::new(
                    span_start..span_end,
                    RemoveIndex {
                        subject: Box::new(ident),
                        table: Box::new(table),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_remove, _, ident, perm) = x.unwrap_seq4();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, partial_on_table, opt_partial_on_database) =
                    perm.unwrap_seq3();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
                    Some(end) => {
                        span_end = end;
                        true
                    }
                    None => false,
                };

                let table = extract_partial_on_table(*partial_on_table);
                span_end = usize::max(span_end, table.span.end);

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                Ast::new(
                    span_start..span_end,
                    RemoveIndex {
                        subject: Box::new(ident),
                        table: Box::new(table),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `REMOVE MODULE` expression.
    ///
    /// ```txt
    /// remove_module_exp =
    ///     | kw_remove kw_module partial_if_exists identifier partial_on_database?
    ///     | kw_remove kw_module identifier << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_remove_module_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_remove
                parse_kw_module
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_remove
                parse_kw_module
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_remove, _, partial_if_exists, ident, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveModule {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_remove, _, ident, perm) = x.unwrap_seq4();

                let kw_remove = kw_remove.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveModule {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `REMOVE PARAM` expression.
    ///
    /// ```txt
    /// remove_param_exp =
    ///     | kw_remove kw_param partial_if_exists variable partial_on_database?
    ///     | kw_remove kw_param variable << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_remove_param_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_remove
                parse_kw_param
                parse_partial_if_exists
                parse_variable
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_remove
                parse_kw_param
                parse_variable
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_remove, _, partial_if_exists, variable, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_remove = kw_remove.unwrap_single();
                let variable = variable.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = variable.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveParam {
                        subject: Box::new(variable),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_remove, _, variable, perm) = x.unwrap_seq4();

                let kw_remove = kw_remove.unwrap_single();
                let variable = variable.unwrap_single();

                let span_start = kw_remove.span.start;
                let mut span_end = variable.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    RemoveParam {
                        subject: Box::new(variable),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `REMOVE *` expression.
    ///
    /// ```txt
    /// remove_exp =
    ///     | remove_namespace_exp
    ///     | remove_database_exp
    ///     | remove_table_exp
    ///     | remove_edge_exp
    ///     | remove_type_exp
    ///     | remove_enum_exp
    ///     | remove_index_exp
    ///     | remove_module_exp
    ///     | remove_param_exp
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_remove_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_remove_namespace_exp
            parse_remove_database_exp
            parse_remove_table_exp
            parse_remove_edge_exp
            parse_remove_type_exp
            parse_remove_enum_exp
            parse_remove_index_exp
            parse_remove_module_exp
            parse_remove_param_exp
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

    /// Parses `DESCRIBE NAMESPACE` expression.
    ///
    /// ```txt
    /// describe_namespace_exp =
    ///     | kw_describe (kw_namespace | kw_ns) << partial_if_exists? identifier >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_describe_namespace_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_describe
            (alt
                parse_kw_namespace
                parse_kw_ns
            )
            (perm
                (opt parse_partial_if_exists)
                parse_identifier
            )
        ));

        let ast = result.map(|x| {
            let (kw_describe, _, perm) = x.unwrap_seq3();
            let (opt_partial_if_exists, ident) = perm.unwrap_seq2();

            let kw_describe = kw_describe.unwrap_single();
            let ident = ident.unwrap_indexed().1.unwrap_single();

            let span_start = kw_describe.span.start;
            let mut span_end = ident.span.end;

            let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
                Some(end) => {
                    span_end = usize::max(span_end, end);
                    true
                }
                None => false,
            };

            Ast::new(
                span_start..span_end,
                DescribeNamespace {
                    subject: Box::new(ident),
                    if_exists,
                },
            )
        });

        Ok(ast)
    }

    /// Parses `DESCRIBE DATABASE` expression.
    ///
    /// ```txt
    /// describe_database_exp =
    ///     | kw_describe (kw_database | kw_db) partial_if_exists identifier partial_on_namespace?
    ///     | kw_describe (kw_database | kw_db) identifier << partial_if_exists? partial_on_namespace? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_describe_database_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_describe
                (alt
                    parse_kw_database
                    parse_kw_db
                )
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_namespace)
            )
            (seq
                parse_kw_describe
                (alt
                    parse_kw_database
                    parse_kw_db
                )
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_namespace)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_describe, _, partial_if_exists, ident, opt_partial_on_namespace) =
                    x.unwrap_seq5();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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
                    DescribeDatabase {
                        subject: Box::new(ident),
                        if_exists,
                        namespace,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_describe, _, ident, perm) = x.unwrap_seq4();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_namespace) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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
                    DescribeDatabase {
                        subject: Box::new(ident),
                        if_exists,
                        namespace,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `DESCRIBE TABLE` expression.
    ///
    /// ```txt
    /// describe_table_exp =
    ///     | kw_describe kw_table partial_if_exists identifier partial_on_database?
    ///     | kw_describe kw_table identifier << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_describe_table_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_describe
                parse_kw_table
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_describe
                parse_kw_table
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_describe, _, partial_if_exists, ident, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeTable {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_describe, _, ident, perm) = x.unwrap_seq4();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeTable {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `DESCRIBE EDGE` expression.
    ///
    /// ```txt
    /// describe_edge_exp =
    ///     | kw_describe kw_edge partial_if_exists identifier partial_on_database?
    ///     | kw_describe kw_edge identifier << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_describe_edge_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_describe
                parse_kw_edge
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_describe
                parse_kw_edge
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_describe, _, partial_if_exists, ident, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeEdge {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_describe, _, ident, perm) = x.unwrap_seq4();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeEdge {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `DESCRIBE TYPE` expression.
    ///
    /// ```txt
    /// describe_type_exp =
    ///     | kw_describe kw_type partial_if_exists identifier partial_on_database?
    ///     | kw_describe kw_type identifier << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_describe_type_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_describe
                parse_kw_type
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_describe
                parse_kw_type
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_describe, _, partial_if_exists, ident, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeType {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_describe, _, ident, perm) = x.unwrap_seq4();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeType {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `DESCRIBE ENUM` expression.
    ///
    /// ```txt
    /// describe_enum_exp =
    ///     | kw_describe kw_enum partial_if_exists identifier partial_on_database?
    ///     | kw_describe kw_enum identifier << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_describe_enum_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_describe
                parse_kw_enum
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_describe
                parse_kw_enum
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_describe, _, partial_if_exists, ident, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeEnum {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_describe, _, ident, perm) = x.unwrap_seq4();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeEnum {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `DESCRIBE INDEX` expression.
    ///
    /// ```txt
    /// describe_index_exp =
    ///     | kw_describe kw_index partial_if_exists identifier << partial_on_table partial_on_database? >>
    ///     | kw_describe kw_index identifier << partial_if_exists? partial_on_table partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_describe_index_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_describe
                parse_kw_index
                parse_partial_if_exists
                parse_identifier
                (perm
                    parse_partial_on_table
                    (opt parse_partial_on_database)
                )
            )
            (seq
                parse_kw_describe
                parse_kw_index
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    parse_partial_on_table
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_describe, _, partial_if_exists, ident, perm) = x.unwrap_seq5();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
                    Some(end) => {
                        span_end = end;
                        true
                    }
                    None => false,
                };

                let (partial_on_table, opt_partial_on_database) = perm.unwrap_seq2();

                let table = extract_partial_on_table(*partial_on_table);
                span_end = usize::max(span_end, table.span.end);

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                Ast::new(
                    span_start..span_end,
                    DescribeIndex {
                        subject: Box::new(ident),
                        table: Box::new(table),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_describe, _, ident, perm) = x.unwrap_seq4();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, partial_on_table, opt_partial_on_database) =
                    perm.unwrap_seq3();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
                    Some(end) => {
                        span_end = end;
                        true
                    }
                    None => false,
                };

                let table = extract_partial_on_table(*partial_on_table);
                span_end = usize::max(span_end, table.span.end);

                let database = match extract_opt_partial_on_database(*opt_partial_on_database) {
                    Some(ident) => {
                        span_end = usize::max(span_end, ident.span.end);
                        Some(Box::new(ident))
                    }
                    None => None,
                };

                Ast::new(
                    span_start..span_end,
                    DescribeIndex {
                        subject: Box::new(ident),
                        table: Box::new(table),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `DESCRIBE MODULE` expression.
    ///
    /// ```txt
    /// describe_module_exp =
    ///     | kw_describe kw_module partial_if_exists identifier partial_on_database?
    ///     | kw_describe kw_module identifier << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_describe_module_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_describe
                parse_kw_module
                parse_partial_if_exists
                parse_identifier
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_describe
                parse_kw_module
                parse_identifier
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_describe, _, partial_if_exists, ident, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeModule {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_describe, _, ident, perm) = x.unwrap_seq4();

                let kw_describe = kw_describe.unwrap_single();
                let ident = ident.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = ident.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeModule {
                        subject: Box::new(ident),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `DESCRIBE PARAM` expression.
    ///
    /// ```txt
    /// describe_param_exp =
    ///     | kw_describe kw_param partial_if_exists variable partial_on_database?
    ///     | kw_describe kw_param variable << partial_if_exists? partial_on_database? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_describe_param_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_kw_describe
                parse_kw_param
                parse_partial_if_exists
                parse_variable
                (opt parse_partial_on_database)
            )
            (seq
                parse_kw_describe
                parse_kw_param
                parse_variable
                (perm
                    (opt parse_partial_if_exists)
                    (opt parse_partial_on_database)
                )
            )
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (kw_describe, _, partial_if_exists, variable, opt_partial_on_database) =
                    x.unwrap_seq5();

                let kw_describe = kw_describe.unwrap_single();
                let variable = variable.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = variable.span.end;

                let if_exists = match extract_opt_partial_if_exists(*partial_if_exists) {
                    Some(end) => {
                        span_end = end;
                        true
                    }
                    None => false,
                };

                let database: Option<Box<Ast>> =
                    match extract_opt_partial_on_database(*opt_partial_on_database) {
                        Some(ident) => {
                            span_end = usize::max(span_end, ident.span.end);
                            Some(Box::new(ident))
                        }
                        None => None,
                    };

                Ast::new(
                    span_start..span_end,
                    DescribeParam {
                        subject: Box::new(variable),
                        if_exists,
                        database,
                    },
                )
            }
            Choice::B(x) => {
                let (kw_describe, _, variable, perm) = x.unwrap_seq4();

                let kw_describe = kw_describe.unwrap_single();
                let variable = variable.unwrap_single();

                let span_start = kw_describe.span.start;
                let mut span_end = variable.span.end;

                let (opt_partial_if_exists, opt_partial_on_database) = perm.unwrap_seq2();

                let if_exists = match extract_opt_partial_if_exists(*opt_partial_if_exists) {
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

                Ast::new(
                    span_start..span_end,
                    DescribeParam {
                        subject: Box::new(variable),
                        if_exists,
                        database,
                    },
                )
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses `DESCRIBE *` expression.
    ///
    /// ```txt
    /// describe_exp =
    ///     | describe_namespace_exp
    ///     | describe_database_exp
    ///     | describe_table_exp
    ///     | describe_edge_exp
    ///     | describe_type_exp
    ///     | describe_enum_exp
    ///     | describe_index_exp
    ///     | describe_module_exp
    ///     | describe_param_exp
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_describe_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_describe_namespace_exp
            parse_describe_database_exp
            parse_describe_table_exp
            parse_describe_edge_exp
            parse_describe_type_exp
            parse_describe_enum_exp
            parse_describe_index_exp
            parse_describe_module_exp
            parse_describe_param_exp
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

    /// Parses `BEGIN TRANSACTION` expression.
    ///
    /// ```txt
    /// begin_exp =
    ///     | kw_begin kw_transaction?
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_begin_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_begin
            (opt parse_kw_transaction)
        ));

        let ast = result.map(|x| {
            let (kw_begin, opt_kw_transaction) = x.unwrap_seq2();
            let kw_begin = kw_begin.unwrap_single();

            let span_start = kw_begin.span.start;
            let mut span_end = kw_begin.span.end;

            if let Combinator::Single(kw_transaction) = *opt_kw_transaction {
                span_end = kw_transaction.span.end;
            }

            Ast::new(span_start..span_end, BeginTransaction)
        });

        Ok(ast)
    }

    /// Parses `COMMIT TRANSACTION` expression.
    ///
    /// ```txt
    /// commit_exp =
    ///     | kw_commit kw_transaction?
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_commit_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_commit
            (opt parse_kw_transaction)
        ));

        let ast = result.map(|x| {
            let (kw_commit, opt_kw_transaction) = x.unwrap_seq2();
            let kw_commit = kw_commit.unwrap_single();

            let span_start = kw_commit.span.start;
            let mut span_end = kw_commit.span.end;

            if let Combinator::Single(kw_transaction) = *opt_kw_transaction {
                span_end = kw_transaction.span.end;
            }

            Ast::new(span_start..span_end, CommitTransaction)
        });

        Ok(ast)
    }

    /// Parses `CANCEL TRANSACTION` expression.
    ///
    /// ```txt
    /// cancel_exp =
    ///     | kw_cancel kw_transaction?
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_cancel_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_cancel
            (opt parse_kw_transaction)
        ));

        let ast = result.map(|x| {
            let (kw_cancel, opt_kw_transaction) = x.unwrap_seq2();
            let kw_cancel = kw_cancel.unwrap_single();

            let span_start = kw_cancel.span.start;
            let mut span_end = kw_cancel.span.end;

            if let Combinator::Single(kw_transaction) = *opt_kw_transaction {
                span_end = kw_transaction.span.end;
            }

            Ast::new(span_start..span_end, CancelTransaction)
        });

        Ok(ast)
    }

    /// Parses `FOR` expression.
    ///
    /// ```txt
    /// for_exp =
    ///     | kw_for variable op_in range_op kw_do program kw_end
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_for_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_for
            parse_variable
            parse_op_in
            parse_range_op
            parse_kw_do
            parse_program
            parse_kw_end
        ));

        let ast = result.map(|x| {
            let (kw_for, variable, _, range_op, _, program, kw_end) = x.unwrap_seq7();
            let kw_for = kw_for.unwrap_single();
            let variable = variable.unwrap_single();
            let range_op = range_op.unwrap_single();
            let program = program.unwrap_single();
            let kw_end = kw_end.unwrap_single();

            Ast::new(
                kw_for.span.start..kw_end.span.end,
                For {
                    variable: Box::new(variable),
                    iterator: Box::new(range_op),
                    body: Box::new(program),
                },
            )
        });

        Ok(ast)
    }

    /// Parses `WHILE` expression.
    ///
    /// ```txt
    /// while_exp =
    ///     | kw_while range_op kw_do program kw_end
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_while_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_while
            parse_range_op
            parse_kw_do
            parse_program
            parse_kw_end
        ));

        let ast = result.map(|x| {
            let (kw_while, range_op, _, program, kw_end) = x.unwrap_seq5();
            let kw_while = kw_while.unwrap_single();
            let range_op = range_op.unwrap_single();
            let program = program.unwrap_single();
            let kw_end = kw_end.unwrap_single();

            Ast::new(
                kw_while.span.start..kw_end.span.end,
                While {
                    condition: Box::new(range_op),
                    body: Box::new(program),
                },
            )
        });

        Ok(ast)
    }

    /// Parses partial `else_if_part` syntax.
    ///
    /// ```txt
    /// partial_else_if_part =
    ///     | kw_else kw_if range_op kw_then program
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_else_if_part(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_else
            parse_kw_if
            parse_range_op
            parse_kw_then
            parse_program
        ));

        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));

        Ok(ast)
    }

    /// Parses `IF` expression.
    ///
    /// ```txt
    /// if_else_exp  =
    ///     | kw_if range_op kw_then program partial_else_if_part* (kw_else program)? kw_end
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_if_else_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_if
            parse_range_op
            parse_kw_then
            parse_program
            (many_0 parse_partial_else_if_part)
            (opt (seq
                parse_kw_else
                parse_program
            ))
            parse_kw_end
        ));

        let ast = result.map(|x| {
            let (kw_if, range_op, _, program, partial_else_if_parts, opt_kw_else, kw_end) =
                x.unwrap_seq7();

            let range_op = range_op.unwrap_single();
            let program = program.unwrap_single();

            let mut else_ifs = vec![];
            for partial_else_if_part in partial_else_if_parts.unwrap_many() {
                let (_, _, range_op, _, program) = partial_else_if_part
                    .unwrap_single()
                    .unwrap_temp()
                    .unwrap_seq5();

                else_ifs.push(ElseIfPart {
                    condition: Box::new(range_op.unwrap_single()),
                    body: Box::new(program.unwrap_single()),
                });
            }

            let r#else = match *opt_kw_else {
                Combinator::Void => None,
                Combinator::Seq2(_, program) => Some(Box::new(program.unwrap_single())),
                _ => unreachable!(),
            };

            Ast::new(
                kw_if.unwrap_single().span.start..kw_end.unwrap_single().span.end,
                If {
                    condition: Box::new(range_op),
                    then: Box::new(program),
                    else_ifs,
                    r#else,
                },
            )
        });

        Ok(ast)
    }

    /// Parses partial `type_sig` syntax.
    ///
    /// ```txt
    /// partial_type_sig =
    ///     | "[" partial_type_sig integer_lit "]" "?"*
    ///     | "[" partial_type_sig "]" "?"*
    ///     | "(" partial_type_sig ("," partial_type_sig)* ","? ")" "?"*
    ///     | identifier_scope_op "[" partial_type_sig ("," partial_type_sig)* ","? "]" "?"*
    ///     | identifier_scope_op "?"*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_partial_type_sig(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                (arg parse_tok OpOpenSquareBracket)
                parse_partial_type_sig
                parse_integer_lit
                (arg parse_tok OpCloseSquareBracket)
                (many_0 (arg parse_tok OpOptional))
            )
            (seq
                (arg parse_tok OpOpenSquareBracket)
                parse_partial_type_sig
                (arg parse_tok OpCloseSquareBracket)
                (many_0 (arg parse_tok OpOptional))
            )
            (seq
                (arg parse_tok OpOpenParen)
                parse_partial_type_sig
                (many_0 (seq
                    (arg parse_tok OpComma)
                    parse_partial_type_sig
                ))
                (opt (arg parse_tok OpComma))
                (arg parse_tok OpCloseParen)
                (many_0 (arg parse_tok OpOptional))
            )
            (seq
                parse_identifier_scope_op
                (arg parse_tok OpOpenSquareBracket)
                parse_partial_type_sig
                (many_0 (seq
                    (arg parse_tok OpComma)
                    parse_partial_type_sig
                ))
                (opt (arg parse_tok OpComma))
                (arg parse_tok OpCloseSquareBracket)
                (many_0 (arg parse_tok OpOptional))
            )
            (seq
                parse_identifier_scope_op
                (many_0 (arg parse_tok OpOptional))
            )
        ));

        let ast = result.map(|x| Ast::new(0..0, Temp(Some(Box::new(x)))));

        Ok(ast)
    }

    /// Parses `LET` expression.
    ///
    /// ```txt
    /// let_exp =
    ///     | kw_let variable (kw_type type_sig)? op_is_lexer exp
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_let_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_let
            parse_variable
            (opt (seq
                parse_kw_type
                parse_partial_type_sig
            ))
            (arg parse_tok OpIsLexer)
            parse_exp
        ));

        let ast = result.map(|x| {
            let (kw_let, variable, opt_type_sig, _, exp) = x.unwrap_seq5();
            let kw_let = kw_let.unwrap_single();
            let variable = variable.unwrap_single();
            let exp = exp.unwrap_single();

            let r#type = match *opt_type_sig {
                Combinator::Void => None,
                Combinator::Seq2(_, partial_type_sig) => {
                    Some(Box::new(extract_partial_type_sig(*partial_type_sig).0))
                }
                _ => unreachable!(),
            };

            Ast::new(
                kw_let.span.start..exp.span.end,
                Let {
                    name: Box::new(variable),
                    value: Box::new(exp),
                    r#type,
                },
            )
        });

        Ok(ast)
    }

    /// Parses `SET` expression.
    ///
    /// ```txt
    /// set_exp =
    ///     | kw_set variable partial_op_update_assign exp
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_set_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_set
            parse_variable
            parse_partial_op_update_assign
            parse_exp
        ));

        let ast = result.map(|x| {
            let (kw_set, variable, partial_op_update_assign, exp) = x.unwrap_seq4();
            let kw_set = kw_set.unwrap_single();
            let variable = variable.unwrap_single();
            let exp = exp.unwrap_single();

            let op = match partial_op_update_assign
                .unwrap_single()
                .unwrap_temp()
                .unwrap_choice()
            {
                Choice::A(_) => Direct,
                Choice::B(_) => Plus,
                Choice::C(_) => Minus,
                Choice::D(_) => Mul,
                Choice::E(_) => Div,
                Choice::F(_) => Mod,
                Choice::G(_) => Pow,
                Choice::H(_) => BitAnd,
                Choice::I(_) => BitOr,
                Choice::J(rest) => match rest.unwrap_choice() {
                    Choice::A(_) => BitXor,
                    Choice::B(_) => BitNot,
                    Choice::C(_) => Shl,
                    Choice::D(_) => Shr,
                    _ => unreachable!(),
                },
            };

            Ast::new(
                kw_set.span.start..exp.span.end,
                Set {
                    variable: Box::new(variable),
                    op,
                    value: Box::new(exp),
                },
            )
        });

        Ok(ast)
    }

    /// Parses any expression.
    ///
    /// ```txt
    /// exp =
    ///     | relate_exp
    ///     | create_exp
    ///     | delete_exp
    ///     | update_exp
    ///     | select_exp
    ///     | remove_exp
    ///     | describe_exp
    ///     | begin_exp
    ///     | commit_exp
    ///     | cancel_exp
    ///     | for_exp
    ///     | if_else_exp
    ///     | let_exp
    ///     | set_exp
    ///     | op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_relate_exp
            parse_create_exp
            parse_delete_exp
            parse_update_exp
            parse_select_exp
            parse_remove_exp
            parse_describe_exp
            parse_begin_exp
            parse_commit_exp
            (alt
                parse_cancel_exp
                parse_for_exp
                parse_if_else_exp
                parse_let_exp
                parse_set_exp
                parse_op
            )
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
            Choice::J(x) => match x.unwrap_choice() {
                Choice::A(x) => x.unwrap_single(),
                Choice::B(x) => x.unwrap_single(),
                Choice::C(x) => x.unwrap_single(),
                Choice::D(x) => x.unwrap_single(),
                Choice::E(x) => x.unwrap_single(),
                Choice::F(x) => x.unwrap_single(),
                _ => unreachable!(),
            },
        });

        Ok(ast)
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn extract_partial_select_from(comb: Combinator<Ast<'_>>) -> (Vec<Ast<'_>>, usize) {
    match comb.unwrap_single().unwrap_temp().unwrap_choice() {
        Choice::A(x) => {
            let (_, op0, ops) = x.unwrap_seq3();
            let op0 = op0.unwrap_single();
            let mut span_end = op0.span.end;

            let mut op_asts = vec![op0];
            for op in ops.unwrap_many() {
                let op = op.unwrap_seq2().1.unwrap_single();
                span_end = op.span.end;
                op_asts.push(op);
            }

            (op_asts, span_end)
        }
        Choice::B(x) => {
            let wildcard = x.unwrap_seq2().1.unwrap_single();
            let span_end = wildcard.span.end;
            (vec![Ast::new(wildcard.span, Wildcard)], span_end)
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_partial_select_fields(
    comb: Combinator<Ast<'_>>,
) -> (Vec<SelectColumn<'_>>, Vec<Ast<'_>>) {
    let (partial_select_fields0, fields, opt_omit) =
        comb.unwrap_single().unwrap_temp().unwrap_seq3();

    let select_field0 = extract_partial_select_field(*partial_select_fields0);

    let mut field_asts = vec![select_field0];
    for field in fields.unwrap_many() {
        let (_, partial_select_field) = field.unwrap_seq2();
        let select_field = extract_partial_select_field(*partial_select_field);
        field_asts.push(select_field);
    }

    let omit_asts = match *opt_omit {
        Combinator::Void => vec![],
        Combinator::Seq2(_, partial_select_omit) => {
            extract_partial_select_omit(*partial_select_omit)
        }
        _ => unreachable!(),
    };

    (field_asts, omit_asts)
}

pub(crate) fn extract_partial_select_omit(comb: Combinator<Ast<'_>>) -> Vec<Ast<'_>> {
    let (_, op, ops) = comb.unwrap_single().unwrap_temp().unwrap_seq3();
    let op = op.unwrap_single();

    let mut op_asts = vec![op];
    for op in ops.unwrap_many() {
        let (_, op) = op.unwrap_seq2();
        op_asts.push(op.unwrap_single());
    }

    op_asts
}

pub(crate) fn extract_partial_select_field(comb: Combinator<Ast<'_>>) -> SelectColumn<'_> {
    match comb.unwrap_single().unwrap_temp().unwrap_choice() {
        Choice::A(partial_select_field_fold) => {
            let (fold, range_op, opt_partial_as) = partial_select_field_fold
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();

            let fold = fold.unwrap_single();
            let range_op = range_op.unwrap_single();
            match parse_operations::extract_opt_partial_as(*opt_partial_as) {
                Some((alias, span_end)) => SelectColumn::Fold {
                    subject: Box::new(Ast::new(fold.span.start..span_end, range_op.kind)),
                    alias: Some(alias),
                },
                None => SelectColumn::Fold {
                    subject: Box::new(Ast::new(fold.span.start..range_op.span.end, range_op.kind)),
                    alias: None,
                },
            }
        }
        Choice::B(op) => SelectColumn::Column(Box::new(op.unwrap_single())),
        Choice::C(op_star) => {
            SelectColumn::Column(Box::new(Ast::new(op_star.unwrap_single().span, Wildcard)))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_select_where_guard(
    comb: Combinator<Ast<'_>>,
) -> Option<(usize, SelectTransform, usize)> {
    extract_opt_partial_where_guard(comb)
        .map(|(i, op, span_end)| (i, SelectTransform::WhereGuard(op), span_end))
}

pub(crate) fn extract_opt_partial_where_guard(
    comb: Combinator<Ast<'_>>,
) -> Option<(usize, Box<Ast<'_>>, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(i, partial_where_guard) => {
            let (_, alt) = partial_where_guard
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq2();

            match alt.unwrap_choice() {
                Choice::A(x) => {
                    let op = x.unwrap_single();
                    let span_end = op.span.end;
                    Some((i, Box::new(op), span_end))
                }
                Choice::B(x) => {
                    let op_star = x.unwrap_single();
                    let span_end = op_star.span.end;
                    Some((i, Box::new(Ast::new(op_star.span, Wildcard)), span_end))
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_select_with_indices(
    comb: Combinator<Ast<'_>>,
) -> Option<(usize, SelectTransform, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(i, partial_select_with_indices) => {
            match partial_select_with_indices
                .unwrap_single()
                .unwrap_temp()
                .unwrap_choice()
            {
                Choice::A(x) => {
                    let (_, _, ident, idents) = x.unwrap_seq4();
                    let ident = ident.unwrap_single();
                    let mut span_end = ident.span.end;

                    let mut indices = vec![ident];
                    for ident in idents.unwrap_many() {
                        let ident = ident.unwrap_seq2().1.unwrap_single();
                        span_end = ident.span.end;
                        indices.push(ident);
                    }

                    Some((i, SelectTransform::WithIndexes(indices), span_end))
                }
                Choice::B(x) => {
                    let (_, _, index) = x.unwrap_seq3();
                    let index = index.unwrap_single();

                    Some((i, SelectTransform::WithNoIndex, index.span.end))
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_select_group_by(
    comb: Combinator<Ast<'_>>,
) -> Option<(usize, SelectTransform, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(i, partial_select_group_by) => {
            let (_, _, range_op, range_ops) = partial_select_group_by
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq4();

            let range_op = range_op.unwrap_single();
            let mut span_end = range_op.span.end;

            let mut range_op_asts = vec![range_op];
            for range_op in range_ops.unwrap_many() {
                let range_op = range_op.unwrap_seq2().1.unwrap_single();
                span_end = range_op.span.end;
                range_op_asts.push(range_op);
            }

            Some((i, SelectTransform::GroupBy(range_op_asts), span_end))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_select_order_by(
    comb: Combinator<Ast<'_>>,
) -> Option<(usize, SelectTransform, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(i, partial_select_order_by) => {
            let (_, _, range_op, range_ops, opt_alt) = partial_select_order_by
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq5();

            let range_op = range_op.unwrap_single();
            let mut span_end = range_op.span.end;

            let mut range_op_asts = vec![range_op];
            for range_op in range_ops.unwrap_many() {
                let range_op = range_op.unwrap_seq2().1.unwrap_single();
                span_end = range_op.span.end;
                range_op_asts.push(range_op);
            }

            let direction = match *opt_alt {
                Combinator::Void => Direction::Ascending,
                Combinator::Choice(direction) => match direction {
                    Choice::A(_) => Direction::Ascending,
                    Choice::B(_) => Direction::Descending,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            Some((
                i,
                SelectTransform::OrderBy {
                    fields: range_op_asts,
                    direction,
                },
                span_end,
            ))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_select_limit_to(
    comb: Combinator<Ast<'_>>,
) -> Option<(usize, SelectTransform, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(i, partial_select_limit_to) => {
            let (_, _, range_op) = partial_select_limit_to
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();
            let range_op = range_op.unwrap_single();
            let span_end = range_op.span.end;

            Some((i, SelectTransform::LimitTo(Box::new(range_op)), span_end))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_select_start_at(
    comb: Combinator<Ast<'_>>,
) -> Option<(usize, SelectTransform, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(i, partial_select_start_at) => {
            let (_, _, range_op) = partial_select_start_at
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();
            let range_op = range_op.unwrap_single();
            let span_end = range_op.span.end;

            Some((i, SelectTransform::StartAt(Box::new(range_op)), span_end))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_if_exists(comb: Combinator<Ast<'_>>) -> Option<usize> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_if_exists) => {
            let (_, kw_exists) = partial_if_exists
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq2();

            match kw_exists.unwrap_choice() {
                Choice::A(kw_exists) => Some(kw_exists.unwrap_single().span.end),
                Choice::B(kw_exist) => Some(kw_exist.unwrap_single().span.end),
                _ => unreachable!(),
            }
        }
        Combinator::Single(partial_if_exists) => {
            let (_, kw_exists) = partial_if_exists.unwrap_temp().unwrap_seq2();
            match kw_exists.unwrap_choice() {
                Choice::A(kw_exists) => Some(kw_exists.unwrap_single().span.end),
                Choice::B(kw_exist) => Some(kw_exist.unwrap_single().span.end),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_on_namespace(comb: Combinator<Ast<'_>>) -> Option<Ast<'_>> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_on_namespace) => {
            let (_, _, ident) = partial_on_namespace
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();

            Some(ident.unwrap_single())
        }
        Combinator::Single(partial_on_namespace) => {
            let (_, _, ident) = partial_on_namespace.unwrap_temp().unwrap_seq3();
            Some(ident.unwrap_single())
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_on_database(comb: Combinator<Ast<'_>>) -> Option<Ast<'_>> {
    match comb {
        Combinator::Void => None,
        Combinator::Indexed(_, partial_on_database) => {
            let (_, _, ident) = partial_on_database
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();

            Some(ident.unwrap_single())
        }
        Combinator::Single(partial_on_database) => {
            let (_, _, ident) = partial_on_database.unwrap_temp().unwrap_seq3();
            Some(ident.unwrap_single())
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_partial_on_table(comb: Combinator<Ast<'_>>) -> Ast<'_> {
    match comb {
        Combinator::Indexed(_, partial_on_database) => {
            let (_, _, ident) = partial_on_database
                .unwrap_single()
                .unwrap_temp()
                .unwrap_seq3();

            ident.unwrap_single()
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_partial_type_sig(comb: Combinator<Ast<'_>>) -> (TypeSig<'_>, usize) {
    match comb.unwrap_single().unwrap_temp().unwrap_choice() {
        Choice::A(type_sig) => {
            let (_, partial_type_sig, integer_lit, close, options) = type_sig.unwrap_seq5();
            let (partial_type_sig, _) = extract_partial_type_sig(*partial_type_sig);
            let integer_lit = integer_lit.unwrap_single();
            let close = close.unwrap_single();

            let mut sig = TypeSig::Array {
                r#type: Box::new(partial_type_sig),
                length: Box::new(integer_lit),
            };

            let mut span_end = close.span.end;
            for option in options.unwrap_many() {
                sig = TypeSig::Option(Box::new(sig));
                span_end = option.unwrap_single().span.end;
            }

            (sig, span_end)
        }
        Choice::B(type_sig) => {
            let (_, partial_type_sig, close, options) = type_sig.unwrap_seq4();
            let (partial_type_sig, _) = extract_partial_type_sig(*partial_type_sig);
            let close = close.unwrap_single();

            let mut sig = TypeSig::List(Box::new(partial_type_sig));

            let mut span_end = close.span.end;
            for option in options.unwrap_many() {
                sig = TypeSig::Option(Box::new(sig));
                span_end = option.unwrap_single().span.end;
            }

            (sig, span_end)
        }
        Choice::C(type_sig) => {
            let (_, partial_type_sig, type_sigs, _, close, options) = type_sig.unwrap_seq6();
            let close = close.unwrap_single();
            let (partial_type_sig, _) = extract_partial_type_sig(*partial_type_sig);
            let mut sigs = vec![partial_type_sig];

            for type_sig in type_sigs.unwrap_many() {
                let (_, partial_type_sig) = type_sig.unwrap_seq2();
                let (partial_type_sig, _) = extract_partial_type_sig(*partial_type_sig);
                sigs.push(partial_type_sig);
            }

            let mut span_end = close.span.end;
            let mut sig = TypeSig::Tuple(sigs);
            for option in options.unwrap_many() {
                sig = TypeSig::Option(Box::new(sig));
                span_end = option.unwrap_single().span.end;
            }

            (sig, span_end)
        }
        Choice::D(type_sig) => {
            let (ident, _, partial_type_sig, type_sigs, _, close, options) = type_sig.unwrap_seq7();

            let ident = ident.unwrap_single();
            let close = close.unwrap_single();
            let (partial_type_sig, _) = extract_partial_type_sig(*partial_type_sig);
            let mut sigs = vec![partial_type_sig];

            for type_sig in type_sigs.unwrap_many() {
                let (_, partial_type_sig) = type_sig.unwrap_seq2();
                let (partial_type_sig, _) = extract_partial_type_sig(*partial_type_sig);
                sigs.push(partial_type_sig);
            }

            let mut sig = TypeSig::Generic {
                name: Box::new(ident),
                parameters: sigs,
            };

            let mut span_end = close.span.end;
            for option in options.unwrap_many() {
                sig = TypeSig::Option(Box::new(sig));
                span_end = option.unwrap_single().span.end;
            }

            (sig, span_end)
        }
        Choice::E(type_sig) => {
            let (ident, options) = type_sig.unwrap_seq2();

            let ident = ident.unwrap_single();

            let mut span_end = ident.span.end;
            let mut sig = TypeSig::Basic(Box::new(ident));
            for option in options.unwrap_many() {
                sig = TypeSig::Option(Box::new(sig));
                span_end = option.unwrap_single().span.end;
            }

            (sig, span_end)
        }
        _ => unreachable!(),
    }
}

//--------------------------------------------------------------------------------------------------
// Macros
//--------------------------------------------------------------------------------------------------

macro_rules! ast_as {
    ($ast:expr, $name:ident ( $($param:ident),* )) => {
        if let Ast {
            span,
            kind: $name ($($param),*),
        } = $ast
        {
            (($($param),*), span.end)
        } else {
            unreachable!();
        }
    };
    ($ast:expr, $name:ident { $($param:ident),* }) => {
        if let Ast {
            span,
            kind: $name { $($param),* , .. },
        } = $ast
        {
            (($($param),*), span.end)
        } else {
            unreachable!();
        }
    };
}

pub(crate) use ast_as;
