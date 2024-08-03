use zeroql_macros::{backtrack, memoize};

use crate::{
    ast::{Ast, AstKind, RelateArrow},
    lexer::TokenKind::*,
    parse,
    parser::{Parser, ParserResult},
};

use super::{Choice, Combinator};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[memoize(cache = self.cache, state = self.lexer.state)]
#[backtrack(state = self.lexer.state, condition = |r| matches!(r, Ok(None)))]
impl<'a> Parser<'a> {
    /// Parses a parens operation.
    ///
    /// ```txt
    /// parens_op =
    ///     | "(" exp ")"
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_parens_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            (arg parse_tok OpOpenParen) parse_op (arg parse_tok OpCloseParen)
        ));

        let ast = result.map(|x| x.unwrap_seq3().1.unwrap_single());

        Ok(ast)
    }

    /// Parses a scope operation.
    ///
    /// ```txt
    /// id_op =
    ///     | identifier ":" (lit | identifier | variable | op_star)
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_id_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_identifier
            (arg parse_tok OpColon)
            (alt parse_lit parse_identifier parse_variable (arg parse_tok OpStar))
        ));

        let ast = result.map(|x| {
            let (ident, _, value) = x.unwrap_seq3();
            let value = match value.unwrap_choice() {
                Choice::A(x) => x.unwrap_single(),
                Choice::B(x) => x.unwrap_single(),
                Choice::C(x) => x.unwrap_single(),
                Choice::D(x) => Ast::new(x.unwrap_single().get_span(), AstKind::Star),
                _ => unreachable!(),
            };

            let ident = ident.unwrap_single();
            Ast::new(
                ident.get_span().start..value.get_span().end,
                AstKind::IdOp(Box::new(ident), Box::new(value)),
            )
        });

        Ok(ast)
    }

    /// Parses a scope operation.
    ///
    /// ```txt
    /// identifier_scope_op =
    ///     | identifier (op_scope identifier)+
    ///     | identifier
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_identifier_scope_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_identifier
                (many_1 (seq (arg parse_tok OpScope) parse_identifier))
            )
            parse_identifier
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (ident, scopes) = x.unwrap_seq2();
                let ident = ident.unwrap_single();
                let start = ident.get_span().start;
                let mut end = ident.get_span().end;

                let mut scoped_identifiers = vec![ident];
                for scope in scopes.unwrap_many() {
                    let (_, scope) = scope.unwrap_seq2();
                    let scope = scope.unwrap_single();
                    end = scope.get_span().end;

                    scoped_identifiers.push(scope);
                }

                Ast::new(start..end, AstKind::ScopedIdentifier(scoped_identifiers))
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses an atom operation.
    ///
    /// ```txt
    /// atom_op =
    ///     | variable
    ///     | lit
    ///     | id_op
    ///     | identfier_scope_op
    ///     | parens_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_atom_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_variable
            parse_lit
            parse_id_op
            parse_identifier_scope_op
            parse_parens_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => x.unwrap_single(),
            Choice::B(x) => x.unwrap_single(),
            Choice::C(x) => x.unwrap_single(),
            Choice::D(x) => x.unwrap_single(),
            Choice::E(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses an index operation.
    ///
    /// ```txt
    /// index_op =
    ///     | atom_op "[" exp "]"
    ///     | atom_op
    /// ````
    #[memoize]
    #[backtrack]
    pub fn parse_index_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq parse_atom_op (arg parse_tok OpOpenSquareBracket) parse_op (arg parse_tok OpCloseSquareBracket)) // TODO: Should be parse_exp
            parse_atom_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (atom_op, _, exp, close) = x.unwrap_seq4();
                let atom_op = atom_op.unwrap_single();
                let exp = exp.unwrap_single();

                Ast::new(
                    atom_op.get_span().start..close.unwrap_single().get_span().end,
                    AstKind::Index {
                        subject: Box::new(atom_op),
                        index: Box::new(exp),
                    },
                )
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a function argument.
    ///
    /// ```txt
    /// function_arg =
    ///     | (identifier op_is_lexer)? op
    /// ```
    #[memoize]
    #[backtrack]
    fn parse_function_arg(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            (opt (seq parse_identifier (arg parse_tok OpIsLexer)))
            parse_op
        ));

        let ast = result.map(|x| {
            let (name_part, op) = x.unwrap_seq2();
            let name = match *name_part {
                Combinator::Seq2(name, _) => Some(name.unwrap_single()),
                Combinator::Void => None,
                _ => unreachable!(),
            };

            let op = op.unwrap_single();
            let start = name
                .as_ref()
                .map(|n| n.get_span().start)
                .unwrap_or(op.get_span().start);

            Ast::new(
                start..op.get_span().end,
                AstKind::FunctionArg {
                    name: name.map(Box::new),
                    value: Box::new(op),
                },
            )
        });

        Ok(ast)
    }

    /// Parses a function call operation.
    ///
    /// ```txt
    /// function_call_op =
    ///     | index_op "(" (function_arg ("," function_arg)* ","?)? ")"
    ///     | index_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_function_call_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_index_op
                (arg parse_tok OpOpenParen)
                (opt (seq
                    parse_function_arg
                    (many_0 (seq (arg parse_tok OpComma) parse_function_arg))
                    (opt (arg parse_tok OpComma))
                ))
                (arg parse_tok OpCloseParen)
            )
            parse_index_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (index_op, _, args, close) = x.unwrap_seq4();
                let subject = Box::new(index_op.unwrap_single());
                let args = match *args {
                    Combinator::Seq3(arg0, args, _) => {
                        let mut arg_asts = vec![arg0.unwrap_single()];
                        for arg in args.unwrap_many() {
                            let (_, arg) = arg.unwrap_seq2();
                            arg_asts.push(arg.unwrap_single());
                        }

                        arg_asts
                    }
                    Combinator::Void => vec![],
                    _ => unreachable!(),
                };

                Ast::new(
                    subject.get_span().start..close.unwrap_single().get_span().end,
                    AstKind::FunctionCall { subject, args },
                )
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a not operation.
    ///
    /// ```txt
    /// not_op =
    ///     | (op_not | op_match_lexer) function_call_op
    ///     | function_call_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_not_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                (alt parse_op_not (arg parse_tok OpMatchLexer))
                parse_function_call_op
            )
            parse_function_call_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (op, atom_op) = x.unwrap_seq2();
                let atom_op = atom_op.unwrap_single();

                match op.unwrap_choice() {
                    Choice::A(x) => Ast::new(
                        x.unwrap_single().get_span().start..atom_op.get_span().end,
                        AstKind::LogicalNotOp(Box::new(atom_op)),
                    ),
                    Choice::B(x) => Ast::new(
                        x.unwrap_single().get_span().start..atom_op.get_span().end,
                        AstKind::BitwiseNotOp(Box::new(atom_op)),
                    ),
                    _ => unreachable!(),
                }
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a sign operation.
    ///
    /// ```txt
    /// sign_op =
    ///     | (op_plus | op_minus) not_op
    ///     | not_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_sign_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq (alt (arg parse_tok OpPlus) (arg parse_tok OpMinus)) parse_not_op)
            parse_not_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (op, atom_op) = x.unwrap_seq2();
                let atom_op = atom_op.unwrap_single();

                match op.unwrap_choice() {
                    Choice::A(x) => Ast::new(
                        x.unwrap_single().get_span().start..atom_op.get_span().end,
                        AstKind::PlusSignOp(Box::new(atom_op)),
                    ),
                    Choice::B(x) => Ast::new(
                        x.unwrap_single().get_span().start..atom_op.get_span().end,
                        AstKind::MinusSignOp(Box::new(atom_op)),
                    ),
                    _ => unreachable!(),
                }
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a access operation.
    ///
    /// ```txt
    /// op_access =
    ///     | op_dot
    ///     | op_safe_nav
    ///
    /// access_op = (* Left Associative *)
    ///     | sign_op (op_access identifier)+
    ///     | sign_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_access_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_sign_op
                (many_1 (seq
                    (alt (arg parse_tok OpDot) (arg parse_tok OpSafeNav))
                    parse_identifier
                ))
            )
            parse_sign_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (op, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;

                    let ast = match op.unwrap_choice() {
                        Choice::A(_) => AstKind::DotAccessOp(Box::new(l), Box::new(r)),
                        Choice::B(_) => AstKind::SafeNavigationAccessOp(Box::new(l), Box::new(r)),
                        _ => unreachable!(),
                    };

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a power operation.
    ///
    /// ```txt
    /// pow_op = (* Right Associative *)
    ///     | (access_op op_pow)+ access_op
    ///     | access_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_pow_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                (many_1 (seq
                    parse_access_op
                    (arg parse_tok OpPow)
                ))
                parse_access_op
            )
            parse_access_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (rest, r) = x.unwrap_seq2();
                let mut r = r.unwrap_single();

                // Handle right associativity
                for comb in rest.unwrap_many().into_iter().rev() {
                    let (l, _) = comb.unwrap_seq2();
                    let l = l.unwrap_single();
                    r = Ast::new(
                        l.get_span().start..r.get_span().end,
                        AstKind::ExponentiationOp(Box::new(l), Box::new(r)),
                    );
                }

                r
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a multiplicative operation.
    ///
    /// ```txt
    /// op_multiplicative =
    ///     | op_mul
    ///     | op_div
    ///     | op_mod
    ///
    /// mul_op = (* Left Associative *)
    ///     | pow_op (op_multiplicative pow_op)+
    ///     | pow_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_mul_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_pow_op
                (many_1 (seq
                    (alt parse_op_mul (arg parse_tok OpDiv) (arg parse_tok OpMod))
                    parse_pow_op
                ))
            )
            parse_pow_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (op, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;

                    let ast = match op.unwrap_choice() {
                        Choice::A(_) => AstKind::MultiplicationOp(Box::new(l), Box::new(r)),
                        Choice::B(_) => AstKind::DivisionOp(Box::new(l), Box::new(r)),
                        Choice::C(_) => AstKind::ModulusOp(Box::new(l), Box::new(r)),
                        _ => unreachable!(),
                    };

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses an additive operation.
    ///
    /// ```txt
    /// op_additive =
    ///     | op_plus
    ///     | op_minus
    ///
    /// add_op = (* Left Associative *)
    ///     | mul_op (op_additive mul_op)+
    ///     | mul_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_add_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_mul_op
                (many_1 (seq
                    (alt (arg parse_tok OpPlus) (arg parse_tok OpMinus))
                    parse_mul_op
                ))
            )
            parse_mul_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (op, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;

                    let ast = match op.unwrap_choice() {
                        Choice::A(_) => AstKind::AdditionOp(Box::new(l), Box::new(r)),
                        Choice::B(_) => AstKind::SubtractionOp(Box::new(l), Box::new(r)),
                        _ => unreachable!(),
                    };

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a shift operation.
    ///
    /// ```txt
    /// op_shift =
    ///     | op_shl
    ///     | op_shr
    ///
    /// shift_op = (* Left Associative *)
    ///     | add_op (op_shift add_op)+
    ///     | add_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_shift_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_add_op
                (many_1 (seq
                    (alt (arg parse_tok OpShl) (arg parse_tok OpShr))
                    parse_add_op
                ))
            )
            parse_add_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (op, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;

                    let ast = match op.unwrap_choice() {
                        Choice::A(_) => AstKind::LeftShiftOp(Box::new(l), Box::new(r)),
                        Choice::B(_) => AstKind::RightShiftOp(Box::new(l), Box::new(r)),
                        _ => unreachable!(),
                    };

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a match similarity operation.
    ///
    /// ```txt
    /// op_match_sim =
    ///     | op_match
    ///     | op_not_match
    ///     | op_similarity
    ///
    /// match_sim_op = (* Left Associative *)
    ///     | shift_op (op_match_sim shift_op)+
    ///     | shift_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_match_sim_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_shift_op
                (many_1 (seq
                    (alt parse_op_match parse_op_not_match (arg parse_tok OpSimilarity))
                    parse_shift_op
                ))
            )
            parse_shift_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (op, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;

                    let ast = match op.unwrap_choice() {
                        Choice::A(_) => AstKind::MatchOp(Box::new(l), Box::new(r)),
                        Choice::B(_) => AstKind::NotMatchOp(Box::new(l), Box::new(r)),
                        Choice::C(_) => AstKind::SimilarityOp(Box::new(l), Box::new(r)),
                        _ => unreachable!(),
                    };

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a relational operation.
    ///
    /// ```txt
    /// op_relational =
    ///     | op_lt
    ///     | op_gt
    ///     | op_lte
    ///     | op_gte
    ///     | op_in
    ///     | op_not_in
    ///     | op_not_contains
    ///     | op_contains_none
    ///     | op_contains_all
    ///     | op_contains_any
    ///     | op_contains
    ///
    /// rel_op = (* Left Associative *)
    ///     | match_sim_op (op_relational match_sim_op)+
    ///     | match_sim_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_rel_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_match_sim_op
                (many_1 (seq
                    (alt
                        (arg parse_tok OpLt)
                        (arg parse_tok OpGt)
                        (arg parse_tok OpLte)
                        (arg parse_tok OpGte)
                        (alt
                            parse_op_in
                            parse_op_not_in
                            parse_op_not_contains
                            parse_op_contains_none
                            parse_op_contains_all
                            parse_op_contains_any
                            parse_op_contains
                        )
                    )
                    parse_match_sim_op
                ))
            )
            parse_match_sim_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (op, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;

                    let ast = match op.unwrap_choice() {
                        Choice::A(_) => AstKind::LessThanOp(Box::new(l), Box::new(r)),
                        Choice::B(_) => AstKind::GreaterThanOp(Box::new(l), Box::new(r)),
                        Choice::C(_) => AstKind::LessThanEqualToOp(Box::new(l), Box::new(r)),
                        Choice::D(_) => AstKind::GreaterThanEqualToOp(Box::new(l), Box::new(r)),
                        Choice::E(x) => match x.unwrap_choice() {
                            Choice::A(_) => AstKind::InOp(Box::new(l), Box::new(r)),
                            Choice::B(_) => AstKind::NotInOp(Box::new(l), Box::new(r)),
                            Choice::C(_) => AstKind::ContainsOp(Box::new(l), Box::new(r)),
                            Choice::D(_) => AstKind::NotContainsOp(Box::new(l), Box::new(r)),
                            Choice::E(_) => AstKind::ContainsNoneOp(Box::new(l), Box::new(r)),
                            Choice::F(_) => AstKind::ContainsAllOp(Box::new(l), Box::new(r)),
                            Choice::G(_) => AstKind::ContainsAnyOp(Box::new(l), Box::new(r)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses an equality operation.
    ///
    /// ```txt
    /// op_eq_is =
    ///     | op_eq
    ///     | op_is_not
    ///     | op_is
    ///
    /// eq_op = (* Left Associative *)
    ///     | rel_op (op_eq_is eq_op)+
    ///     | rel_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_eq_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_rel_op
                (many_1 (seq
                    (alt (arg parse_tok OpEq) parse_op_is_not parse_op_is)
                    parse_rel_op
                ))
            )
            parse_rel_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (op, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;

                    let ast = match op.unwrap_choice() {
                        Choice::A(_) => AstKind::EqualToOp(Box::new(l), Box::new(r)),
                        Choice::B(_) => AstKind::IsNotOp(Box::new(l), Box::new(r)),
                        Choice::C(_) => AstKind::IsOp(Box::new(l), Box::new(r)),
                        _ => unreachable!(),
                    };

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a bitwise and operation.
    ///
    /// ```txt
    /// bit_and_op = (* Left Associative *)
    ///     | eq_op (op_bit_and eq_op)+
    ///     | eq_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_bit_and_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_eq_op
                (many_1 (seq
                    (arg parse_tok OpBitAnd)
                    parse_eq_op
                ))
            )
            parse_eq_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (_, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;
                    let ast = AstKind::BitwiseAndOp(Box::new(l), Box::new(r));

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a bitwise xor operation.
    ///
    /// ```txt
    /// bit_xor_op = (* Left Associative *)
    ///     | bit_and_op (op_bit_xor bit_and_op)+
    ///     | bit_and_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_bit_xor_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_bit_and_op
                (many_1 (seq
                    (arg parse_tok OpBitXor)
                    parse_bit_and_op
                ))
            )
            parse_bit_and_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (_, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;
                    let ast = AstKind::BitwiseXorOp(Box::new(l), Box::new(r));

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a bitwise or operation.
    ///
    /// ```txt
    /// bit_or_op = (* Left Associative *)
    ///     | bit_xor_op (op_bit_or bit_xor_op)+
    ///     | bit_xor_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_bit_or_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_bit_xor_op
                (many_1 (seq
                    (arg parse_tok OpBitOr)
                    parse_bit_xor_op
                ))
            )
            parse_bit_xor_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (_, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;
                    let ast = AstKind::BitwiseOrOp(Box::new(l), Box::new(r));

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses an and operation.
    ///
    /// ```txt
    /// and_op = (* Left Associative *)
    ///     | bit_or_op (op_and bit_or_op)+
    ///     | bit_or_op
    /// ```
    ///
    #[memoize]
    #[backtrack]
    pub fn parse_and_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_bit_or_op
                (many_1 (seq
                    parse_op_and
                    parse_bit_or_op
                ))
            )
            parse_bit_or_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (_, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;
                    let ast = AstKind::LogicalAndOp(Box::new(l), Box::new(r));

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses an or null coalesce operation.
    ///
    /// ```txt
    /// op_or_null_coalesce =
    ///     | op_or
    ///     | op_null_coalesce
    ///
    /// or_null_coalesce_op = (* Left Associative *)
    ///     | and_op op_or_null_coalesce and_op
    ///     | and_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_or_null_coalesce_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_and_op
                (many_1 (seq
                    (alt parse_op_or (arg parse_tok OpNullCoalesce))
                    parse_and_op
                ))
            )
            parse_and_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (op, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;

                    let ast = match op.unwrap_choice() {
                        Choice::A(_) => AstKind::LogicalOrOp(Box::new(l), Box::new(r)),
                        Choice::B(_) => AstKind::NullCoalesceOp(Box::new(l), Box::new(r)),
                        _ => unreachable!(),
                    };

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a range operation.
    ///
    /// ```txt
    /// op_ranges =
    ///     | op_range_incl
    ///     | op_range
    ///
    /// range_op =
    ///     | or_null_coalesce_op op_ranges or_null_coalesce_op
    ///     | or_null_coalesce_op
    /// ```
    ///
    #[memoize]
    #[backtrack]
    pub fn parse_range_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_or_null_coalesce_op
                (alt (arg parse_tok OpRangeIncl) (arg parse_tok OpRange))
                parse_or_null_coalesce_op
            )
            parse_or_null_coalesce_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, op, r) = x.unwrap_seq3();
                let l = l.unwrap_single();
                let r = r.unwrap_single();

                match op.unwrap_choice() {
                    Choice::A(_) => Ast::new(
                        l.get_span().start..r.get_span().end,
                        AstKind::RangeInclusiveOp(Box::new(l), Box::new(r)),
                    ),
                    Choice::B(_) => Ast::new(
                        l.get_span().start..r.get_span().end,
                        AstKind::RangeOp(Box::new(l), Box::new(r)),
                    ),
                    _ => unreachable!(),
                }
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a single relate id.
    ///
    /// ```txt
    /// single_relate_id =
    ///     | id_op
    ///     | identifier
    ///     | range_op
    ///     | op_star
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_single_relate_id(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_id_op
            parse_identifier
            parse_range_op
            (arg parse_tok OpStar)
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => x.unwrap_single(),
            Choice::B(x) => x.unwrap_single(),
            Choice::C(x) => x.unwrap_single(),
            Choice::D(x) => Ast::new(x.unwrap_single().get_span(), AstKind::Star),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a multi relate id.
    ///
    /// ```txt
    /// multi_relate_id =
    ///     | "[" single_relate_id ("," single_relate_id)* ","? "]"
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_multi_relate_id(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            (arg parse_tok OpOpenSquareBracket)
            parse_single_relate_id
            (many_0 (seq
                (arg parse_tok OpComma)
                parse_single_relate_id
            ))
            (opt (arg parse_tok OpComma))
            (arg parse_tok OpCloseSquareBracket)
        ));

        let ast = result.map(|x| {
            let (open, id0, ids, _, close) = x.unwrap_seq5();
            let mut id_asts = vec![id0.unwrap_single()];

            for id in ids.unwrap_many() {
                let (_, id) = id.unwrap_seq2();
                id_asts.push(id.unwrap_single());
            }

            Ast::new(
                open.unwrap_single().get_span().start..close.unwrap_single().get_span().end,
                AstKind::ListLiteral(id_asts),
            )
        });

        Ok(ast)
    }

    /// Parses a relate id.
    ///
    /// ```txt
    /// relate_id =
    ///     | multi_relate_id
    ///     | single_relate_id
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_relate_id(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_multi_relate_id
            parse_single_relate_id
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => x.unwrap_single(),
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a relate edge id.
    ///
    /// ```txt
    /// relate_edge_id =
    ///     | identifier "[" op "]"
    ///     | identifier
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_relate_edge_id(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_identifier
                (arg parse_tok OpOpenSquareBracket)
                parse_op
                (arg parse_tok OpCloseSquareBracket)
            )
            parse_identifier
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (ident, _, op, close) = x.unwrap_seq4();
                let ident = ident.unwrap_single();
                let op = op.unwrap_single();
                let close = close.unwrap_single();

                let span = ident.get_span().start..close.get_span().end;
                let ast = AstKind::RelateEdgeId(Box::new(ident), Some(Box::new(op)));

                Ast::new(span, ast)
            }
            Choice::B(x) => {
                let id = x.unwrap_single();
                Ast::new(id.get_span(), AstKind::RelateEdgeId(Box::new(id), None))
            }
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a relate edge not operation.
    ///
    /// ```txt
    /// relate_edge_not_op =
    ///     | op_not relate_edge_id
    ///     | relate_edge_id
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_relate_edge_not_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq parse_op_not parse_relate_edge_id)
            parse_relate_edge_id
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (_, id) = x.unwrap_seq2();
                let id = id.unwrap_single();
                Ast::new(id.get_span(), AstKind::LogicalNotOp(Box::new(id)))
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a relate edge and operation.
    ///
    /// ```txt
    /// relate_edge_and_op = (* Left Associative *)
    ///     | relate_edge_not_op (op_and relate_edge_not_op)+
    ///     | relate_edge_not_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_relate_edge_and_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_relate_edge_not_op
                (many_1 (seq
                    parse_op_and
                    parse_relate_edge_not_op
                ))
            )
            parse_relate_edge_not_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (_, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;
                    let ast = AstKind::LogicalAndOp(Box::new(l), Box::new(r));

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a relate edge or operation.
    ///
    /// ```txt
    /// relate_edge_or_op = (* Left Associative *)
    ///     | relate_edge_and_op (op_or relate_edge_and_op)+
    ///     | relate_edge_and_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_relate_edge_or_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            (seq
                parse_relate_edge_and_op
                (many_1 (seq
                    parse_op_or
                    parse_relate_edge_and_op
                ))
            )
            parse_relate_edge_and_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => {
                let (l, rest) = x.unwrap_seq2();
                let mut l = l.unwrap_single();

                // Handle left associativity
                for comb in rest.unwrap_many() {
                    let (_, r) = comb.unwrap_seq2();
                    let r = r.unwrap_single();
                    let span = l.get_span().start..r.get_span().end;
                    let ast = AstKind::LogicalOrOp(Box::new(l), Box::new(r));

                    l = Ast::new(span, ast);
                }

                l
            }
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }

    /// Parses a relate operation.
    ///
    /// ```txt
    /// relate_op = (* Left Associative *)
    ///     | relate_id (op_arrow relate_edge_or_op op_arrow relate_id)+
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_relate_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            parse_relate_id
            (many_1 (seq
                (alt
                    (arg parse_tok OpArrowLeft)
                    (arg parse_tok OpArrowRight)
                    (arg parse_tok OpMultiArrowLeft)
                    (arg parse_tok OpMultiArrowRight)
                )
                parse_relate_edge_or_op
                (alt
                    (arg parse_tok OpArrowLeft)
                    (arg parse_tok OpArrowRight)
                    (arg parse_tok OpMultiArrowLeft)
                    (arg parse_tok OpMultiArrowRight)
                )
                parse_relate_id
            ))
        ));

        let ast = result.map(|x| {
            let (l, rest) = x.unwrap_seq2();
            let mut l = l.unwrap_single();

            // Handle left associativity
            for comb in rest.unwrap_many() {
                let (arr_l, e, arr_r, r) = comb.unwrap_seq4();
                let e = e.unwrap_single();
                let r = r.unwrap_single();

                let arr_l = match arr_l.unwrap_choice() {
                    Choice::A(_) => RelateArrow::Left,
                    Choice::B(_) => RelateArrow::Right,
                    Choice::C(_) => RelateArrow::MultiLeft,
                    Choice::D(_) => RelateArrow::MultiRight,
                    _ => unreachable!(),
                };

                let arr_r = match arr_r.unwrap_choice() {
                    Choice::A(_) => RelateArrow::Left,
                    Choice::B(_) => RelateArrow::Right,
                    Choice::C(_) => RelateArrow::MultiLeft,
                    Choice::D(_) => RelateArrow::MultiRight,
                    _ => unreachable!(),
                };

                let span = l.get_span().start..r.get_span().end;
                let ast = AstKind::RelateOp(Box::new(l), arr_l, Box::new(e), arr_r, Box::new(r));

                l = Ast::new(span, ast);
            }

            l
        });

        Ok(ast)
    }

    /// Parses an operation.
    ///
    /// ```txt
    /// op =
    ///     | relate_op
    ///     | range_op
    /// ```
    #[memoize]
    #[backtrack]
    pub fn parse_op(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (alt
            parse_relate_op
            parse_range_op
        ));

        let ast = result.map(|x| match x.unwrap_choice() {
            Choice::A(x) => x.unwrap_single(),
            Choice::B(x) => x.unwrap_single(),
            _ => unreachable!(),
        });

        Ok(ast)
    }
}
