use tracing::info;

use crate::{
    ast::{Ast, AstKind::*, RelateArrow::*, UpdateAssign::*},
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
fn test_parser_create_exp() -> anyhow::Result<()> {
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

#[test_log::test]
fn test_parser_relate_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"relate p:john -> likes -> p:alice SET a = 10, b = 'Hello'\
        RELATE a -> to -> b\
        relate x -> y -> z SET { a: 10,
        b: 'Hello' }"#,
        20,
    );
    let result_a = parser.parse_relate_exp()?;
    let result_b = parser.parse_relate_exp()?;
    let result_c = parser.parse_relate_exp()?;

    info!(
        r#"input = {:?} | parse_relate_exp parse_relate_exp parse_relate_exp = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..57,
            kind: Relate {
                relate_op: Box::new(Ast {
                    span: 7..33,
                    kind: RelateOp {
                        left: Box::new(Ast {
                            span: 7..13,
                            kind: SingleRelateId {
                                subject: Box::new(Ast {
                                    span: 7..13,
                                    kind: IdOp(
                                        Box::new(Ast {
                                            span: 7..8,
                                            kind: Identifier("p",),
                                        }),
                                        Box::new(Ast {
                                            span: 9..13,
                                            kind: Identifier("john",),
                                        }),
                                    ),
                                }),
                                alias: None,
                            },
                        }),
                        l_op: Right,
                        edge: Box::new(Ast {
                            span: 17..22,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 17..22,
                                    kind: Identifier("likes",),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                        r_op: Right,
                        right: Box::new(Ast {
                            span: 26..33,
                            kind: SingleRelateId {
                                subject: Box::new(Ast {
                                    span: 26..33,
                                    kind: IdOp(
                                        Box::new(Ast {
                                            span: 26..27,
                                            kind: Identifier("p",),
                                        }),
                                        Box::new(Ast {
                                            span: 28..33,
                                            kind: Identifier("alice",),
                                        }),
                                    ),
                                }),
                                alias: None,
                            },
                        }),
                    },
                }),
                columns: vec![
                    Ast {
                        span: 38..39,
                        kind: Identifier("a",),
                    },
                    Ast {
                        span: 46..47,
                        kind: Identifier("b",),
                    },
                ],
                value: vec![
                    Ast {
                        span: 42..44,
                        kind: IntegerLiteral(10,),
                    },
                    Ast {
                        span: 50..57,
                        kind: StringLiteral("Hello",),
                    },
                ],
            },
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 67..86,
            kind: Relate {
                relate_op: Box::new(Ast {
                    span: 74..86,
                    kind: RelateOp {
                        left: Box::new(Ast {
                            span: 74..75,
                            kind: SingleRelateId {
                                subject: Box::new(Ast {
                                    span: 74..75,
                                    kind: Identifier("a",),
                                }),
                                alias: None,
                            },
                        }),
                        l_op: Right,
                        edge: Box::new(Ast {
                            span: 79..81,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 79..81,
                                    kind: Identifier("to",),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                        r_op: Right,
                        right: Box::new(Ast {
                            span: 85..86,
                            kind: SingleRelateId {
                                subject: Box::new(Ast {
                                    span: 85..86,
                                    kind: Identifier("b",),
                                }),
                                alias: None,
                            },
                        }),
                    },
                }),
                columns: vec![],
                value: vec![],
            },
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 96..148,
            kind: Relate {
                relate_op: Box::new(Ast {
                    span: 103..114,
                    kind: RelateOp {
                        left: Box::new(Ast {
                            span: 103..104,
                            kind: SingleRelateId {
                                subject: Box::new(Ast {
                                    span: 103..104,
                                    kind: Identifier("x",),
                                }),
                                alias: None,
                            },
                        }),
                        l_op: Right,
                        edge: Box::new(Ast {
                            span: 108..109,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 108..109,
                                    kind: Identifier("y",),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                        r_op: Right,
                        right: Box::new(Ast {
                            span: 113..114,
                            kind: SingleRelateId {
                                subject: Box::new(Ast {
                                    span: 113..114,
                                    kind: Identifier("z",),
                                }),
                                alias: None,
                            },
                        }),
                    },
                }),
                columns: vec![
                    Ast {
                        span: 121..122,
                        kind: Identifier("a",),
                    },
                    Ast {
                        span: 136..137,
                        kind: Identifier("b",),
                    },
                ],
                value: vec![
                    Ast {
                        span: 124..126,
                        kind: IntegerLiteral(10,),
                    },
                    Ast {
                        span: 139..146,
                        kind: StringLiteral("Hello",),
                    },
                ],
            },
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_partial_target() -> anyhow::Result<()> {
    let parser = &mut Parser::new("identifier id:op relate -> op <- relate", 20);
    let result_a = parser.parse_partial_target()?;
    let result_b = parser.parse_partial_target()?;
    let result_c = parser.parse_partial_target()?;

    info!(
        r#"input = {:?} | parse_partial_target parse_partial_target parse_partial_target = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());
    assert!(result_c.is_some());

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

#[test_log::test]
fn test_parser_delete_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "delete a WHERE a = 10 DELETE person ->> likes ->> animal DELETE person:john",
        20,
    );
    let result_a = parser.parse_delete_exp()?;
    let result_b = parser.parse_delete_exp()?;
    let result_c = parser.parse_delete_exp()?;

    info!(
        r#"input = {:?} | parse_set_object parse_set_object = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..21,
            kind: Delete {
                target: Box::new(Ast {
                    span: 7..8,
                    kind: Identifier("a",),
                }),
                where_guard: Some(Box::new(Ast {
                    span: 15..21,
                    kind: IsOp(
                        Box::new(Ast {
                            span: 15..16,
                            kind: Identifier("a",),
                        }),
                        Box::new(Ast {
                            span: 19..21,
                            kind: IntegerLiteral(10,),
                        }),
                    ),
                })),
            },
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 22..56,
            kind: Delete {
                target: Box::new(Ast {
                    span: 29..56,
                    kind: RelateOp {
                        left: Box::new(Ast {
                            span: 29..35,
                            kind: SingleRelateId {
                                subject: Box::new(Ast {
                                    span: 29..35,
                                    kind: Identifier("person",),
                                }),
                                alias: None,
                            },
                        }),
                        l_op: MultiRight,
                        edge: Box::new(Ast {
                            span: 40..45,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 40..45,
                                    kind: Identifier("likes",),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                        r_op: MultiRight,
                        right: Box::new(Ast {
                            span: 50..56,
                            kind: SingleRelateId {
                                subject: Box::new(Ast {
                                    span: 50..56,
                                    kind: Identifier("animal",),
                                }),
                                alias: None,
                            },
                        }),
                    },
                }),
                where_guard: None,
            },
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 57..75,
            kind: Delete {
                target: Box::new(Ast {
                    span: 64..75,
                    kind: IdOp(
                        Box::new(Ast {
                            span: 64..70,
                            kind: Identifier("person",),
                        }),
                        Box::new(Ast {
                            span: 71..75,
                            kind: Identifier("john",),
                        }),
                    ),
                }),
                where_guard: None,
            },
        },)
    );

    Ok(())
}

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
fn test_parser_update_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"UPDATE person SET a -= 10, b = 'Hello' WHERE a = 10\
        update person:john SET a -= 10, b = 'Hello'\
        update test SET {a: 10, b: 'Hello'} WHERE a ~ 10"#,
        20,
    );
    let result_a = parser.parse_update_exp()?;
    let result_b = parser.parse_update_exp()?;
    let result_c = parser.parse_update_exp()?;

    info!(
        r#"input = {:?} | parse_update_exp parse_update_exp parse_update_exp = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..51,
            kind: Update {
                target: Box::new(Ast {
                    span: 7..13,
                    kind: Identifier("person"),
                }),
                where_guard: Some(Box::new(Ast {
                    span: 45..51,
                    kind: IsOp(
                        Box::new(Ast {
                            span: 45..46,
                            kind: Identifier("a"),
                        }),
                        Box::new(Ast {
                            span: 49..51,
                            kind: IntegerLiteral(10),
                        }),
                    ),
                })),
                column_ops: vec![
                    (
                        Ast {
                            span: 18..19,
                            kind: Identifier("a"),
                        },
                        Minus,
                        Ast {
                            span: 23..25,
                            kind: IntegerLiteral(10),
                        },
                    ),
                    (
                        Ast {
                            span: 27..28,
                            kind: Identifier("b"),
                        },
                        Direct,
                        Ast {
                            span: 31..38,
                            kind: StringLiteral("Hello"),
                        },
                    ),
                ],
            },
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 61..104,
            kind: Update {
                target: Box::new(Ast {
                    span: 68..79,
                    kind: IdOp(
                        Box::new(Ast {
                            span: 68..74,
                            kind: Identifier("person"),
                        }),
                        Box::new(Ast {
                            span: 75..79,
                            kind: Identifier("john"),
                        }),
                    ),
                }),
                where_guard: None,
                column_ops: vec![
                    (
                        Ast {
                            span: 84..85,
                            kind: Identifier("a"),
                        },
                        Minus,
                        Ast {
                            span: 89..91,
                            kind: IntegerLiteral(10),
                        },
                    ),
                    (
                        Ast {
                            span: 93..94,
                            kind: Identifier("b"),
                        },
                        Direct,
                        Ast {
                            span: 97..104,
                            kind: StringLiteral("Hello"),
                        },
                    ),
                ],
            },
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 114..162,
            kind: Update {
                target: Box::new(Ast {
                    span: 121..125,
                    kind: Identifier("test"),
                }),
                where_guard: Some(Box::new(Ast {
                    span: 156..162,
                    kind: MatchOp(
                        Box::new(Ast {
                            span: 156..157,
                            kind: Identifier("a"),
                        }),
                        Box::new(Ast {
                            span: 160..162,
                            kind: IntegerLiteral(10),
                        }),
                    ),
                })),
                column_ops: vec![
                    (
                        Ast {
                            span: 131..132,
                            kind: Identifier("a"),
                        },
                        Direct,
                        Ast {
                            span: 134..136,
                            kind: IntegerLiteral(10),
                        },
                    ),
                    (
                        Ast {
                            span: 138..139,
                            kind: Identifier("b"),
                        },
                        Direct,
                        Ast {
                            span: 141..148,
                            kind: StringLiteral("Hello"),
                        },
                    ),
                ],
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_partial_select_field_fold() -> anyhow::Result<()> {
    let parser = &mut Parser::new("FOLD avg([1, 2, 3]) fold mod::avg(age) as avg", 20);
    let result_a = parser.parse_partial_select_field_fold()?;
    let result_b = parser.parse_partial_select_field_fold()?;

    info!(
        r#"input = {:?} | parse_partial_select_field_fold parse_partial_select_field_fold = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_select_field() -> anyhow::Result<()> {
    let parser = &mut Parser::new("fold avg(price) as a age / 20 as d name.* as c *", 20);
    let result_a = parser.parse_partial_select_field()?;
    let result_b = parser.parse_partial_select_field()?;
    let result_c = parser.parse_partial_select_field()?;
    let result_d = parser.parse_partial_select_field()?;

    info!(
        r#"input = {:?} | parse_partial_select_field parse_partial_select_field parse_partial_select_field parse_partial_select_field = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());
    assert!(result_c.is_some());
    assert!(result_d.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_select_omit() -> anyhow::Result<()> {
    let parser = &mut Parser::new("omit a, b\tOMIT 5 + 20, 'Hello'", 20);
    let result_a = parser.parse_partial_select_omit()?;
    let result_b = parser.parse_partial_select_omit()?;

    info!(
        r#"input = {:?} | parse_partial_select_omit parse_partial_select_omit = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_select_fields() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"name as n, test.age as a OMIT a, b\
        test.* OMIT test.x"#,
        20,
    );
    let result_a = parser.parse_partial_select_fields()?;
    let result_b = parser.parse_partial_select_fields()?;

    info!(
        r#"input = {:?} | parse_partial_select_fields parse_partial_select_fields = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_select_from() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "from person as p, organisation as o FROM friend as f -> likes as l -> animal as a",
        20,
    );
    let result_a = parser.parse_partial_select_from()?;
    let result_b = parser.parse_partial_select_from()?;

    info!(
        r#"input = {:?} | parse_partial_select_from parse_partial_select_from = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_select_with_indices() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "with indices a, b, c WITH INDEX a with no index with indexes a",
        20,
    );
    let result_a = parser.parse_partial_select_with_indices()?;
    let result_b = parser.parse_partial_select_with_indices()?;
    let result_c = parser.parse_partial_select_with_indices()?;
    let result_d = parser.parse_partial_select_with_indices()?;

    info!(
        r#"input = {:?} | parse_partial_select_with_indices parse_partial_select_with_indices parse_partial_select_with_indices parse_partial_select_with_indices = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());
    assert!(result_c.is_some());
    assert!(result_d.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_select_group_by() -> anyhow::Result<()> {
    let parser = &mut Parser::new("group by a, b, c GROUP a + 0b100", 20);
    let result_a = parser.parse_partial_select_group_by()?;
    let result_b = parser.parse_partial_select_group_by()?;

    info!(
        r#"input = {:?} | parse_partial_select_group_by parse_partial_select_group_by = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_select_order_by() -> anyhow::Result<()> {
    let parser = &mut Parser::new("order by a, b, c ORDER a + 0b100", 20);
    let result_a = parser.parse_partial_select_order_by()?;
    let result_b = parser.parse_partial_select_order_by()?;

    info!(
        r#"input = {:?} | parse_set_object parse_set_object = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_select_start_at() -> anyhow::Result<()> {
    let parser = &mut Parser::new("start at 10 START n * 0x5", 20);
    let result_a = parser.parse_partial_select_start_at()?;
    let result_b = parser.parse_partial_select_start_at()?;

    info!(
        r#"input = {:?} | parse_partial_select_start_at parse_partial_select_start_at = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_select_limit_to() -> anyhow::Result<()> {
    let parser = &mut Parser::new("limit to 10 LIMIT n * 0x5", 20);
    let result_a = parser.parse_partial_select_limit_to()?;
    let result_b = parser.parse_partial_select_limit_to()?;

    info!(
        r#"input = {:?} | parse_partial_select_limit_to parse_partial_select_limit_to = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_select_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"SELECT p.address.*, o.name as n OMIT p.name FROM person as p, organisation as o WHERE p.age > 18 ORDER BY p.age WITH NO INDEX\
        select name, age from person order by name group by age + 1, name with index name, age limit to 100 start at 5 where age > 18\
        SELECT *, FOLD avg(age) as avg_age FROM *
        "#,
        20,
    );
    let result_a = parser.parse_select_exp()?;
    let result_b = parser.parse_select_exp()?;
    let result_c = parser.parse_select_exp()?;

    info!(
        r#"input = {:?} | parse_select_exp parse_select_exp parse_select_exp = {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    // assert_eq!(result_a, None);

    // assert_eq!(result_b, None);

    // assert_eq!(result_c, None);

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
