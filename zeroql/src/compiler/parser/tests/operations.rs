use tracing::info;

use crate::{
    ast::{Ast, AstKind::*, RelateArrow::*},
    lexer::RegexFlags,
    parser::Parser,
};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test]
fn test_parser_parens_op() -> anyhow::Result<()> {
    // TODO: Need more op examples
    let parser = &mut Parser::new("('Hello')", 20);
    let result_a = parser.parse_parens_op()?;
    // let result_b = parser.parse_parens_op()?;

    info!(
        r#"input = {:?} | parse_parens_op = {:?}"#,
        parser.lexer.string, result_a,
    );

    assert_eq!(result_a, Some(Ast::new(1..8, StringLiteral("Hello"))));

    Ok(())
}

#[test_log::test]
fn test_parser_id_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new("a1:0o1770 b2:_xyz c3:$var d4:*", 20);
    let result_a = parser.parse_id_op()?;
    let result_b = parser.parse_id_op()?;
    let result_c = parser.parse_id_op()?;
    let result_d = parser.parse_id_op()?;

    info!(
        r#"input = {:?} | parse_id_op parse_id_op parse_id_op parse_id_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..9,
            kind: IdOp(
                Box::new(Ast {
                    span: 0..2,
                    kind: Identifier("a1"),
                }),
                Box::new(Ast {
                    span: 3..9,
                    kind: IntegerLiteral(0o1770),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 10..17,
            kind: IdOp(
                Box::new(Ast {
                    span: 10..12,
                    kind: Identifier("b2"),
                }),
                Box::new(Ast {
                    span: 13..17,
                    kind: Identifier("_xyz"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 18..25,
            kind: IdOp(
                Box::new(Ast {
                    span: 18..20,
                    kind: Identifier("c3"),
                }),
                Box::new(Ast {
                    span: 21..25,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 26..30,
            kind: IdOp(
                Box::new(Ast {
                    span: 26..28,
                    kind: Identifier("d4"),
                }),
                Box::new(Ast {
                    span: 29..30,
                    kind: Wildcard,
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_identifier_scope_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new("a1 a1::b2 a1::b2::c3", 20);
    let result_a = parser.parse_identifier_scope_op()?;
    let result_b = parser.parse_identifier_scope_op()?;
    let result_c = parser.parse_identifier_scope_op()?;

    info!(
        r#"input = {:?} | parse_identifier_scope_op parse_identifier_scope_op parse_identifier_scope_op = {:?} {:?} {:?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(result_a, Some(Ast::new(0..2, Identifier("a1"))));
    assert_eq!(
        result_b,
        Some(Ast::new(
            3..9,
            ScopedIdentifier(vec![
                Ast::new(3..5, Identifier("a1")),
                Ast::new(7..9, Identifier("b2")),
            ])
        ))
    );
    assert_eq!(
        result_c,
        Some(Ast::new(
            10..20,
            ScopedIdentifier(vec![
                Ast::new(10..12, Identifier("a1")),
                Ast::new(14..16, Identifier("b2")),
                Ast::new(18..20, Identifier("c3")),
            ])
        ))
    );

    Ok(())
}

#[test_log::test]
fn test_parser_atom_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new("(2.,) a1::b2 (//[a-z]//xim) $var", 20);
    let result_a = parser.parse_atom_op()?;
    let result_b = parser.parse_atom_op()?;
    let result_c = parser.parse_atom_op()?;
    let result_d = parser.parse_atom_op()?;

    info!(
        r#"input = {:?} | parse_atom_op parse_atom_op parse_atom_op parse_atom_op = {:?} {:?} {:?} {:?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast::new(
            0..5,
            TupleLiteral(vec![Ast::new(1..3, FloatLiteral(2.0))])
        ))
    );
    assert_eq!(
        result_b,
        Some(Ast::new(
            6..12,
            ScopedIdentifier(vec![
                Ast::new(6..8, Identifier("a1")),
                Ast::new(10..12, Identifier("b2"))
            ])
        ))
    );
    assert_eq!(
        result_c,
        Some(Ast::new(
            14..26,
            RegexLiteral {
                pattern: r"[a-z]",
                flags: RegexFlags::X_EXTENDED | RegexFlags::I_IGNORE_CASE | RegexFlags::M_MULTILINE,
            }
        ))
    );
    assert_eq!(result_d, Some(Ast::new(28..32, Variable("var"))));

    Ok(())
}

#[test_log::test]
fn test_parser_index_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new("$var[0] (ident)[5.0] [1,2,3][5]", 20);
    let result_a = parser.parse_index_op()?;
    let result_b = parser.parse_index_op()?;
    let result_c = parser.parse_index_op()?;

    info!(
        r#"input = {:?} | parse_index_op parse_index_op parse_index_op = {:?} {:?} {:?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..7,
            kind: Index {
                subject: Box::new(Ast {
                    span: 0..4,
                    kind: Variable("var")
                }),
                index: Box::new(Ast {
                    span: 5..6,
                    kind: IntegerLiteral(0)
                })
            }
        })
    );
    assert_eq!(
        result_b,
        Some(Ast {
            span: 9..20,
            kind: Index {
                subject: Box::new(Ast {
                    span: 9..14,
                    kind: Identifier("ident")
                }),
                index: Box::new(Ast {
                    span: 16..19,
                    kind: FloatLiteral(5.0)
                })
            }
        })
    );
    assert_eq!(
        result_c,
        Some(Ast {
            span: 21..31,
            kind: Index {
                subject: Box::new(Ast {
                    span: 21..28,
                    kind: ListLiteral(vec![
                        Ast {
                            span: 22..23,
                            kind: IntegerLiteral(1)
                        },
                        Ast {
                            span: 24..25,
                            kind: IntegerLiteral(2)
                        },
                        Ast {
                            span: 26..27,
                            kind: IntegerLiteral(3)
                        }
                    ])
                }),
                index: Box::new(Ast {
                    span: 29..30,
                    kind: IntegerLiteral(5)
                })
            }
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_function_call_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"([1, 2])(1, 3,) std::function(a = 12, b = //[^\s]+//im) $var(5, test = true) true[0]"#,
        100,
    );
    let result_a = parser.parse_function_call_op()?;
    let result_b = parser.parse_function_call_op()?;
    let result_c = parser.parse_function_call_op()?;
    let result_d = parser.parse_function_call_op()?;

    info!(
        r#"input = {:?} | parse_function_call_op parse_function_call_op parse_function_call_op parse_function_call_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 1..15,
            kind: FunctionCall {
                subject: Box::new(Ast {
                    span: 1..7,
                    kind: ListLiteral(vec![
                        Ast {
                            span: 2..3,
                            kind: IntegerLiteral(1),
                        },
                        Ast {
                            span: 5..6,
                            kind: IntegerLiteral(2),
                        },
                    ]),
                }),
                args: vec![
                    Ast {
                        span: 9..10,
                        kind: FunctionArg {
                            name: None,
                            value: Box::new(Ast {
                                span: 9..10,
                                kind: IntegerLiteral(1),
                            }),
                        },
                    },
                    Ast {
                        span: 12..13,
                        kind: FunctionArg {
                            name: None,
                            value: Box::new(Ast {
                                span: 12..13,
                                kind: IntegerLiteral(3),
                            }),
                        },
                    },
                ],
            },
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 16..55,
            kind: FunctionCall {
                subject: Box::new(Ast {
                    span: 16..29,
                    kind: ScopedIdentifier(vec![
                        Ast {
                            span: 16..19,
                            kind: Identifier("std"),
                        },
                        Ast {
                            span: 21..29,
                            kind: Identifier("function"),
                        },
                    ]),
                }),
                args: vec![
                    Ast {
                        span: 30..36,
                        kind: FunctionArg {
                            name: Some(Box::new(Ast {
                                span: 30..31,
                                kind: Identifier("a"),
                            })),
                            value: Box::new(Ast {
                                span: 34..36,
                                kind: IntegerLiteral(12),
                            }),
                        },
                    },
                    Ast {
                        span: 38..54,
                        kind: FunctionArg {
                            name: Some(Box::new(Ast {
                                span: 38..39,
                                kind: Identifier("b"),
                            })),
                            value: Box::new(Ast {
                                span: 42..54,
                                kind: RegexLiteral {
                                    pattern: "[^\\s]+",
                                    flags: RegexFlags::I_IGNORE_CASE | RegexFlags::M_MULTILINE,
                                },
                            }),
                        },
                    },
                ],
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 56..76,
            kind: FunctionCall {
                subject: Box::new(Ast {
                    span: 56..60,
                    kind: Variable("var"),
                }),
                args: vec![
                    Ast {
                        span: 61..62,
                        kind: FunctionArg {
                            name: None,
                            value: Box::new(Ast {
                                span: 61..62,
                                kind: IntegerLiteral(5),
                            }),
                        },
                    },
                    Ast {
                        span: 64..75,
                        kind: FunctionArg {
                            name: Some(Box::new(Ast {
                                span: 64..68,
                                kind: Identifier("test"),
                            })),
                            value: Box::new(Ast {
                                span: 71..75,
                                kind: BooleanLiteral(true),
                            }),
                        },
                    },
                ],
            },
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 77..84,
            kind: Index {
                subject: Box::new(Ast {
                    span: 77..81,
                    kind: BooleanLiteral(true),
                }),
                index: Box::new(Ast {
                    span: 82..83,
                    kind: IntegerLiteral(0),
                }),
            },
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_not_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(r#"NOT 5 ~//^\s+\d\b// !$hello not parse() (5,)(none)"#, 20);
    let result_a = parser.parse_not_op()?;
    let result_b = parser.parse_not_op()?;
    let result_c = parser.parse_not_op()?;
    let result_d = parser.parse_not_op()?;
    let result_e = parser.parse_not_op()?;

    info!(
        r#"input = {:?} | parse_not_op parse_not_op parse_not_op parse_not_op parse_not_op = {:#?} {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d, result_e,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..5,
            kind: LogicalNotOp(Box::new(Ast {
                span: 4..5,
                kind: IntegerLiteral(5),
            }))
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 6..19,
            kind: BitwiseNotOp(Box::new(Ast {
                span: 7..19,
                kind: RegexLiteral {
                    pattern: "^\\s+\\d\\b",
                    flags: RegexFlags::empty(),
                },
            }))
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 20..27,
            kind: LogicalNotOp(Box::new(Ast {
                span: 21..27,
                kind: Variable("hello"),
            }))
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 28..39,
            kind: LogicalNotOp(Box::new(Ast {
                span: 32..39,
                kind: FunctionCall {
                    subject: Box::new(Ast {
                        span: 32..37,
                        kind: Identifier("parse"),
                    }),
                    args: vec![],
                },
            }))
        })
    );

    assert_eq!(
        result_e,
        Some(Ast {
            span: 40..50,
            kind: FunctionCall {
                subject: Box::new(Ast {
                    span: 40..44,
                    kind: TupleLiteral(vec![Ast {
                        span: 41..42,
                        kind: IntegerLiteral(5),
                    }]),
                }),
                args: vec![Ast {
                    span: 45..49,
                    kind: FunctionArg {
                        name: None,
                        value: Box::new(Ast {
                            span: 45..49,
                            kind: NoneLiteral,
                        }),
                    },
                }],
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_sign_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(r#"+0x1ab3 -(.02) +!new () not {a: "b"}"#, 20);
    let result_a = parser.parse_sign_op()?;
    let result_b = parser.parse_sign_op()?;
    let result_c = parser.parse_sign_op()?;
    let result_d = parser.parse_sign_op()?;

    info!(
        r#"input = {:?} | parse_sign_op parse_sign_op parse_sign_op parse_sign_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..7,
            kind: PlusSignOp(Box::new(Ast {
                span: 1..7,
                kind: IntegerLiteral(0x1ab3),
            }))
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 8..13,
            kind: MinusSignOp(Box::new(Ast {
                span: 10..13,
                kind: FloatLiteral(0.02),
            }))
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 15..23,
            kind: PlusSignOp(Box::new(Ast {
                span: 16..23,
                kind: LogicalNotOp(Box::new(Ast {
                    span: 17..23,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 17..20,
                            kind: Identifier("new"),
                        }),
                        args: vec![],
                    },
                }))
            }))
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 24..36,
            kind: LogicalNotOp(Box::new(Ast {
                span: 28..36,
                kind: ObjectLiteral(vec![(
                    Ast {
                        span: 29..30,
                        kind: Identifier("a"),
                    },
                    Ast {
                        span: 32..35,
                        kind: StringLiteral("b"),
                    },
                )]),
            }))
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_access_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"$var.TEST?.Now person[5].names.surname -0o5_00?.max $var.test.* +0x1234"#,
        100,
    );
    let result_a = parser.parse_access_op()?;
    let result_b = parser.parse_access_op()?;
    let result_c = parser.parse_access_op()?;
    let result_d = parser.parse_access_op()?;
    let result_e = parser.parse_access_op()?;

    info!(
        r#"input = {:?} | parse_access_op parse_access_op parse_access_op parse_access_op parse_access_op = {:#?} {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d, result_e,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..14,
            kind: SafeNavigationAccessOp {
                subject: Box::new(Ast {
                    span: 0..9,
                    kind: DotAccessOp {
                        subject: Box::new(Ast {
                            span: 0..4,
                            kind: Variable("var",),
                        }),
                        field: Box::new(Ast {
                            span: 5..9,
                            kind: Identifier("TEST",),
                        }),
                    },
                }),
                field: Box::new(Ast {
                    span: 11..14,
                    kind: Identifier("Now",),
                }),
            },
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 15..38,
            kind: DotAccessOp {
                subject: Box::new(Ast {
                    span: 15..30,
                    kind: DotAccessOp {
                        subject: Box::new(Ast {
                            span: 15..24,
                            kind: Index {
                                subject: Box::new(Ast {
                                    span: 15..21,
                                    kind: Identifier("person",),
                                }),
                                index: Box::new(Ast {
                                    span: 22..23,
                                    kind: IntegerLiteral(5),
                                }),
                            },
                        }),
                        field: Box::new(Ast {
                            span: 25..30,
                            kind: Identifier("names",),
                        }),
                    },
                }),
                field: Box::new(Ast {
                    span: 31..38,
                    kind: Identifier("surname",),
                }),
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 39..51,
            kind: SafeNavigationAccessOp {
                subject: Box::new(Ast {
                    span: 39..46,
                    kind: MinusSignOp(Box::new(Ast {
                        span: 40..46,
                        kind: IntegerLiteral(0o500),
                    })),
                }),
                field: Box::new(Ast {
                    span: 48..51,
                    kind: Identifier("max",),
                }),
            },
        },)
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 52..63,
            kind: DotAccessWildcardOp {
                subject: Box::new(Ast {
                    span: 52..61,
                    kind: DotAccessOp {
                        subject: Box::new(Ast {
                            span: 52..56,
                            kind: Variable("var",),
                        }),
                        field: Box::new(Ast {
                            span: 57..61,
                            kind: Identifier("test",),
                        }),
                    },
                }),
            },
        })
    );

    assert_eq!(
        result_e,
        Some(Ast {
            span: 64..71,
            kind: PlusSignOp(Box::new(Ast {
                span: 65..71,
                kind: IntegerLiteral(0x1234),
            })),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_pow_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two ** 3 ** 0b1010 ** $var b"test".hello ** 5 parse()?.name"#,
        100,
    );
    let result_a = parser.parse_pow_op()?;
    let result_b = parser.parse_pow_op()?;
    let result_c = parser.parse_pow_op()?;

    info!(
        r#"input = {:?} | parse_pow_op parse_pow_op parse_pow_op = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..26,
            kind: ExponentiationOp(
                Box::new(Ast {
                    span: 0..3,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 7..26,
                    kind: ExponentiationOp(
                        Box::new(Ast {
                            span: 7..8,
                            kind: IntegerLiteral(3),
                        }),
                        Box::new(Ast {
                            span: 12..26,
                            kind: ExponentiationOp(
                                Box::new(Ast {
                                    span: 12..18,
                                    kind: IntegerLiteral(0b1010),
                                }),
                                Box::new(Ast {
                                    span: 22..26,
                                    kind: Variable("var"),
                                }),
                            ),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 27..45,
            kind: ExponentiationOp(
                Box::new(Ast {
                    span: 27..40,
                    kind: DotAccessOp {
                        subject: Box::new(Ast {
                            span: 27..34,
                            kind: ByteStringLiteral("test"),
                        }),
                        field: Box::new(Ast {
                            span: 35..40,
                            kind: Identifier("hello"),
                        }),
                    },
                }),
                Box::new(Ast {
                    span: 44..45,
                    kind: IntegerLiteral(5),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 46..59,
            kind: SafeNavigationAccessOp {
                subject: Box::new(Ast {
                    span: 46..53,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 46..51,
                            kind: Identifier("parse",),
                        }),
                        args: vec![],
                    },
                }),
                field: Box::new(Ast {
                    span: 55..59,
                    kind: Identifier("name",),
                }),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_mul_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two × 3 0b1010 * $var b"test".hello ** 5 * 2 ** 0x1234 100 ** `hello`"#,
        100,
    );
    let result_a = parser.parse_mul_op()?;
    let result_b = parser.parse_mul_op()?;
    let result_c = parser.parse_mul_op()?;
    let result_d = parser.parse_mul_op()?;

    info!(
        r#"input = {:?} | parse_mul_op parse_mul_op parse_mul_op parse_mul_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..8,
            kind: MultiplicationOp(
                Box::new(Ast {
                    span: 0..3,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 7..8,
                    kind: IntegerLiteral(3),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 9..22,
            kind: MultiplicationOp(
                Box::new(Ast {
                    span: 9..15,
                    kind: IntegerLiteral(0b1010),
                }),
                Box::new(Ast {
                    span: 18..22,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 23..55,
            kind: MultiplicationOp(
                Box::new(Ast {
                    span: 23..41,
                    kind: ExponentiationOp(
                        Box::new(Ast {
                            span: 23..36,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 23..30,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 31..36,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 40..41,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 44..55,
                    kind: ExponentiationOp(
                        Box::new(Ast {
                            span: 44..45,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 49..55,
                            kind: IntegerLiteral(0x1234),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 56..70,
            kind: ExponentiationOp(
                Box::new(Ast {
                    span: 56..59,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 63..70,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_add_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two + 3 0b1010 - $var b"test".hello × 5 + 2 * 0x1234 100 * `hello`"#,
        100,
    );
    let result_a = parser.parse_add_op()?;
    let result_b = parser.parse_add_op()?;
    let result_c = parser.parse_add_op()?;
    let result_d = parser.parse_add_op()?;

    info!(
        r#"input = {:?} | parse_add_op parse_add_op parse_add_op parse_add_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..7,
            kind: AdditionOp(
                Box::new(Ast {
                    span: 0..3,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 6..7,
                    kind: IntegerLiteral(3),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 8..21,
            kind: SubtractionOp(
                Box::new(Ast {
                    span: 8..14,
                    kind: IntegerLiteral(0b1010),
                }),
                Box::new(Ast {
                    span: 17..21,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 22..53,
            kind: AdditionOp(
                Box::new(Ast {
                    span: 22..40,
                    kind: MultiplicationOp(
                        Box::new(Ast {
                            span: 22..35,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 22..29,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 30..35,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 39..40,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 43..53,
                    kind: MultiplicationOp(
                        Box::new(Ast {
                            span: 43..44,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 47..53,
                            kind: IntegerLiteral(0x1234),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 54..67,
            kind: MultiplicationOp(
                Box::new(Ast {
                    span: 54..57,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 60..67,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_shift_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two << 3 0b1010 >> $var b"test".hello + 5 << 2 - 0x1234 100 + `hello`"#,
        100,
    );
    let result_a = parser.parse_shift_op()?;
    let result_b = parser.parse_shift_op()?;
    let result_c = parser.parse_shift_op()?;
    let result_d = parser.parse_shift_op()?;

    info!(
        r#"input = {:?} | parse_shift_op parse_shift_op parse_shift_op parse_shift_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..8,
            kind: LeftShiftOp(
                Box::new(Ast {
                    span: 0..3,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 7..8,
                    kind: IntegerLiteral(3),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 9..23,
            kind: RightShiftOp(
                Box::new(Ast {
                    span: 9..15,
                    kind: IntegerLiteral(0b1010),
                }),
                Box::new(Ast {
                    span: 19..23,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 24..55,
            kind: LeftShiftOp(
                Box::new(Ast {
                    span: 24..41,
                    kind: AdditionOp(
                        Box::new(Ast {
                            span: 24..37,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 24..31,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 32..37,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 40..41,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 45..55,
                    kind: SubtractionOp(
                        Box::new(Ast {
                            span: 45..46,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 49..55,
                            kind: IntegerLiteral(0x1234),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 56..69,
            kind: AdditionOp(
                Box::new(Ast {
                    span: 56..59,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 62..69,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_match_sim_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two not match 3 0b1010 <> $var b"test".hello >> 5 match 2 << 0x1234 100 >> `hello`"#,
        100,
    );
    let result_a = parser.parse_match_sim_op()?;
    let result_b = parser.parse_match_sim_op()?;
    let result_c = parser.parse_match_sim_op()?;
    let result_d = parser.parse_match_sim_op()?;

    info!(
        r#"input = {:?} | parse_match_sim_op parse_match_sim_op parse_match_sim_op parse_match_sim_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..15,
            kind: NotMatchOp(
                Box::new(Ast {
                    span: 0..3,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 14..15,
                    kind: IntegerLiteral(3),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 16..30,
            kind: SimilarityOp(
                Box::new(Ast {
                    span: 16..22,
                    kind: IntegerLiteral(0b1010),
                }),
                Box::new(Ast {
                    span: 26..30,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 31..67,
            kind: MatchOp(
                Box::new(Ast {
                    span: 31..49,
                    kind: RightShiftOp(
                        Box::new(Ast {
                            span: 31..44,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 31..38,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 39..44,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 48..49,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 56..67,
                    kind: LeftShiftOp(
                        Box::new(Ast {
                            span: 56..57,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 61..67,
                            kind: IntegerLiteral(0x1234),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 68..82,
            kind: RightShiftOp(
                Box::new(Ast {
                    span: 68..71,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 75..82,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_rel_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two < 3 two > 3 two <= 3 two >= 3 two in [1, 2, 3] two not in [1, 2, 3] two contains 1 two not contains 1 two contains none [1, 2, 3] two contains all [1, 2, 3] two contains any [1, 2, 3] two ~ 100"#,
        100,
    );
    let result_a = parser.parse_rel_op()?;
    let result_b = parser.parse_rel_op()?;
    let result_c = parser.parse_rel_op()?;
    let result_d = parser.parse_rel_op()?;
    let result_e = parser.parse_rel_op()?;
    let result_f = parser.parse_rel_op()?;
    let result_g = parser.parse_rel_op()?;
    let result_h = parser.parse_rel_op()?;
    let result_i = parser.parse_rel_op()?;
    let result_j = parser.parse_rel_op()?;
    let result_k = parser.parse_rel_op()?;
    let result_l = parser.parse_rel_op()?;

    info!(
        r#"input = {:?} | = {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string,
        result_a,
        result_b,
        result_c,
        result_d,
        result_e,
        result_f,
        result_g,
        result_h,
        result_i,
        result_j,
        result_k,
        result_l,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..7,
            kind: LessThanOp(
                Box::new(Ast {
                    span: 0..3,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 6..7,
                    kind: IntegerLiteral(3),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 8..15,
            kind: GreaterThanOp(
                Box::new(Ast {
                    span: 8..11,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 14..15,
                    kind: IntegerLiteral(3),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 16..24,
            kind: LessThanEqualToOp(
                Box::new(Ast {
                    span: 16..19,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 23..24,
                    kind: IntegerLiteral(3),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 25..33,
            kind: GreaterThanEqualToOp(
                Box::new(Ast {
                    span: 25..28,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 32..33,
                    kind: IntegerLiteral(3),
                }),
            ),
        })
    );

    assert_eq!(
        result_e,
        Some(Ast {
            span: 34..50,
            kind: InOp(
                Box::new(Ast {
                    span: 34..37,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 41..50,
                    kind: ListLiteral(vec![
                        Ast {
                            span: 42..43,
                            kind: IntegerLiteral(1),
                        },
                        Ast {
                            span: 45..46,
                            kind: IntegerLiteral(2),
                        },
                        Ast {
                            span: 48..49,
                            kind: IntegerLiteral(3),
                        },
                    ]),
                }),
            ),
        })
    );

    assert_eq!(
        result_f,
        Some(Ast {
            span: 51..71,
            kind: NotInOp(
                Box::new(Ast {
                    span: 51..54,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 62..71,
                    kind: ListLiteral(vec![
                        Ast {
                            span: 63..64,
                            kind: IntegerLiteral(1),
                        },
                        Ast {
                            span: 66..67,
                            kind: IntegerLiteral(2),
                        },
                        Ast {
                            span: 69..70,
                            kind: IntegerLiteral(3),
                        },
                    ]),
                }),
            ),
        })
    );

    assert_eq!(
        result_g,
        Some(Ast {
            span: 72..86,
            kind: ContainsAnyOp(
                Box::new(Ast {
                    span: 72..75,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 85..86,
                    kind: IntegerLiteral(1),
                }),
            ),
        })
    );

    assert_eq!(
        result_h,
        Some(Ast {
            span: 87..105,
            kind: ContainsOp(
                Box::new(Ast {
                    span: 87..90,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 104..105,
                    kind: IntegerLiteral(1),
                }),
            ),
        })
    );

    assert_eq!(
        result_i,
        Some(Ast {
            span: 106..133,
            kind: NotContainsOp(
                Box::new(Ast {
                    span: 106..109,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 124..133,
                    kind: ListLiteral(vec![
                        Ast {
                            span: 125..126,
                            kind: IntegerLiteral(1),
                        },
                        Ast {
                            span: 128..129,
                            kind: IntegerLiteral(2),
                        },
                        Ast {
                            span: 131..132,
                            kind: IntegerLiteral(3),
                        },
                    ]),
                }),
            ),
        })
    );

    assert_eq!(
        result_j,
        Some(Ast {
            span: 134..160,
            kind: ContainsNoneOp(
                Box::new(Ast {
                    span: 134..137,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 151..160,
                    kind: ListLiteral(vec![
                        Ast {
                            span: 152..153,
                            kind: IntegerLiteral(1),
                        },
                        Ast {
                            span: 155..156,
                            kind: IntegerLiteral(2),
                        },
                        Ast {
                            span: 158..159,
                            kind: IntegerLiteral(3),
                        },
                    ]),
                }),
            ),
        })
    );

    assert_eq!(
        result_k,
        Some(Ast {
            span: 161..187,
            kind: ContainsAllOp(
                Box::new(Ast {
                    span: 161..164,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 178..187,
                    kind: ListLiteral(vec![
                        Ast {
                            span: 179..180,
                            kind: IntegerLiteral(1),
                        },
                        Ast {
                            span: 182..183,
                            kind: IntegerLiteral(2),
                        },
                        Ast {
                            span: 185..186,
                            kind: IntegerLiteral(3),
                        },
                    ]),
                }),
            ),
        },)
    );

    assert_eq!(
        result_l,
        Some(Ast {
            span: 188..197,
            kind: MatchOp(
                Box::new(Ast {
                    span: 188..191,
                    kind: Identifier("two"),
                }),
                Box::new(Ast {
                    span: 194..197,
                    kind: IntegerLiteral(100),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_eq_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two is 3 0b1010 == $var b"test".hello in 5 is not 2 >= 0x1234 100 < `hello`"#,
        100,
    );
    let result_a = parser.parse_eq_op()?;
    let result_b = parser.parse_eq_op()?;
    let result_c = parser.parse_eq_op()?;
    let result_d = parser.parse_eq_op()?;

    info!(
        r#"input = {:?} | parse_eq_op parse_eq_op parse_eq_op parse_eq_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..8,
            kind: IsOp(
                Box::new(Ast {
                    span: 0..3,
                    kind: Identifier("two",),
                }),
                Box::new(Ast {
                    span: 7..8,
                    kind: IntegerLiteral(3),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 9..23,
            kind: EqualToOp(
                Box::new(Ast {
                    span: 9..15,
                    kind: IntegerLiteral(0b1010),
                }),
                Box::new(Ast {
                    span: 19..23,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 24..61,
            kind: IsNotOp(
                Box::new(Ast {
                    span: 24..42,
                    kind: InOp(
                        Box::new(Ast {
                            span: 24..37,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 24..31,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 32..37,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 41..42,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 50..61,
                    kind: GreaterThanEqualToOp(
                        Box::new(Ast {
                            span: 50..51,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 55..61,
                            kind: IntegerLiteral(0x1234),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 62..75,
            kind: LessThanOp(
                Box::new(Ast {
                    span: 62..65,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 68..75,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_bit_and_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two & 3 & 6 0b1010 & $var b"test".hello = 5 & 2 is 0x1234 100 is `hello`"#,
        100,
    );
    let result_a = parser.parse_bit_and_op()?;
    let result_b = parser.parse_bit_and_op()?;
    let result_c = parser.parse_bit_and_op()?;
    let result_d = parser.parse_bit_and_op()?;

    info!(
        r#"input = {:?} | parse_bit_and_op parse_bit_and_op parse_bit_and_op parse_bit_and_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..11,
            kind: BitwiseAndOp(
                Box::new(Ast {
                    span: 0..7,
                    kind: BitwiseAndOp(
                        Box::new(Ast {
                            span: 0..3,
                            kind: Identifier("two"),
                        }),
                        Box::new(Ast {
                            span: 6..7,
                            kind: IntegerLiteral(3),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 10..11,
                    kind: IntegerLiteral(6),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 12..25,
            kind: BitwiseAndOp(
                Box::new(Ast {
                    span: 12..18,
                    kind: IntegerLiteral(10),
                }),
                Box::new(Ast {
                    span: 21..25,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 26..57,
            kind: BitwiseAndOp(
                Box::new(Ast {
                    span: 26..43,
                    kind: IsOp(
                        Box::new(Ast {
                            span: 26..39,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 26..33,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 34..39,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 42..43,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 46..57,
                    kind: IsOp(
                        Box::new(Ast {
                            span: 46..47,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 51..57,
                            kind: IntegerLiteral(0x1234),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 58..72,
            kind: IsOp(
                Box::new(Ast {
                    span: 58..61,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 65..72,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_bit_xor_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two ^ 3 ^ 6 0b1010 ^ $var b"test".hello & 5 ^ 2 & 0x1234 100 & `hello`"#,
        100,
    );
    let result_a = parser.parse_bit_xor_op()?;
    let result_b = parser.parse_bit_xor_op()?;
    let result_c = parser.parse_bit_xor_op()?;
    let result_d = parser.parse_bit_xor_op()?;

    info!(
        r#"input = {:?} | parse_bit_xor_op parse_bit_xor_op parse_bit_xor_op parse_bit_xor_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..11,
            kind: BitwiseXorOp(
                Box::new(Ast {
                    span: 0..7,
                    kind: BitwiseXorOp(
                        Box::new(Ast {
                            span: 0..3,
                            kind: Identifier("two"),
                        }),
                        Box::new(Ast {
                            span: 6..7,
                            kind: IntegerLiteral(3),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 10..11,
                    kind: IntegerLiteral(6),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 12..25,
            kind: BitwiseXorOp(
                Box::new(Ast {
                    span: 12..18,
                    kind: IntegerLiteral(0b1010),
                }),
                Box::new(Ast {
                    span: 21..25,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 26..56,
            kind: BitwiseXorOp(
                Box::new(Ast {
                    span: 26..43,
                    kind: BitwiseAndOp(
                        Box::new(Ast {
                            span: 26..39,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 26..33,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 34..39,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 42..43,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 46..56,
                    kind: BitwiseAndOp(
                        Box::new(Ast {
                            span: 46..47,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 50..56,
                            kind: IntegerLiteral(0x1234),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 57..70,
            kind: BitwiseAndOp(
                Box::new(Ast {
                    span: 57..60,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 63..70,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_bit_or_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two | 3 | 6 0b1010 | $var b"test".hello ^ 5 | 2 ^ 0x1234 100 ^ `hello`"#,
        100,
    );
    let result_a = parser.parse_bit_or_op()?;
    let result_b = parser.parse_bit_or_op()?;
    let result_c = parser.parse_bit_or_op()?;
    let result_d = parser.parse_bit_or_op()?;

    info!(
        r#"input = {:?} | parse_bit_or_op parse_bit_or_op parse_bit_or_op parse_bit_or_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..11,
            kind: BitwiseOrOp(
                Box::new(Ast {
                    span: 0..7,
                    kind: BitwiseOrOp(
                        Box::new(Ast {
                            span: 0..3,
                            kind: Identifier("two",),
                        }),
                        Box::new(Ast {
                            span: 6..7,
                            kind: IntegerLiteral(3),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 10..11,
                    kind: IntegerLiteral(6),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 12..25,
            kind: BitwiseOrOp(
                Box::new(Ast {
                    span: 12..18,
                    kind: IntegerLiteral(10),
                }),
                Box::new(Ast {
                    span: 21..25,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 26..56,
            kind: BitwiseOrOp(
                Box::new(Ast {
                    span: 26..43,
                    kind: BitwiseXorOp(
                        Box::new(Ast {
                            span: 26..39,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 26..33,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 34..39,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 42..43,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 46..56,
                    kind: BitwiseXorOp(
                        Box::new(Ast {
                            span: 46..47,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 50..56,
                            kind: IntegerLiteral(0x1234),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 57..70,
            kind: BitwiseXorOp(
                Box::new(Ast {
                    span: 57..60,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 63..70,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_and_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two && 3 AND 6 0b1010 and $var b"test".hello | 5 && 2 | 0x1234 100 | `hello`"#,
        100,
    );
    let result_a = parser.parse_and_op()?;
    let result_b = parser.parse_and_op()?;
    let result_c = parser.parse_and_op()?;
    let result_d = parser.parse_and_op()?;

    info!(
        r#"input = {:?} | parse_and_op parse_and_op parse_and_op parse_and_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..14,
            kind: LogicalAndOp(
                Box::new(Ast {
                    span: 0..8,
                    kind: LogicalAndOp(
                        Box::new(Ast {
                            span: 0..3,
                            kind: Identifier("two",),
                        }),
                        Box::new(Ast {
                            span: 7..8,
                            kind: IntegerLiteral(3),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 13..14,
                    kind: IntegerLiteral(6),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 15..30,
            kind: LogicalAndOp(
                Box::new(Ast {
                    span: 15..21,
                    kind: IntegerLiteral(10),
                }),
                Box::new(Ast {
                    span: 26..30,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 31..62,
            kind: LogicalAndOp(
                Box::new(Ast {
                    span: 31..48,
                    kind: BitwiseOrOp(
                        Box::new(Ast {
                            span: 31..44,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 31..38,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 39..44,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 47..48,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 52..62,
                    kind: BitwiseOrOp(
                        Box::new(Ast {
                            span: 52..53,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 56..62,
                            kind: IntegerLiteral(0x1234),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 63..76,
            kind: BitwiseOrOp(
                Box::new(Ast {
                    span: 63..66,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 69..76,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_or_null_coalesce_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two || 3 OR 6 0b1010 and $var b"test".hello && 5 ?: 2 and 0x1234 100 && `hello`"#,
        100,
    );
    let result_a = parser.parse_or_null_coalesce_op()?;
    let result_b = parser.parse_or_null_coalesce_op()?;
    let result_c = parser.parse_or_null_coalesce_op()?;
    let result_d = parser.parse_or_null_coalesce_op()?;

    info!(
        r#"input = {:?} | parse_or_null_coalesce_op parse_or_null_coalesce_op parse_or_null_coalesce_op parse_or_null_coalesce_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..13,
            kind: LogicalOrOp(
                Box::new(Ast {
                    span: 0..8,
                    kind: LogicalOrOp(
                        Box::new(Ast {
                            span: 0..3,
                            kind: Identifier("two",),
                        }),
                        Box::new(Ast {
                            span: 7..8,
                            kind: IntegerLiteral(3),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 12..13,
                    kind: IntegerLiteral(6),
                }),
            ),
        }),
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 14..29,
            kind: LogicalAndOp(
                Box::new(Ast {
                    span: 14..20,
                    kind: IntegerLiteral(0b1010),
                }),
                Box::new(Ast {
                    span: 25..29,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 30..64,
            kind: NullCoalesceOp(
                Box::new(Ast {
                    span: 30..48,
                    kind: LogicalAndOp(
                        Box::new(Ast {
                            span: 30..43,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 30..37,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 38..43,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 47..48,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 52..64,
                    kind: LogicalAndOp(
                        Box::new(Ast {
                            span: 52..53,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 58..64,
                            kind: IntegerLiteral(0x1234),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 65..79,
            kind: LogicalAndOp(
                Box::new(Ast {
                    span: 65..68,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 72..79,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_range_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"two..3 0b1010..=$var b"test".hello ?: 5..2 or 0x1234 100 || `hello`"#,
        100,
    );
    let result_a = parser.parse_range_op()?;
    let result_b = parser.parse_range_op()?;
    let result_c = parser.parse_range_op()?;
    let result_d = parser.parse_range_op()?;

    info!(
        r#"input = {:?} | parse_range_op parse_range_op parse_range_op parse_range_op = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..6,
            kind: RangeOp(
                Box::new(Ast {
                    span: 0..3,
                    kind: Identifier("two",),
                }),
                Box::new(Ast {
                    span: 5..6,
                    kind: IntegerLiteral(3),
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 7..20,
            kind: RangeInclusiveOp(
                Box::new(Ast {
                    span: 7..13,
                    kind: IntegerLiteral(10),
                }),
                Box::new(Ast {
                    span: 16..20,
                    kind: Variable("var"),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 21..52,
            kind: RangeOp(
                Box::new(Ast {
                    span: 21..39,
                    kind: NullCoalesceOp(
                        Box::new(Ast {
                            span: 21..34,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 21..28,
                                    kind: ByteStringLiteral("test"),
                                }),
                                field: Box::new(Ast {
                                    span: 29..34,
                                    kind: Identifier("hello"),
                                }),
                            },
                        }),
                        Box::new(Ast {
                            span: 38..39,
                            kind: IntegerLiteral(5),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 41..52,
                    kind: LogicalOrOp(
                        Box::new(Ast {
                            span: 41..42,
                            kind: IntegerLiteral(2),
                        }),
                        Box::new(Ast {
                            span: 46..52,
                            kind: IntegerLiteral(4660),
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 53..67,
            kind: LogicalOrOp(
                Box::new(Ast {
                    span: 53..56,
                    kind: IntegerLiteral(100),
                }),
                Box::new(Ast {
                    span: 60..67,
                    kind: Identifier("hello"),
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_partial_as() -> anyhow::Result<()> {
    let parser = &mut Parser::new("as identifier AS identifier", 20);
    let result_a = parser.parse_partial_as()?;
    let result_b = parser.parse_partial_as()?;

    info!(
        r#"input = {:?} | parse_partial_as parse_partial_as = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_single_relate_id() -> anyhow::Result<()> {
    let parser = &mut Parser::new(r#"id:* name 4..=5 name as n *"#, 20);
    let result_a = parser.parse_single_relate_id()?;
    let result_b = parser.parse_single_relate_id()?;
    let result_c = parser.parse_single_relate_id()?;
    let result_d = parser.parse_single_relate_id()?;
    let result_e = parser.parse_single_relate_id()?;

    info!(
        r#"input = {:?} | parse_single_relate_id parse_single_relate_id parse_single_relate_id parse_single_relate_id parse_single_relate_id = {:#?} {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d, result_e,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..4,
            kind: SingleRelateId {
                subject: Box::new(Ast {
                    span: 0..4,
                    kind: IdOp(
                        Box::new(Ast {
                            span: 0..2,
                            kind: Identifier("id",),
                        }),
                        Box::new(Ast {
                            span: 3..4,
                            kind: Wildcard,
                        }),
                    ),
                }),
                alias: None,
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 5..9,
            kind: SingleRelateId {
                subject: Box::new(Ast {
                    span: 5..9,
                    kind: Identifier("name",),
                }),
                alias: None,
            },
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 10..15,
            kind: SingleRelateId {
                subject: Box::new(Ast {
                    span: 10..15,
                    kind: RangeInclusiveOp(
                        Box::new(Ast {
                            span: 10..11,
                            kind: IntegerLiteral(4,),
                        }),
                        Box::new(Ast {
                            span: 14..15,
                            kind: IntegerLiteral(5,),
                        }),
                    ),
                }),
                alias: None,
            },
        },)
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 16..25,
            kind: SingleRelateId {
                subject: Box::new(Ast {
                    span: 16..20,
                    kind: Identifier("name",),
                }),
                alias: Some(Box::new(Ast {
                    span: 24..25,
                    kind: Identifier("n",),
                })),
            },
        },)
    );

    assert_eq!(
        result_e,
        Some(Ast {
            span: 26..27,
            kind: Wildcard,
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_multi_relate_id() -> anyhow::Result<()> {
    let parser = &mut Parser::new(r#"[id:*, name] [4..=5, *,]"#, 20);
    let result_a = parser.parse_multi_relate_id()?;
    let result_b = parser.parse_multi_relate_id()?;

    info!(
        r#"input = {:?} | parse_multi_relate_id parse_multi_relate_id = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..12,
            kind: ListLiteral(vec![
                Ast {
                    span: 1..5,
                    kind: SingleRelateId {
                        subject: Box::new(Ast {
                            span: 1..5,
                            kind: IdOp(
                                Box::new(Ast {
                                    span: 1..3,
                                    kind: Identifier("id",),
                                }),
                                Box::new(Ast {
                                    span: 4..5,
                                    kind: Wildcard,
                                }),
                            ),
                        }),
                        alias: None,
                    },
                },
                Ast {
                    span: 7..11,
                    kind: SingleRelateId {
                        subject: Box::new(Ast {
                            span: 7..11,
                            kind: Identifier("name",),
                        }),
                        alias: None,
                    },
                },
            ]),
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 13..24,
            kind: ListLiteral(vec![
                Ast {
                    span: 14..19,
                    kind: SingleRelateId {
                        subject: Box::new(Ast {
                            span: 14..19,
                            kind: RangeInclusiveOp(
                                Box::new(Ast {
                                    span: 14..15,
                                    kind: IntegerLiteral(4,),
                                }),
                                Box::new(Ast {
                                    span: 18..19,
                                    kind: IntegerLiteral(5,),
                                }),
                            ),
                        }),
                        alias: None,
                    },
                },
                Ast {
                    span: 21..22,
                    kind: Wildcard,
                },
            ],),
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_relate_id() -> anyhow::Result<()> {
    let parser = &mut Parser::new(r#"[id:*, name,] 4..=5 *"#, 20);
    let result_a = parser.parse_relate_id()?;
    let result_b = parser.parse_relate_id()?;
    let result_c = parser.parse_relate_id()?;

    info!(
        r#"input = {:?} | parse_relate_id parse_relate_id parse_relate_id = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..13,
            kind: ListLiteral(vec![
                Ast {
                    span: 1..5,
                    kind: SingleRelateId {
                        subject: Box::new(Ast {
                            span: 1..5,
                            kind: IdOp(
                                Box::new(Ast {
                                    span: 1..3,
                                    kind: Identifier("id",),
                                }),
                                Box::new(Ast {
                                    span: 4..5,
                                    kind: Wildcard,
                                }),
                            ),
                        }),
                        alias: None,
                    },
                },
                Ast {
                    span: 7..11,
                    kind: SingleRelateId {
                        subject: Box::new(Ast {
                            span: 7..11,
                            kind: Identifier("name",),
                        }),
                        alias: None,
                    },
                },
            ],),
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 14..19,
            kind: SingleRelateId {
                subject: Box::new(Ast {
                    span: 14..19,
                    kind: RangeInclusiveOp(
                        Box::new(Ast {
                            span: 14..15,
                            kind: IntegerLiteral(4,),
                        }),
                        Box::new(Ast {
                            span: 18..19,
                            kind: IntegerLiteral(5,),
                        }),
                    ),
                }),
                alias: None,
            },
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 20..21,
            kind: Wildcard,
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_relate_edge_id() -> anyhow::Result<()> {
    let parser = &mut Parser::new(r#"friend[5] friend likes as l"#, 20);
    let result_a = parser.parse_relate_edge_id()?;
    let result_b = parser.parse_relate_edge_id()?;
    let result_c = parser.parse_relate_edge_id()?;

    info!(
        r#"input = {:?} | parse_relate_edge_id parse_relate_edge_id parse_relate_edge_id = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..9,
            kind: RelateEdgeId {
                subject: Box::new(Ast {
                    span: 0..6,
                    kind: Identifier("friend"),
                }),
                depth: Some(Box::new(Ast {
                    span: 7..8,
                    kind: IntegerLiteral(5),
                })),
                alias: None,
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 10..16,
            kind: RelateEdgeId {
                subject: Box::new(Ast {
                    span: 10..16,
                    kind: Identifier("friend"),
                }),
                depth: None,
                alias: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 17..27,
            kind: RelateEdgeId {
                subject: Box::new(Ast {
                    span: 17..22,
                    kind: Identifier("likes",),
                }),
                depth: None,
                alias: Some(Box::new(Ast {
                    span: 26..27,
                    kind: Identifier("l",),
                })),
            },
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_relate_edge_not_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(r#"NOT friend !likes not`drives`*"#, 20);
    let result_a = parser.parse_relate_edge_not_op()?;
    let result_b = parser.parse_relate_edge_not_op()?;
    let result_c = parser.parse_relate_edge_not_op()?;

    info!(
        r#"input = {:?} | parse_relate_edge_not_op parse_relate_edge_not_op parse_relate_edge_not_op = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 4..10,
            kind: LogicalNotOp(Box::new(Ast {
                span: 4..10,
                kind: RelateEdgeId {
                    subject: Box::new(Ast {
                        span: 4..10,
                        kind: Identifier("friend",),
                    }),
                    depth: None,
                    alias: None,
                },
            })),
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 12..17,
            kind: LogicalNotOp(Box::new(Ast {
                span: 12..17,
                kind: RelateEdgeId {
                    subject: Box::new(Ast {
                        span: 12..17,
                        kind: Identifier("likes",),
                    }),
                    depth: None,
                    alias: None,
                },
            })),
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 21..29,
            kind: LogicalNotOp(Box::new(Ast {
                span: 21..29,
                kind: RelateEdgeId {
                    subject: Box::new(Ast {
                        span: 21..29,
                        kind: Identifier("drives",),
                    }),
                    depth: None,
                    alias: None,
                },
            })),
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_relate_edge_and_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"friend AND sibling and family has && owns NOT likes"#,
        100,
    );
    let result_a = parser.parse_relate_edge_and_op()?;
    let result_b = parser.parse_relate_edge_and_op()?;
    let result_c = parser.parse_relate_edge_and_op()?;

    info!(
        r#"input = {:?} | parse_relate_edge_and_op parse_relate_edge_and_op parse_relate_edge_and_op = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..29,
            kind: LogicalAndOp(
                Box::new(Ast {
                    span: 0..18,
                    kind: LogicalAndOp(
                        Box::new(Ast {
                            span: 0..6,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 0..6,
                                    kind: Identifier("friend"),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                        Box::new(Ast {
                            span: 11..18,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 11..18,
                                    kind: Identifier("sibling"),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 23..29,
                    kind: RelateEdgeId {
                        subject: Box::new(Ast {
                            span: 23..29,
                            kind: Identifier("family",),
                        }),
                        depth: None,
                        alias: None,
                    },
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 30..41,
            kind: LogicalAndOp(
                Box::new(Ast {
                    span: 30..33,
                    kind: RelateEdgeId {
                        subject: Box::new(Ast {
                            span: 30..33,
                            kind: Identifier("has",),
                        }),
                        depth: None,
                        alias: None,
                    },
                }),
                Box::new(Ast {
                    span: 37..41,
                    kind: RelateEdgeId {
                        subject: Box::new(Ast {
                            span: 37..41,
                            kind: Identifier("owns",),
                        }),
                        depth: None,
                        alias: None,
                    },
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 46..51,
            kind: LogicalNotOp(Box::new(Ast {
                span: 46..51,
                kind: RelateEdgeId {
                    subject: Box::new(Ast {
                        span: 46..51,
                        kind: Identifier("likes",),
                    }),
                    depth: None,
                    alias: None,
                },
            })),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_relate_edge_or_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"friend OR sibling or family has and owns || needs AND have plays AND toys"#,
        100,
    );
    let result_a = parser.parse_relate_edge_or_op()?;
    let result_b = parser.parse_relate_edge_or_op()?;
    let result_c = parser.parse_relate_edge_or_op()?;

    info!(
        r#"input = {:?} | parse_relate_edge_or_op parse_relate_edge_or_op parse_relate_edge_or_op = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..27,
            kind: LogicalOrOp(
                Box::new(Ast {
                    span: 0..17,
                    kind: LogicalOrOp(
                        Box::new(Ast {
                            span: 0..6,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 0..6,
                                    kind: Identifier("friend",),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                        Box::new(Ast {
                            span: 10..17,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 10..17,
                                    kind: Identifier("sibling",),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 21..27,
                    kind: RelateEdgeId {
                        subject: Box::new(Ast {
                            span: 21..27,
                            kind: Identifier("family",),
                        }),
                        depth: None,
                        alias: None,
                    },
                }),
            ),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 28..58,
            kind: LogicalOrOp(
                Box::new(Ast {
                    span: 28..40,
                    kind: LogicalAndOp(
                        Box::new(Ast {
                            span: 28..31,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 28..31,
                                    kind: Identifier("has",),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                        Box::new(Ast {
                            span: 36..40,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 36..40,
                                    kind: Identifier("owns",),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 44..58,
                    kind: LogicalAndOp(
                        Box::new(Ast {
                            span: 44..49,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 44..49,
                                    kind: Identifier("needs",),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                        Box::new(Ast {
                            span: 54..58,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 54..58,
                                    kind: Identifier("have",),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                    ),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 59..73,
            kind: LogicalAndOp(
                Box::new(Ast {
                    span: 59..64,
                    kind: RelateEdgeId {
                        subject: Box::new(Ast {
                            span: 59..64,
                            kind: Identifier("plays",),
                        }),
                        depth: None,
                        alias: None,
                    },
                }),
                Box::new(Ast {
                    span: 69..73,
                    kind: RelateEdgeId {
                        subject: Box::new(Ast {
                            span: 69..73,
                            kind: Identifier("toys",),
                        }),
                        depth: None,
                        alias: None,
                    },
                }),
            ),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_relate_op() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"[name,] <- test[5..8] <<- [*] -> not likes or hates as h ->> person:john"#,
        20,
    );
    let result_a = parser.parse_relate_op()?;

    info!(
        r#"input = {:?} | parse_relate_op = {:#?}"#,
        parser.lexer.string, result_a,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..72,
            kind: RelateOp {
                left: Box::new(Ast {
                    span: 0..29,
                    kind: RelateOp {
                        left: Box::new(Ast {
                            span: 0..7,
                            kind: ListLiteral(vec![Ast {
                                span: 1..5,
                                kind: SingleRelateId {
                                    subject: Box::new(Ast {
                                        span: 1..5,
                                        kind: Identifier("name",),
                                    }),
                                    alias: None,
                                },
                            }]),
                        }),
                        l_op: Left,
                        edge: Box::new(Ast {
                            span: 11..21,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 11..15,
                                    kind: Identifier("test",),
                                }),
                                depth: Some(Box::new(Ast {
                                    span: 16..20,
                                    kind: RangeOp(
                                        Box::new(Ast {
                                            span: 16..17,
                                            kind: IntegerLiteral(5,),
                                        }),
                                        Box::new(Ast {
                                            span: 19..20,
                                            kind: IntegerLiteral(8,),
                                        }),
                                    ),
                                })),
                                alias: None,
                            },
                        }),
                        r_op: MultiLeft,
                        right: Box::new(Ast {
                            span: 26..29,
                            kind: ListLiteral(vec![Ast {
                                span: 27..28,
                                kind: Wildcard,
                            }]),
                        }),
                    },
                }),
                l_op: Right,
                edge: Box::new(Ast {
                    span: 37..56,
                    kind: LogicalOrOp(
                        Box::new(Ast {
                            span: 37..42,
                            kind: LogicalNotOp(Box::new(Ast {
                                span: 37..42,
                                kind: RelateEdgeId {
                                    subject: Box::new(Ast {
                                        span: 37..42,
                                        kind: Identifier("likes",),
                                    }),
                                    depth: None,
                                    alias: None,
                                },
                            })),
                        }),
                        Box::new(Ast {
                            span: 46..56,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 46..51,
                                    kind: Identifier("hates",),
                                }),
                                depth: None,
                                alias: Some(Box::new(Ast {
                                    span: 55..56,
                                    kind: Identifier("h",),
                                })),
                            },
                        }),
                    ),
                }),
                r_op: MultiRight,
                right: Box::new(Ast {
                    span: 61..72,
                    kind: SingleRelateId {
                        subject: Box::new(Ast {
                            span: 61..72,
                            kind: IdOp(
                                Box::new(Ast {
                                    span: 61..67,
                                    kind: Identifier("person",),
                                }),
                                Box::new(Ast {
                                    span: 68..72,
                                    kind: Identifier("john",),
                                }),
                            ),
                        }),
                        alias: None,
                    },
                }),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_operation() -> anyhow::Result<()> {
    let parser = &mut Parser::new(r#"* -> likes <- person:john 1..=2.0e-1 5 + 20 as sum"#, 20);
    let result_a = parser.parse_op()?;
    let result_b = parser.parse_op()?;
    let result_c = parser.parse_op()?;

    info!(
        r#"input = {:?} | parse_op parse_op parse_op = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..25,
            kind: RelateOp {
                left: Box::new(Ast {
                    span: 0..1,
                    kind: Wildcard,
                }),
                l_op: Right,
                edge: Box::new(Ast {
                    span: 5..10,
                    kind: RelateEdgeId {
                        subject: Box::new(Ast {
                            span: 5..10,
                            kind: Identifier("likes",),
                        }),
                        depth: None,
                        alias: None,
                    },
                }),
                r_op: Left,
                right: Box::new(Ast {
                    span: 14..25,
                    kind: SingleRelateId {
                        subject: Box::new(Ast {
                            span: 14..25,
                            kind: IdOp(
                                Box::new(Ast {
                                    span: 14..20,
                                    kind: Identifier("person",),
                                }),
                                Box::new(Ast {
                                    span: 21..25,
                                    kind: Identifier("john",),
                                }),
                            ),
                        }),
                        alias: None,
                    },
                }),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 26..36,
            kind: RangeInclusiveOp(
                Box::new(Ast {
                    span: 26..27,
                    kind: IntegerLiteral(1,),
                }),
                Box::new(Ast {
                    span: 30..36,
                    kind: FloatLiteral(0.2,),
                }),
            ),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 37..50,
            kind: AliasOp {
                subject: Box::new(Ast {
                    span: 37..43,
                    kind: AdditionOp(
                        Box::new(Ast {
                            span: 37..38,
                            kind: IntegerLiteral(5,),
                        }),
                        Box::new(Ast {
                            span: 41..43,
                            kind: IntegerLiteral(20,),
                        }),
                    ),
                }),
                alias: Box::new(Ast {
                    span: 47..50,
                    kind: Identifier("sum",),
                }),
            },
        },)
    );

    Ok(())
}
