use tracing::info;

use crate::{
    ast::{Ast, AstKind::*, UpdateAssign::*},
    parser::Parser,
};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test]
fn test_parser_continuation_brackets() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"{ a: 10
        , b: 20
        }

        [1, 2
        , 3]



        (1
        ,)
        "#,
        20,
    );

    let result_a = parser.parse_object_lit()?;
    let result_b = parser.parse_terminator()?;
    let result_c = parser.parse_list_lit()?;
    let result_d = parser.parse_terminator()?;
    let result_e = parser.parse_tuple_lit()?;

    info!(
        r#"input = {:?} | parse_object_lit parse_terminator parse_list_lit parse_terminator parse_tuple_lit = {:#?} {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d, result_e,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..33,
            kind: ObjectLiteral(vec![
                (
                    Ast {
                        span: 2..3,
                        kind: Identifier("a"),
                    },
                    Ast {
                        span: 5..7,
                        kind: IntegerLiteral(10),
                    },
                ),
                (
                    Ast {
                        span: 18..19,
                        kind: Identifier("b"),
                    },
                    Ast {
                        span: 21..23,
                        kind: IntegerLiteral(20),
                    },
                ),
            ]),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 33..43,
            kind: Temp(None),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 43..61,
            kind: ListLiteral(vec![
                Ast {
                    span: 44..45,
                    kind: IntegerLiteral(1,),
                },
                Ast {
                    span: 47..48,
                    kind: IntegerLiteral(2,),
                },
                Ast {
                    span: 59..60,
                    kind: IntegerLiteral(3,),
                },
            ]),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 61..73,
            kind: Temp(None),
        })
    );

    assert_eq!(
        result_e,
        Some(Ast {
            span: 73..86,
            kind: TupleLiteral(vec![Ast {
                span: 74..75,
                kind: IntegerLiteral(1),
            }]),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_continuation_comma_assignment() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"CREATE person SET name = "John Doe",
        age = 30

        LET $variable =
        [1, 2, 3]

        UPDATE person SET age +=
        0x01
        "#,
        20,
    );

    let result_a = parser.parse_create_exp()?;
    let result_b = parser.parse_terminator()?;
    let result_c = parser.parse_let_exp()?;
    let result_d = parser.parse_terminator()?;
    let result_e = parser.parse_update_exp()?;

    info!(
        r#"input = {:?} | parse_create_exp parse_terminator parse_let_exp parse_terminator parse_update_exp = {:#?} {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d, result_e,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..53,
            kind: Create {
                subject: Box::new(Ast {
                    span: 7..13,
                    kind: Identifier("person"),
                }),
                columns: vec![
                    Ast {
                        span: 18..22,
                        kind: Identifier("name"),
                    },
                    Ast {
                        span: 45..48,
                        kind: Identifier("age",),
                    },
                ],
                values: vec![vec![
                    Ast {
                        span: 25..35,
                        kind: StringLiteral("John Doe"),
                    },
                    Ast {
                        span: 51..53,
                        kind: IntegerLiteral(30),
                    },
                ]],
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 53..63,
            kind: Temp(None),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 63..96,
            kind: Let {
                name: Box::new(Ast {
                    span: 67..76,
                    kind: Variable("variable"),
                }),
                r#type: None,
                value: Box::new(Ast {
                    span: 87..96,
                    kind: ListLiteral(vec![
                        Ast {
                            span: 88..89,
                            kind: IntegerLiteral(1),
                        },
                        Ast {
                            span: 91..92,
                            kind: IntegerLiteral(2),
                        },
                        Ast {
                            span: 94..95,
                            kind: IntegerLiteral(3),
                        },
                    ]),
                }),
            },
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 96..106,
            kind: Temp(None),
        })
    );

    assert_eq!(
        result_e,
        Some(Ast {
            span: 106..143,
            kind: Update {
                target: Box::new(Ast {
                    span: 113..119,
                    kind: Identifier("person"),
                }),
                where_guard: None,
                column_ops: vec![(
                    Ast {
                        span: 124..127,
                        kind: Identifier("age"),
                    },
                    Plus,
                    Ast {
                        span: 139..143,
                        kind: IntegerLiteral(0x01),
                    },
                )],
            },
        })
    );

    Ok(())
}
