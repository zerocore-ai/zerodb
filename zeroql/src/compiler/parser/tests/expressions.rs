use tracing::info;

use crate::{
    ast::{Ast, AstKind::*},
    parser::Parser,
};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test]
fn test_parser_partial_set_object() -> anyhow::Result<()> {
    let parser = &mut Parser::new("set {} SET { a: person[5], }", 20);
    let result_a = parser.parse_partial_set_object()?;
    let result_b = parser.parse_partial_set_object()?;

    info!(
        r#"input = {:?} | parse_partial_set_object parse_partial_set_object = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_set_assign() -> anyhow::Result<()> {
    let parser = &mut Parser::new("SET a = 10, b = 'Hello' set name = 'John'", 20);
    let result_a = parser.parse_partial_set_assign()?;
    let result_b = parser.parse_partial_set_assign()?;

    info!(
        r#"input = {:?} | parse_partial_set_assign = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_create() -> anyhow::Result<()> {
    let parser = &mut Parser::new("create person set { a: 10, b: [1, 2] } CREATE person:john SET a = 10, b = [1, 2]  CREATE person SET (a, b,) VALUES (1, 2), (3, 4)", 20);
    let result_a = parser.parse_create_exp()?;
    let result_b = parser.parse_create_exp()?;
    let result_c = parser.parse_create_exp()?;

    info!(
        r#"input = {:?} | parse_create_exp parse_create_exp parse_create_exp = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..38,
            kind: Create {
                subject: Box::new(Ast {
                    span: 7..13,
                    kind: Identifier("person",),
                }),
                columns: vec![
                    Ast {
                        span: 20..21,
                        kind: Identifier("a",),
                    },
                    Ast {
                        span: 27..28,
                        kind: Identifier("b",),
                    },
                ],
                values: vec![vec![
                    Ast {
                        span: 23..25,
                        kind: IntegerLiteral(10,),
                    },
                    Ast {
                        span: 30..36,
                        kind: ListLiteral(vec![
                            Ast {
                                span: 31..32,
                                kind: IntegerLiteral(1,),
                            },
                            Ast {
                                span: 34..35,
                                kind: IntegerLiteral(2,),
                            },
                        ]),
                    },
                ]],
            },
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 39..80,
            kind: Create {
                subject: Box::new(Ast {
                    span: 46..57,
                    kind: IdOp(
                        Box::new(Ast {
                            span: 46..52,
                            kind: Identifier("person",),
                        }),
                        Box::new(Ast {
                            span: 53..57,
                            kind: Identifier("john",),
                        }),
                    ),
                }),
                columns: vec![
                    Ast {
                        span: 62..63,
                        kind: Identifier("a",),
                    },
                    Ast {
                        span: 70..71,
                        kind: Identifier("b",),
                    },
                ],
                values: vec![vec![
                    Ast {
                        span: 66..68,
                        kind: IntegerLiteral(10,),
                    },
                    Ast {
                        span: 74..80,
                        kind: ListLiteral(vec![
                            Ast {
                                span: 75..76,
                                kind: IntegerLiteral(1,),
                            },
                            Ast {
                                span: 78..79,
                                kind: IntegerLiteral(2,),
                            },
                        ]),
                    },
                ]],
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 82..129,
            kind: Create {
                subject: Box::new(Ast {
                    span: 89..95,
                    kind: Identifier("person",),
                }),
                columns: vec![
                    Ast {
                        span: 101..102,
                        kind: Identifier("a",),
                    },
                    Ast {
                        span: 104..105,
                        kind: Identifier("b",),
                    },
                ],
                values: vec![
                    vec![
                        Ast {
                            span: 116..117,
                            kind: IntegerLiteral(1,),
                        },
                        Ast {
                            span: 119..120,
                            kind: IntegerLiteral(2,),
                        },
                    ],
                    vec![
                        Ast {
                            span: 124..125,
                            kind: IntegerLiteral(3,),
                        },
                        Ast {
                            span: 127..128,
                            kind: IntegerLiteral(4,),
                        },
                    ],
                ],
            },
        })
    );

    Ok(())
}

// #[test_log::test]
// fn test_parser_set_object() -> anyhow::Result<()> {
//     let parser = &mut Parser::new(
//         "relate p:john -> likes -> p:alice SET a = 10, b += 'Hello'",
//         20,
//     );
//     let result_a = parser.parse_relate_exp()?;
//     let result_b = parser.parse_relate_exp()?;

//     // info!(
//     //     r#"input = {:?} | parse_set_object parse_set_object = {:#?} {:#?}"#,
//     //     parser.lexer.string, result_a, result_b,
//     // );

//     // assert_eq!(result_a, None);

//     // assert_eq!(result_b, None);

//     Ok(())
// }

#[test_log::test]
fn test_parser_partial_op_update_assign() -> anyhow::Result<()> {
    let parser = &mut Parser::new("= += -= *= /= %= **= &= |= ^= ~= <<= >>= ??=", 20);
    let result_a = parser.parse_partial_op_update_assign()?;
    let result_b = parser.parse_partial_op_update_assign()?;
    let result_c = parser.parse_partial_op_update_assign()?;
    let result_d = parser.parse_partial_op_update_assign()?;
    let result_e = parser.parse_partial_op_update_assign()?;
    let result_f = parser.parse_partial_op_update_assign()?;
    let result_g = parser.parse_partial_op_update_assign()?;
    let result_h = parser.parse_partial_op_update_assign()?;
    let result_i = parser.parse_partial_op_update_assign()?;
    let result_j = parser.parse_partial_op_update_assign()?;
    let result_k = parser.parse_partial_op_update_assign()?;
    let result_l = parser.parse_partial_op_update_assign()?;
    let result_m = parser.parse_partial_op_update_assign()?;
    let result_n = parser.parse_partial_op_update_assign()?;

    info!(
        r#"input = {:?} | = {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?}"#,
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
        result_m,
        result_n,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());
    assert!(result_c.is_some());
    assert!(result_d.is_some());
    assert!(result_e.is_some());
    assert!(result_f.is_some());
    assert!(result_g.is_some());
    assert!(result_h.is_some());
    assert!(result_i.is_some());
    assert!(result_j.is_some());
    assert!(result_k.is_some());
    assert!(result_l.is_some());
    assert!(result_m.is_some());
    assert!(result_n.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_set_update_assign() -> anyhow::Result<()> {
    let parser = &mut Parser::new("SET a = 10, b += 'Hello'", 20);
    let result_a = parser.parse_partial_set_update_assign()?;

    info!(
        r#"input = {:?} | parse_partial_set_update_assign = {:#?}"#,
        parser.lexer.string, result_a,
    );

    assert!(result_a.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_where_guard() -> anyhow::Result<()> {
    let parser = &mut Parser::new("where a = 10 WHERE b = 20", 20);
    let result_a = parser.parse_partial_where_guard()?;
    let result_b = parser.parse_partial_where_guard()?;

    info!(
        r#"input = {:?} | parse_partial_where_guard parse_partial_where_guard = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

// #[test_log::test]
// fn test_parser_set_object() -> anyhow::Result<()> {
//     let parser = &mut Parser::new("set {} SET { a: person[5], }", 20);
//     let result_a = parser.parse_set_object()?;
//     let result_b = parser.parse_set_object()?;

//     info!(
//         r#"input = {:?} | parse_set_object parse_set_object = {:#?} {:#?}"#,
//         parser.lexer.string, result_a, result_b,
//     );

//     assert_eq!(result_a, None);

//     assert_eq!(result_b, None);

//     Ok(())
// }

// #[test_log::test]
// fn test_parser_set_object() -> anyhow::Result<()> {
//     let parser = &mut Parser::new("set {} SET { a: person[5], }", 20);
//     let result_a = parser.parse_set_object()?;
//     let result_b = parser.parse_set_object()?;

//     info!(
//         r#"input = {:?} | parse_set_object parse_set_object = {:#?} {:#?}"#,
//         parser.lexer.string, result_a, result_b,
//     );

//     assert_eq!(result_a, None);

//     assert_eq!(result_b, None);

//     Ok(())
// }
