use zeroql_macros::{backtrack, memoize};

use crate::{
    ast::{Ast, AstKind::*},
    lexer::TokenKind::*,
    parse,
    parser::Choice,
};

use super::{Combinator, Parser, ParserResult};

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
    ///     | kw_create (id_op | identifier) partial_set_object
    ///     | kw_create (id_op | identifier) partial_set_assign
    ///     | kw_create identifier kw_set "(" (identifier ("," identifier)* ","?)? ")" kw_values tuple_lit ("," tuple_lit)*
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_create_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq parse_kw_create (alt parse_id_op parse_identifier) parse_partial_set_object)
            (seq
                parse_kw_create
                (alt parse_id_op parse_identifier)
                parse_partial_set_assign
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
                let (kw_create, subject, partial_set_object) = x.unwrap_seq3();

                let subject = match subject.unwrap_choice() {
                    Choice::A(x) => x.unwrap_single(),
                    Choice::B(x) => x.unwrap_single(),
                    _ => unreachable!(),
                };

                let object = partial_set_object
                    .unwrap_single()
                    .unwrap_temp()
                    .unwrap_seq2()
                    .1;

                if let Ast {
                    span,
                    kind: ObjectLiteral(tuples),
                } = object.unwrap_single()
                {
                    let (columns, value_tuple): (Vec<_>, Vec<_>) = tuples.into_iter().unzip();

                    Ast::new(
                        kw_create.unwrap_single().span.start..span.end,
                        Create {
                            subject: Box::new(subject),
                            values: vec![value_tuple],
                            columns,
                        },
                    )
                } else {
                    unreachable!();
                }
            }
            Choice::B(x) => {
                let (kw_create, subject, parse_partial_set_assign) = x.unwrap_seq3();
                let (_, column, _, value, kvs) = parse_partial_set_assign
                    .unwrap_single()
                    .unwrap_temp()
                    .unwrap_seq5();

                let subject = match subject.unwrap_choice() {
                    Choice::A(x) => x.unwrap_single(),
                    Choice::B(x) => x.unwrap_single(),
                    _ => unreachable!(),
                };

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

                Ast::new(
                    kw_create.unwrap_single().span.start..span_end,
                    Create {
                        subject: Box::new(subject),
                        values: vec![value_tuple],
                        columns,
                    },
                )
            }
            Choice::C(x) => {
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

    /// Parses `RELATE` expression.
    ///
    /// ```txt
    /// relate_exp =
    ///     | kw_relate relate_op << partial_where_guard? (partial_set_object | partial_set_assign)? >>
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_relate_exp(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_kw_relate
            parse_relate_op
            (perm
                (opt parse_partial_where_guard)
                (opt (alt
                    parse_partial_set_object
                    parse_partial_set_assign
                ))
            )
        ));

        // let ast = result.map(|x| {
        //     let (kw_relate, relate_op, alt) = x.unwrap_seq3();

        //     let relate_op = relate_op.unwrap_single();
        //     let mut span_end = relate_op.span.end;

        //     let (where_guard, columns, value) = match alt.unwrap_choice() {
        //         Choice::A(x) => {
        //             let (where_guard, set_object) = x.unwrap_seq2();

        //             let where_guard = match *where_guard {
        //                 Combinator::Void => None,
        //                 Combinator::Seq2(_, op) => {
        //                     let op = op.unwrap_single();
        //                     span_end = op.span.end;

        //                     Some(Box::new(op))
        //                 }
        //                 _ => unreachable!(),
        //             };

        //             let (columns, value) = match *set_object {
        //                 Combinator::Void => (vec![], vec![]),
        //                 Combinator::Seq2(_, object) => {
        //                     let object = object.unwrap_single();
        //                     span_end = object.span.end;

        //                     if let Ast {
        //                         kind: ObjectLiteral(tuples),
        //                         ..
        //                     } = object
        //                     {
        //                         tuples.into_iter().unzip()
        //                     } else {
        //                         unreachable!()
        //                     }
        //                 }
        //                 _ => unreachable!(),
        //             };

        //             (where_guard, columns, value)
        //         }
        //         Choice::B(x) => {
        //             let (where_guard, set_assign) = x.unwrap_seq2();

        //             let where_guard = match *where_guard {
        //                 Combinator::Void => None,
        //                 Combinator::Seq2(_, op) => {
        //                     let op = op.unwrap_single();
        //                     span_end = op.span.end;

        //                     Some(Box::new(op))
        //                 }
        //                 _ => unreachable!(),
        //             };
        //         }
        //         _ => unreachable!(),
        //     };

        //     Ast::new(
        //         kw_relate.unwrap_single().span.start..span_end,
        //         Relate {
        //             relate_op: Box::new(relate_op),
        //             columns,
        //             value,
        //             where_guard,
        //         },
        //     )
        // });

        // Ok(ast)

        todo!()
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

    /// Parses partial `set_assign` syntax.
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

    // partial_target =
    //     | relate_op
    //     | id_op
    //     | identifier

    // delete_exp =
    //     | kw_delete partial_target partial_where_guard?

    // update_exp =
    //     | kw_update partial_target << partial_where_guard? partial_set_object >>
    //     | kw_update partial_target << partial_where_guard? partial_set_assign >>

    // partial_partial_select_field_fold =
    //     | kw_fold function_call_op

    // partial_select_as =
    //     | kw_as identifier

    // partial_select_field =
    //     partial_| partial_select_field_fold partial_select_as?
    //     | op partial_select_as?
    //     | index_op (op_dot | op_safe_nav) op_star
    //     | op_star

    // partial_select_omit =
    //     | kw_omit identifier ("," identifier)*

    // partial_select_fields =
    //     | partial_select_field ("," partial_select_field)* (","? partial_select_omit)?

    // partial_select_from_table_relate_id_as =
    //     | relate_id partial_select_as?

    // partial_select_from_table_relate =
    //     | (partial_select_from_table_relate_id_as op_arrow)+ partial_select_from_table_relate_id_as

    // partial_select_from_table =
    //     | op partial_select_as?
    //     | partial_select_from_table_relate

    // partial_select_from_tables =
    //     | partial_select_from_table ("," partial_select_from_table)*

    // partial_select_from =
    //     | kw_from partial_select_from_tables
    //     | kw_from op_star

    // partial_select_with_indices =
    //     | kw_with kw_indices identifier ("," identifier)*
    //     | kw_with kw_no kw_index

    // partial_select_group_by =
    //     | kw_group kw_by? identifier ("," identifier)*

    // partial_select_order_by =
    //     | kw_order kw_by? identifier ("," identifier)* (kw_asc | kw_desc)?

    // partial_select_start_at =
    //     | kw_start kw_at? op

    // partial_select_limit_to =
    //     | kw_limit kw_to? op

    // select_exp =
    //     | kw_select partial_select_fields partial_select_from << partial_where_guard? partial_select_with_indices? partial_select_group_by? partial_select_order_by? partial_select_start_at? partial_select_limit_to? >>

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
