use zeroql_macros::{backtrack, memoize};

use crate::{
    ast::{Ast, AstKind::*, Direction, SelectColumn, SelectTransform, UpdateAssign::*},
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
    ///     | op_assign_null_coalesce
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
                (arg parse_tok OpAssignNullCoalesce)
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
            (perm_opt
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

            let (partial_where_guard, partial_set) = perm.unwrap_seq2();

            let (partial_where_guard, partial_where_guard_span_end) = match *partial_where_guard {
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

            let (column_ops, partial_set_span_end) = match partial_set.unwrap_choice() {
                Choice::A(partial_set_object) => {
                    let object = partial_set_object
                        .unwrap_single()
                        .unwrap_temp()
                        .unwrap_seq2()
                        .1;

                    let (tuples, span_end) = ast_as!(object.unwrap_single(), ObjectLiteral(tuples));

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
                            Choice::E(_) => (column, NullCoalesce, value),
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
                                Choice::E(_) => (column, NullCoalesce, value),
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
            (perm_opt
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

            let (fields, omit) = self::extract_partial_select_fields(*partial_select_fields);
            let (from, mut span_end) = self::extract_partial_select_from(*partial_select_from);
            let (
                opt_partial_where_guard,
                opt_partial_select_with_indices,
                opt_partial_select_group_by,
                opt_partial_select_order_by,
                opt_partial_select_start_at,
                opt_partial_select_limit_to,
            ) = perm.unwrap_seq6();

            let mut transforms = vec![];
            if let Some((transform, end)) =
                self::extract_opt_partial_select_where_guard(*opt_partial_where_guard)
            {
                transforms.push(transform);
                span_end = end;
            }

            if let Some((transform, end)) =
                self::extract_opt_partial_select_with_indices(*opt_partial_select_with_indices)
            {
                transforms.push(transform);
                span_end = end;
            }

            if let Some((transform, end)) =
                self::extract_opt_partial_select_group_by(*opt_partial_select_group_by)
            {
                transforms.push(transform);
                span_end = end;
            }

            if let Some((transform, end)) =
                self::extract_opt_partial_select_order_by(*opt_partial_select_order_by)
            {
                transforms.push(transform);
                span_end = end;
            }

            if let Some((transform, end)) =
                self::extract_opt_partial_select_start_at(*opt_partial_select_start_at)
            {
                transforms.push(transform);
                span_end = end;
            }

            if let Some((transform, end)) =
                self::extract_opt_partial_select_limit_to(*opt_partial_select_limit_to)
            {
                transforms.push(transform);
                span_end = end;
            }

            Ast::new(
                span_start..span_end,
                Select {
                    fields,
                    from,
                    omit,
                    transforms,
                },
            )
        });

        Ok(ast)
    }

    // partial_if_not_exists =
    //     | kw_if kw_not kw_exists

    // partial_if_exists =
    //     | kw_if kw_exists

    // partial_on_namespace =
    //     | kw_on (kw_namespace | kw_ns)? identifier

    // partial_on_database =
    //     | kw_on (kw_database | kw_db)? identifier

    // partial_on_table =
    //     | kw_on kw_table identifier

    // remove_namespace_exp =
    //     | kw_remove kw_namespace partial_if_exists? identifier
    //     | kw_remove kw_namespace identifier partial_if_exists

    // remove_database_exp =
    //     | kw_remove kw_database partial_if_exists? identifier partial_on_namespace?
    //     | kw_remove kw_database identifier << partial_if_exists? partial_on_namespace? >>

    // remove_table_exp =
    //     | kw_remove kw_table partial_if_exists? identifier partial_on_database?
    //     | kw_remove kw_table identifier << partial_if_exists? partial_on_database? >>

    // remove_edge_exp =
    //     | kw_remove kw_edge partial_if_exists? identifier partial_on_database?
    //     | kw_remove kw_edge identifier << partial_if_exists? partial_on_database? >>

    // remove_type_exp =
    //     | kw_remove kw_type partial_if_exists? identifier partial_on_database?
    //     | kw_remove kw_type identifier << partial_if_exists? partial_on_database? >>

    // remove_enum_exp =
    //     | kw_remove kw_enum partial_if_exists? identifier partial_on_database?
    //     | kw_remove kw_enum identifier << partial_if_exists? partial_on_database? >>

    // remove_index_exp =
    //     | kw_remove kw_index partial_if_exists? identifier << partial_on_table? partial_on_database? >>
    //     | kw_remove kw_index identifier << partial_if_exists? partial_on_table? partial_on_database? >>

    // remove_module_exp =
    //     | kw_remove kw_module partial_if_exists? identifier partial_on_database?
    //     | kw_remove kw_module identifier << partial_if_exists? partial_on_database? >>

    // remove_param_exp =
    //     | kw_remove kw_param partial_if_exists? variable partial_on_database?
    //     | kw_remove kw_param variable << partial_if_exists? partial_on_database? >>

    // remove_exp =
    //     | remove_namespace_exp
    //     | remove_database_exp
    //     | remove_table_exp
    //     | remove_edge_exp
    //     | remove_type_exp
    //     | remove_enum_exp
    //     | remove_index_exp
    //     | remove_module_exp
    //     | remove_param_exp

    // describe_namespace_exp =
    //     | kw_describe kw_namespace partial_if_exists? identifier
    //     | kw_describe kw_namespace identifier partial_if_exists?

    // describe_database_exp =
    //     | kw_describe kw_database partial_if_exists? identifier partial_on_namespace?
    //     | kw_describe kw_database identifier << partial_if_exists? partial_on_namespace? >>

    // describe_table_exp =
    //     | kw_describe kw_table partial_if_exists? identifier partial_on_database?
    //     | kw_describe kw_table identifier << partial_if_exists? partial_on_database? >>

    // describe_edge_exp =
    //     | kw_describe kw_edge partial_if_exists? identifier partial_on_database?
    //     | kw_describe kw_edge identifier << partial_if_exists? partial_on_database? >>

    // describe_type_exp =
    //     | kw_describe kw_type partial_if_exists? identifier partial_on_database?
    //     | kw_describe kw_type identifier << partial_if_exists? partial_on_database? >>

    // describe_enum_exp =
    //     | kw_describe kw_enum partial_if_exists? identifier partial_on_database?
    //     | kw_describe kw_enum identifier << partial_if_exists? partial_on_database? >>

    // describe_index_exp =
    //     | kw_describe kw_index partial_if_exists? identifier partial_on_table? partial_on_database?
    //     | kw_describe kw_index identifier << partial_if_exists? partial_on_table? partial_on_database? >>

    // describe_module_exp =
    //     | kw_describe kw_module partial_if_exists? identifier partial_on_database?
    //     | kw_describe kw_module identifier << partial_if_exists? partial_on_database? >>

    // describe_param_exp =
    //     | kw_describe kw_param partial_if_exists? variable partial_on_database?
    //     | kw_describe kw_param variable << partial_if_exists? partial_on_database? >>

    // describe_exp =
    //     | describe_namespace_exp
    //     | describe_database_exp
    //     | describe_table_exp
    //     | describe_edge_exp
    //     | describe_type_exp
    //     | describe_enum_exp
    //     | describe_index_exp
    //     | describe_module_exp
    //     | describe_param_exp

    // begin_exp =
    //     | kw_begin kw_transaction?

    // commit_exp =
    //     | kw_commit kw_transaction?

    // cancel_exp =
    //     | kw_cancel kw_transaction?

    // for_exp =
    //     | kw_for variable op_in op kw_do program kw_end

    // partial_else_part =
    //     | kw_else program

    // partial_else_if_part =
    //     | kw_else kw_if op kw_then program

    // if_else_exp =
    //     | kw_if op kw_then program partial_else_if_part* partial_else_part? kw_end

    // let_exp =
    //     | kw_let variable (kw_type type_sig)? op_is_lexer exp

    // set_exp =
    //     | kw_set variable partial_op_update_assign exp

    // exp =
    //     | relate_exp
    //     | create_exp
    //     | delete_exp
    //     | update_exp
    //     | select_exp
    //     | remove_exp
    //     | describe_exp
    //     | begin_exp
    //     | commit_exp
    //     | cancel_exp
    //     | for_exp
    //     | if_else_exp
    //     | let_exp
    //     | set_exp
    //     | op

    /// TODO
    #[memoize]
    #[backtrack]
    pub fn parse_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_identifier() // TODO
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

    let select_field0 = self::extract_partial_select_field(*partial_select_fields0);

    let mut field_asts = vec![select_field0];
    for field in fields.unwrap_many() {
        let (_, partial_select_field) = field.unwrap_seq2();
        let select_field = self::extract_partial_select_field(*partial_select_field);
        field_asts.push(select_field);
    }

    let omit_asts = match *opt_omit {
        Combinator::Void => vec![],
        Combinator::Seq2(_, partial_select_omit) => {
            self::extract_partial_select_omit(*partial_select_omit)
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
) -> Option<(SelectTransform, usize)> {
    self::extract_opt_partial_where_guard(comb)
        .map(|(op, span_end)| (SelectTransform::WhereGuard(op), span_end))
}

pub(crate) fn extract_opt_partial_where_guard(
    comb: Combinator<Ast<'_>>,
) -> Option<(Box<Ast<'_>>, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Single(partial_where_guard) => {
            let (_, alt) = partial_where_guard.unwrap_temp().unwrap_seq2();

            match alt.unwrap_choice() {
                Choice::A(x) => {
                    let op = x.unwrap_single();
                    let span_end = op.span.end;
                    Some((Box::new(op), span_end))
                }
                Choice::B(x) => {
                    let op_star = x.unwrap_single();
                    let span_end = op_star.span.end;
                    Some((Box::new(Ast::new(op_star.span, Wildcard)), span_end))
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_select_with_indices(
    comb: Combinator<Ast<'_>>,
) -> Option<(SelectTransform, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Single(partial_select_with_indices) => {
            match partial_select_with_indices.unwrap_temp().unwrap_choice() {
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

                    Some((SelectTransform::WithIndexes(indices), span_end))
                }
                Choice::B(x) => {
                    let (_, _, index) = x.unwrap_seq3();
                    let index = index.unwrap_single();

                    Some((SelectTransform::WithNoIndex, index.span.end))
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_select_group_by(
    comb: Combinator<Ast<'_>>,
) -> Option<(SelectTransform, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Single(partial_select_group_by) => {
            let (_, _, range_op, range_ops) = partial_select_group_by.unwrap_temp().unwrap_seq4();

            let range_op = range_op.unwrap_single();
            let mut span_end = range_op.span.end;

            let mut range_op_asts = vec![range_op];
            for range_op in range_ops.unwrap_many() {
                let range_op = range_op.unwrap_seq2().1.unwrap_single();
                span_end = range_op.span.end;
                range_op_asts.push(range_op);
            }

            Some((SelectTransform::GroupBy(range_op_asts), span_end))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_select_order_by(
    comb: Combinator<Ast<'_>>,
) -> Option<(SelectTransform, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Single(partial_select_order_by) => {
            let (_, _, range_op, range_ops, opt_alt) =
                partial_select_order_by.unwrap_temp().unwrap_seq5();

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
) -> Option<(SelectTransform, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Single(partial_select_limit_to) => {
            let (_, _, range_op) = partial_select_limit_to.unwrap_temp().unwrap_seq3();
            let range_op = range_op.unwrap_single();
            let span_end = range_op.span.end;

            Some((SelectTransform::LimitTo(Box::new(range_op)), span_end))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn extract_opt_partial_select_start_at(
    comb: Combinator<Ast<'_>>,
) -> Option<(SelectTransform, usize)> {
    match comb {
        Combinator::Void => None,
        Combinator::Single(partial_select_start_at) => {
            let (_, _, range_op) = partial_select_start_at.unwrap_temp().unwrap_seq3();
            let range_op = range_op.unwrap_single();
            let span_end = range_op.span.end;

            Some((SelectTransform::StartAt(Box::new(range_op)), span_end))
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
