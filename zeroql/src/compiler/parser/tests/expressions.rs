use tracing::info;

use crate::{
    ast::{
        Ast, AstKind::*, Direction::*, ElseIfPart, RelateArrow::*, SelectColumn::*,
        SelectTransform::*, TypeSig::*, UpdateAssign::*,
    },
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
        RELATE a -> of -> b\
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
                                    kind: Identifier("a"),
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
                                    kind: Identifier("of"),
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
                                    kind: Identifier("b"),
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
    let parser = &mut Parser::new("identifier id:op person -> op <- person", 20);
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
    let parser = &mut Parser::new("= += -= *= /= %= **= &= |= ^= ~= <<= >>=", 20);
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

    info!(
        r#"input = {:?} | = {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?}"#,
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
    let parser = &mut Parser::new("FOLD avg([1, 2, 3]) fold std::avg(age) as avg", 20);
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

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..125,
            kind: Select {
                fields: vec![
                    Column(Box::new(Ast {
                        span: 7..18,
                        kind: DotAccessWildcardOp {
                            subject: Box::new(Ast {
                                span: 7..16,
                                kind: DotAccessOp {
                                    subject: Box::new(Ast {
                                        span: 7..8,
                                        kind: Identifier("p",),
                                    }),
                                    field: Box::new(Ast {
                                        span: 9..16,
                                        kind: Identifier("address",),
                                    }),
                                },
                            }),
                        },
                    })),
                    Column(Box::new(Ast {
                        span: 20..31,
                        kind: AliasOp {
                            subject: Box::new(Ast {
                                span: 20..26,
                                kind: DotAccessOp {
                                    subject: Box::new(Ast {
                                        span: 20..21,
                                        kind: Identifier("o",),
                                    }),
                                    field: Box::new(Ast {
                                        span: 22..26,
                                        kind: Identifier("name",),
                                    }),
                                },
                            }),
                            alias: Box::new(Ast {
                                span: 30..31,
                                kind: Identifier("n",),
                            }),
                        },
                    })),
                ],
                omit: vec![Ast {
                    span: 37..43,
                    kind: DotAccessOp {
                        subject: Box::new(Ast {
                            span: 37..38,
                            kind: Identifier("p",),
                        }),
                        field: Box::new(Ast {
                            span: 39..43,
                            kind: Identifier("name",),
                        }),
                    },
                }],
                from: vec![
                    Ast {
                        span: 49..60,
                        kind: AliasOp {
                            subject: Box::new(Ast {
                                span: 49..55,
                                kind: Identifier("person",),
                            }),
                            alias: Box::new(Ast {
                                span: 59..60,
                                kind: Identifier("p",),
                            }),
                        },
                    },
                    Ast {
                        span: 62..79,
                        kind: AliasOp {
                            subject: Box::new(Ast {
                                span: 62..74,
                                kind: Identifier("organisation",),
                            }),
                            alias: Box::new(Ast {
                                span: 78..79,
                                kind: Identifier("o",),
                            }),
                        },
                    },
                ],
                transforms: vec![
                    WhereGuard(Box::new(Ast {
                        span: 86..96,
                        kind: GreaterThanOp(
                            Box::new(Ast {
                                span: 86..91,
                                kind: DotAccessOp {
                                    subject: Box::new(Ast {
                                        span: 86..87,
                                        kind: Identifier("p",),
                                    }),
                                    field: Box::new(Ast {
                                        span: 88..91,
                                        kind: Identifier("age",),
                                    }),
                                },
                            }),
                            Box::new(Ast {
                                span: 94..96,
                                kind: IntegerLiteral(18,),
                            }),
                        ),
                    })),
                    OrderBy {
                        fields: vec![Ast {
                            span: 106..111,
                            kind: DotAccessOp {
                                subject: Box::new(Ast {
                                    span: 106..107,
                                    kind: Identifier("p",),
                                }),
                                field: Box::new(Ast {
                                    span: 108..111,
                                    kind: Identifier("age",),
                                }),
                            },
                        },],
                        direction: Ascending,
                    },
                    WithNoIndex,
                ],
            },
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 135..260,
            kind: Select {
                fields: vec![
                    Column(Box::new(Ast {
                        span: 142..146,
                        kind: Identifier("name",),
                    })),
                    Column(Box::new(Ast {
                        span: 148..151,
                        kind: Identifier("age",),
                    })),
                ],
                omit: vec![],
                from: vec![Ast {
                    span: 157..163,
                    kind: Identifier("person",),
                }],
                transforms: vec![
                    OrderBy {
                        fields: vec![Ast {
                            span: 173..177,
                            kind: Identifier("name",),
                        }],
                        direction: Ascending,
                    },
                    GroupBy(vec![
                        Ast {
                            span: 187..194,
                            kind: AdditionOp(
                                Box::new(Ast {
                                    span: 187..190,
                                    kind: Identifier("age",),
                                }),
                                Box::new(Ast {
                                    span: 193..194,
                                    kind: IntegerLiteral(1,),
                                }),
                            ),
                        },
                        Ast {
                            span: 196..200,
                            kind: Identifier("name",),
                        },
                    ],),
                    WithIndexes(vec![
                        Ast {
                            span: 212..216,
                            kind: Identifier("name",),
                        },
                        Ast {
                            span: 218..221,
                            kind: Identifier("age",),
                        },
                    ],),
                    LimitTo(Box::new(Ast {
                        span: 231..234,
                        kind: IntegerLiteral(100,),
                    })),
                    StartAt(Box::new(Ast {
                        span: 244..245,
                        kind: IntegerLiteral(5,),
                    })),
                    WhereGuard(Box::new(Ast {
                        span: 252..260,
                        kind: GreaterThanOp(
                            Box::new(Ast {
                                span: 252..255,
                                kind: Identifier("age",),
                            }),
                            Box::new(Ast {
                                span: 258..260,
                                kind: IntegerLiteral(18,),
                            }),
                        ),
                    })),
                ],
            },
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 270..311,
            kind: Select {
                fields: vec![
                    Column(Box::new(Ast {
                        span: 277..278,
                        kind: Wildcard,
                    })),
                    Fold {
                        subject: Box::new(Ast {
                            span: 280..304,
                            kind: FunctionCall {
                                subject: Box::new(Ast {
                                    span: 285..288,
                                    kind: Identifier("avg",),
                                }),
                                args: vec![Ast {
                                    span: 289..292,
                                    kind: FunctionArg {
                                        name: None,
                                        value: Box::new(Ast {
                                            span: 289..292,
                                            kind: Identifier("age",),
                                        }),
                                    },
                                },],
                            },
                        }),
                        alias: Some(Box::new(Ast {
                            span: 297..304,
                            kind: Identifier("avg_age",),
                        })),
                    },
                ],
                omit: vec![],
                from: vec![Ast {
                    span: 310..311,
                    kind: Wildcard,
                }],
                transforms: vec![],
            },
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_partial_if_exists() -> anyhow::Result<()> {
    let parser = &mut Parser::new("if exists IF EXISTS", 20);
    let result_a = parser.parse_partial_if_exists()?;
    let result_b = parser.parse_partial_if_exists()?;

    info!(
        r#"input = {:?} | parse_partial_if_exists parse_partial_if_exists = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_on_namespace() -> anyhow::Result<()> {
    let parser = &mut Parser::new("on namespace identifier ON NS `ns`", 20);
    let result_a = parser.parse_partial_on_namespace()?;
    let result_b = parser.parse_partial_on_namespace()?;

    info!(
        r#"input = {:?} | parse_partial_on_namespace parse_partial_on_namespace = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_on_database() -> anyhow::Result<()> {
    let parser = &mut Parser::new("on database identifier ON DB `db`", 20);
    let result_a = parser.parse_partial_on_database()?;
    let result_b = parser.parse_partial_on_database()?;

    info!(
        r#"input = {:?} | parse_partial_on_database parse_partial_on_database = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_on_table() -> anyhow::Result<()> {
    let parser = &mut Parser::new("on table identifier ON TABLE `t`", 20);
    let result_a = parser.parse_partial_on_table()?;
    let result_b = parser.parse_partial_on_table()?;

    info!(
        r#"input = {:?} | parse_partial_on_table parse_partial_on_table = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_remove_namespace_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "remove namespace IF EXISTS `ns` REMOVE NAMESPACE app REMOVE NS app IF EXISTS",
        20,
    );
    let result_a = parser.parse_remove_namespace_exp()?;
    let result_b = parser.parse_remove_namespace_exp()?;
    let result_c = parser.parse_remove_namespace_exp()?;

    info!(
        r#"input = {:?} | parse_remove_namespace_exp parse_remove_namespace_exp parse_remove_namespace_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..31,
            kind: RemoveNamespace {
                subject: Box::new(Ast {
                    span: 27..31,
                    kind: Identifier("ns",),
                }),
                if_exists: true,
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 32..52,
            kind: RemoveNamespace {
                subject: Box::new(Ast {
                    span: 49..52,
                    kind: Identifier("app"),
                }),
                if_exists: false,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 53..76,
            kind: RemoveNamespace {
                subject: Box::new(Ast {
                    span: 63..66,
                    kind: Identifier("app"),
                }),
                if_exists: true,
            },
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_remove_database_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "remove database IF EXISTS `db` on ns `ns` REMOVE DATABASE my_db REMOVE DB my_db IF EXISTS ON NAMESPACE my_app",
        20,
    );
    let result_a = parser.parse_remove_database_exp()?;
    let result_b = parser.parse_remove_database_exp()?;
    let result_c = parser.parse_remove_database_exp()?;

    info!(
        r#"input = {:?} | parse_remove_database_exp parse_remove_database_exp parse_remove_database_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..41,
            kind: RemoveDatabase {
                subject: Box::new(Ast {
                    span: 26..30,
                    kind: Identifier("db",),
                }),
                if_exists: true,
                namespace: Some(Box::new(Ast {
                    span: 37..41,
                    kind: Identifier("ns"),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 42..63,
            kind: RemoveDatabase {
                subject: Box::new(Ast {
                    span: 58..63,
                    kind: Identifier("my_db"),
                }),
                if_exists: false,
                namespace: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 64..109,
            kind: RemoveDatabase {
                subject: Box::new(Ast {
                    span: 74..79,
                    kind: Identifier("my_db")
                }),
                if_exists: true,
                namespace: Some(Box::new(Ast {
                    span: 103..109,
                    kind: Identifier("my_app"),
                })),
            },
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_remove_table_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "remove table IF EXISTS `table` on db `db` REMOVE TABLE my_table REMOVE TABLE my_table IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_remove_table_exp()?;
    let result_b = parser.parse_remove_table_exp()?;
    let result_c = parser.parse_remove_table_exp()?;

    info!(
        r#"input = {:?} | parse_remove_table_exp parse_remove_table_exp parse_remove_table_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..41,
            kind: RemoveTable {
                subject: Box::new(Ast {
                    span: 23..30,
                    kind: Identifier("table",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 37..41,
                    kind: Identifier("db"),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 42..63,
            kind: RemoveTable {
                subject: Box::new(Ast {
                    span: 55..63,
                    kind: Identifier("my_table"),
                }),
                if_exists: false,
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 64..113,
            kind: RemoveTable {
                subject: Box::new(Ast {
                    span: 77..85,
                    kind: Identifier("my_table",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 108..113,
                    kind: Identifier("my_db"),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_remove_edge_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "remove edge IF EXISTS `edge` on db `db` REMOVE EDGE my_edge REMOVE EDGE my_edge IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_remove_edge_exp()?;
    let result_b = parser.parse_remove_edge_exp()?;
    let result_c = parser.parse_remove_edge_exp()?;

    info!(
        r#"input = {:?} | parse_remove_edge_exp parse_remove_edge_exp parse_remove_edge_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..39,
            kind: RemoveEdge {
                subject: Box::new(Ast {
                    span: 22..28,
                    kind: Identifier("edge",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 35..39,
                    kind: Identifier("db",),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 40..59,
            kind: RemoveEdge {
                subject: Box::new(Ast {
                    span: 52..59,
                    kind: Identifier("my_edge",),
                }),
                if_exists: false,
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 60..107,
            kind: RemoveEdge {
                subject: Box::new(Ast {
                    span: 72..79,
                    kind: Identifier("my_edge",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 102..107,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_remove_type_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "remove type IF EXISTS `type` on db `db` REMOVE TYPE my_type REMOVE TYPE my_type IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_remove_type_exp()?;
    let result_b = parser.parse_remove_type_exp()?;
    let result_c = parser.parse_remove_type_exp()?;

    info!(
        r#"input = {:?} | parse_remove_type_exp parse_remove_type_exp parse_remove_type_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..39,
            kind: RemoveType {
                subject: Box::new(Ast {
                    span: 22..28,
                    kind: Identifier("type",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 35..39,
                    kind: Identifier("db",),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 40..59,
            kind: RemoveType {
                subject: Box::new(Ast {
                    span: 52..59,
                    kind: Identifier("my_type",),
                }),
                if_exists: false,
                database: None,
            },
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 60..107,
            kind: RemoveType {
                subject: Box::new(Ast {
                    span: 72..79,
                    kind: Identifier("my_type",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 102..107,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_remove_enum_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "remove enum IF EXISTS `enum` on db `db` REMOVE ENUM my_enum REMOVE ENUM my_enum IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_remove_enum_exp()?;
    let result_b = parser.parse_remove_enum_exp()?;
    let result_c = parser.parse_remove_enum_exp()?;

    info!(
        r#"input = {:?} | parse_remove_enum_exp parse_remove_enum_exp parse_remove_enum_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..39,
            kind: RemoveEnum {
                subject: Box::new(Ast {
                    span: 22..28,
                    kind: Identifier("enum",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 35..39,
                    kind: Identifier("db",),
                })),
            },
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 40..59,
            kind: RemoveEnum {
                subject: Box::new(Ast {
                    span: 52..59,
                    kind: Identifier("my_enum",),
                }),
                if_exists: false,
                database: None,
            },
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 60..107,
            kind: RemoveEnum {
                subject: Box::new(Ast {
                    span: 72..79,
                    kind: Identifier("my_enum",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 102..107,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_remove_index_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "remove index IF EXISTS `index` on table `table` REMOVE INDEX my_index ON TABLE my_table REMOVE INDEX my_index IF EXISTS ON TABLE my_table ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_remove_index_exp()?;
    let result_b = parser.parse_remove_index_exp()?;
    let result_c = parser.parse_remove_index_exp()?;

    info!(
        r#"input = {:?} | parse_remove_index_exp parse_remove_index_exp parse_remove_index_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..47,
            kind: RemoveIndex {
                subject: Box::new(Ast {
                    span: 23..30,
                    kind: Identifier("index",),
                }),
                if_exists: true,
                table: Box::new(Ast {
                    span: 40..47,
                    kind: Identifier("table",),
                }),
                database: None,
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 48..87,
            kind: RemoveIndex {
                subject: Box::new(Ast {
                    span: 61..69,
                    kind: Identifier("my_index",),
                }),
                if_exists: false,
                table: Box::new(Ast {
                    span: 79..87,
                    kind: Identifier("my_table",),
                }),
                database: None,
            },
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 88..155,
            kind: RemoveIndex {
                subject: Box::new(Ast {
                    span: 101..109,
                    kind: Identifier("my_index",),
                }),
                if_exists: true,
                table: Box::new(Ast {
                    span: 129..137,
                    kind: Identifier("my_table",),
                }),
                database: Some(Box::new(Ast {
                    span: 150..155,
                    kind: Identifier("my_db",),
                })),
            },
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_remove_module_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "remove module IF EXISTS `module` on db `db` REMOVE MODULE my_module REMOVE MODULE my_module IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_remove_module_exp()?;
    let result_b = parser.parse_remove_module_exp()?;
    let result_c = parser.parse_remove_module_exp()?;

    info!(
        r#"input = {:?} | parse_remove_module_exp parse_remove_module_exp parse_remove_module_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..43,
            kind: RemoveModule {
                subject: Box::new(Ast {
                    span: 24..32,
                    kind: Identifier("module",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 39..43,
                    kind: Identifier("db",),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 44..67,
            kind: RemoveModule {
                subject: Box::new(Ast {
                    span: 58..67,
                    kind: Identifier("my_module",),
                }),
                if_exists: false,
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 68..119,
            kind: RemoveModule {
                subject: Box::new(Ast {
                    span: 82..91,
                    kind: Identifier("my_module",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 114..119,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_remove_param_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "remove param IF EXISTS $param on db `db` REMOVE PARAM $param REMOVE PARAM $my_param IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_remove_param_exp()?;
    let result_b = parser.parse_remove_param_exp()?;
    let result_c = parser.parse_remove_param_exp()?;

    info!(
        r#"input = {:?} | parse_remove_param_exp parse_remove_param_exp parse_remove_param_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..40,
            kind: RemoveParam {
                subject: Box::new(Ast {
                    span: 23..29,
                    kind: Variable("param"),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 36..40,
                    kind: Identifier("db"),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 41..60,
            kind: RemoveParam {
                subject: Box::new(Ast {
                    span: 54..60,
                    kind: Variable("param"),
                }),
                if_exists: false,
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 61..111,
            kind: RemoveParam {
                subject: Box::new(Ast {
                    span: 74..83,
                    kind: Variable("my_param"),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 106..111,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_remove_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"REMOVE NAMESPACE my_ns IF EXISTS\
        REMOVE DATABASE my_db ON NAMESPACE my_ns IF EXISTS\
        REMOVE TABLE my_table ON DATABASE my_db IF EXISTS\
        REMOVE EDGE my_edge ON DATABASE my_db IF EXISTS\
        REMOVE TYPE my_type ON DATABASE my_db IF EXISTS\
        REMOVE ENUM my_enum ON DATABASE my_db IF EXISTS\
        REMOVE INDEX my_index ON DATABASE my_db IF EXISTS ON TABLE my_table\
        REMOVE MODULE my_module ON DATABASE my_db IF EXISTS\
        REMOVE PARAM $my_param ON DATABASE my_db IF EXISTS\
        "#,
        20,
    );

    let result_a = parser.parse_remove_exp()?;
    let result_b = parser.parse_remove_exp()?;
    let result_c = parser.parse_remove_exp()?;
    let result_d = parser.parse_remove_exp()?;
    let result_e = parser.parse_remove_exp()?;
    let result_f = parser.parse_remove_exp()?;
    let result_g = parser.parse_remove_exp()?;
    let result_h = parser.parse_remove_exp()?;
    let result_i = parser.parse_remove_exp()?;

    info!(
        r#"input = {:?} | = {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?}"#,
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

    Ok(())
}

#[test_log::test]
fn test_parser_describe_namespace_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "describe namespace IF EXISTS `ns` DESCRIBE NAMESPACE app DESCRIBE NS app IF EXISTS",
        20,
    );
    let result_a = parser.parse_describe_namespace_exp()?;
    let result_b = parser.parse_describe_namespace_exp()?;
    let result_c = parser.parse_describe_namespace_exp()?;

    info!(
        r#"input = {:?} | parse_describe_namespace_exp parse_describe_namespace_exp parse_describe_namespace_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..33,
            kind: DescribeNamespace {
                subject: Box::new(Ast {
                    span: 29..33,
                    kind: Identifier("ns",),
                }),
                if_exists: true,
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 34..56,
            kind: DescribeNamespace {
                subject: Box::new(Ast {
                    span: 53..56,
                    kind: Identifier("app",),
                }),
                if_exists: false,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 57..82,
            kind: DescribeNamespace {
                subject: Box::new(Ast {
                    span: 69..72,
                    kind: Identifier("app",),
                }),
                if_exists: true,
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_describe_database_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "describe database IF EXISTS `db` on ns `ns` DESCRIBE DATABASE my_db DESCRIBE DB my_db IF EXISTS ON NAMESPACE my_app",
        20,
    );
    let result_a = parser.parse_describe_database_exp()?;
    let result_b = parser.parse_describe_database_exp()?;
    let result_c = parser.parse_describe_database_exp()?;

    info!(
        r#"input = {:?} | parse_describe_database_exp parse_describe_database_exp parse_describe_database_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..43,
            kind: DescribeDatabase {
                subject: Box::new(Ast {
                    span: 28..32,
                    kind: Identifier("db",),
                }),
                if_exists: true,
                namespace: Some(Box::new(Ast {
                    span: 39..43,
                    kind: Identifier("ns",),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 44..67,
            kind: DescribeDatabase {
                subject: Box::new(Ast {
                    span: 62..67,
                    kind: Identifier("my_db",),
                }),
                if_exists: false,
                namespace: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 68..115,
            kind: DescribeDatabase {
                subject: Box::new(Ast {
                    span: 80..85,
                    kind: Identifier("my_db",),
                }),
                if_exists: true,
                namespace: Some(Box::new(Ast {
                    span: 109..115,
                    kind: Identifier("my_app",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_describe_table_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "describe table IF EXISTS `table` on db `db` DESCRIBE TABLE my_table DESCRIBE TABLE my_table IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_describe_table_exp()?;
    let result_b = parser.parse_describe_table_exp()?;
    let result_c = parser.parse_describe_table_exp()?;

    info!(
        r#"input = {:?} | parse_describe_table_exp parse_describe_table_exp parse_describe_table_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..43,
            kind: DescribeTable {
                subject: Box::new(Ast {
                    span: 25..32,
                    kind: Identifier("table",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 39..43,
                    kind: Identifier("db",),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 44..67,
            kind: DescribeTable {
                subject: Box::new(Ast {
                    span: 59..67,
                    kind: Identifier("my_table",),
                }),
                if_exists: false,
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 68..119,
            kind: DescribeTable {
                subject: Box::new(Ast {
                    span: 83..91,
                    kind: Identifier("my_table",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 114..119,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_describe_edge_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "describe edge IF EXISTS `edge` on db `db` DESCRIBE EDGE my_edge DESCRIBE EDGE my_edge IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_describe_edge_exp()?;
    let result_b = parser.parse_describe_edge_exp()?;
    let result_c = parser.parse_describe_edge_exp()?;

    info!(
        r#"input = {:?} | parse_describe_edge_exp parse_describe_edge_exp parse_describe_edge_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..41,
            kind: DescribeEdge {
                subject: Box::new(Ast {
                    span: 24..30,
                    kind: Identifier("edge",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 37..41,
                    kind: Identifier("db",),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 42..63,
            kind: DescribeEdge {
                subject: Box::new(Ast {
                    span: 56..63,
                    kind: Identifier("my_edge",),
                }),
                if_exists: false,
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 64..113,
            kind: DescribeEdge {
                subject: Box::new(Ast {
                    span: 78..85,
                    kind: Identifier("my_edge",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 108..113,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_describe_type_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "describe type IF EXISTS `type` on db `db` DESCRIBE TYPE my_type DESCRIBE TYPE my_type IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_describe_type_exp()?;
    let result_b = parser.parse_describe_type_exp()?;
    let result_c = parser.parse_describe_type_exp()?;

    info!(
        r#"input = {:?} | parse_describe_type_exp parse_describe_type_exp parse_describe_type_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..41,
            kind: DescribeType {
                subject: Box::new(Ast {
                    span: 24..30,
                    kind: Identifier("type",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 37..41,
                    kind: Identifier("db",),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 42..63,
            kind: DescribeType {
                subject: Box::new(Ast {
                    span: 56..63,
                    kind: Identifier("my_type",),
                }),
                if_exists: false,
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 64..113,
            kind: DescribeType {
                subject: Box::new(Ast {
                    span: 78..85,
                    kind: Identifier("my_type",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 108..113,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_describe_enum_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "describe enum IF EXISTS `enum` on db `db` DESCRIBE ENUM my_enum DESCRIBE ENUM my_enum IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_describe_enum_exp()?;
    let result_b = parser.parse_describe_enum_exp()?;
    let result_c = parser.parse_describe_enum_exp()?;

    info!(
        r#"input = {:?} | parse_describe_enum_exp parse_describe_enum_exp parse_describe_enum_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..41,
            kind: DescribeEnum {
                subject: Box::new(Ast {
                    span: 24..30,
                    kind: Identifier("enum",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 37..41,
                    kind: Identifier("db",),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 42..63,
            kind: DescribeEnum {
                subject: Box::new(Ast {
                    span: 56..63,
                    kind: Identifier("my_enum",),
                }),
                if_exists: false,
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 64..113,
            kind: DescribeEnum {
                subject: Box::new(Ast {
                    span: 78..85,
                    kind: Identifier("my_enum",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 108..113,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_describe_index_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "describe index IF EXISTS `index` on table `table` DESCRIBE INDEX my_index ON TABLE my_table DESCRIBE INDEX my_index IF EXISTS ON TABLE my_table ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_describe_index_exp()?;
    let result_b = parser.parse_describe_index_exp()?;
    let result_c = parser.parse_describe_index_exp()?;

    info!(
        r#"input = {:?} | parse_describe_index_exp parse_describe_index_exp parse_describe_index_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..49,
            kind: DescribeIndex {
                subject: Box::new(Ast {
                    span: 25..32,
                    kind: Identifier("index",),
                }),
                if_exists: true,
                table: Box::new(Ast {
                    span: 42..49,
                    kind: Identifier("table",),
                }),
                database: None,
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 50..91,
            kind: DescribeIndex {
                subject: Box::new(Ast {
                    span: 65..73,
                    kind: Identifier("my_index",),
                }),
                if_exists: false,
                table: Box::new(Ast {
                    span: 83..91,
                    kind: Identifier("my_table",),
                }),
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 92..161,
            kind: DescribeIndex {
                subject: Box::new(Ast {
                    span: 107..115,
                    kind: Identifier("my_index",),
                }),
                if_exists: true,
                table: Box::new(Ast {
                    span: 135..143,
                    kind: Identifier("my_table",),
                }),
                database: Some(Box::new(Ast {
                    span: 156..161,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_describe_module_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "describe module IF EXISTS `module` on db `db` DESCRIBE MODULE my_module DESCRIBE MODULE my_module IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_describe_module_exp()?;
    let result_b = parser.parse_describe_module_exp()?;
    let result_c = parser.parse_describe_module_exp()?;

    info!(
        r#"input = {:?} | parse_describe_module_exp parse_describe_module_exp parse_describe_module_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..45,
            kind: DescribeModule {
                subject: Box::new(Ast {
                    span: 26..34,
                    kind: Identifier("module",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 41..45,
                    kind: Identifier("db",),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 46..71,
            kind: DescribeModule {
                subject: Box::new(Ast {
                    span: 62..71,
                    kind: Identifier("my_module",),
                }),
                if_exists: false,
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 72..125,
            kind: DescribeModule {
                subject: Box::new(Ast {
                    span: 88..97,
                    kind: Identifier("my_module",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 120..125,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_describe_param_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "describe param IF EXISTS $param on db `db` DESCRIBE PARAM $param DESCRIBE PARAM $my_param IF EXISTS ON DATABASE my_db",
        20,
    );
    let result_a = parser.parse_describe_param_exp()?;
    let result_b = parser.parse_describe_param_exp()?;
    let result_c = parser.parse_describe_param_exp()?;

    info!(
        r#"input = {:?} | parse_describe_param_exp parse_describe_param_exp parse_describe_param_exp = {:#?} {:#?} {:#?} "#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..42,
            kind: DescribeParam {
                subject: Box::new(Ast {
                    span: 25..31,
                    kind: Variable("param",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 38..42,
                    kind: Identifier("db",),
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 43..64,
            kind: DescribeParam {
                subject: Box::new(Ast {
                    span: 58..64,
                    kind: Variable("param",),
                }),
                if_exists: false,
                database: None,
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 65..117,
            kind: DescribeParam {
                subject: Box::new(Ast {
                    span: 80..89,
                    kind: Variable("my_param",),
                }),
                if_exists: true,
                database: Some(Box::new(Ast {
                    span: 112..117,
                    kind: Identifier("my_db",),
                })),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_describe_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"DESCRIBE NAMESPACE my_ns IF EXISTS\
        DESCRIBE DATABASE my_db ON NAMESPACE my_ns IF EXISTS\
        DESCRIBE TABLE my_table ON DATABASE my_db IF EXISTS\
        DESCRIBE EDGE my_edge ON DATABASE my_db IF EXISTS\
        DESCRIBE TYPE my_type ON DATABASE my_db IF EXISTS\
        DESCRIBE ENUM my_enum ON DATABASE my_db IF EXISTS\
        DESCRIBE INDEX my_index ON DATABASE my_db IF EXISTS ON TABLE my_table\
        DESCRIBE MODULE my_module ON DATABASE my_db IF EXISTS\
        DESCRIBE PARAM $my_param ON DATABASE my_db IF EXISTS\
        "#,
        20,
    );

    let result_a = parser.parse_describe_exp()?;
    let result_b = parser.parse_describe_exp()?;
    let result_c = parser.parse_describe_exp()?;
    let result_d = parser.parse_describe_exp()?;
    let result_e = parser.parse_describe_exp()?;
    let result_f = parser.parse_describe_exp()?;
    let result_g = parser.parse_describe_exp()?;
    let result_h = parser.parse_describe_exp()?;
    let result_i = parser.parse_describe_exp()?;

    info!(
        r#"input = {:?} | = {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?}"#,
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

    Ok(())
}

#[test_log::test]
fn test_parser_begin_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new("begin BEGIN TRANSACTION", 20);
    let result_a = parser.parse_begin_exp()?;
    let result_b = parser.parse_begin_exp()?;

    info!(
        r#"input = {:?} | parse_begin_exp parse_begin_exp = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..5,
            kind: BeginTransaction,
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 6..23,
            kind: BeginTransaction,
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_commit_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new("commit COMMIT TRANSACTION", 20);
    let result_a = parser.parse_commit_exp()?;
    let result_b = parser.parse_commit_exp()?;

    info!(
        r#"input = {:?} | parse_commit_exp parse_commit_exp = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..6,
            kind: CommitTransaction,
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 7..25,
            kind: CommitTransaction,
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_cancel_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new("cancel CANCEL TRANSACTION", 20);
    let result_a = parser.parse_cancel_exp()?;
    let result_b = parser.parse_cancel_exp()?;

    info!(
        r#"input = {:?} | parse_cancel_exp parse_cancel_exp = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..6,
            kind: CancelTransaction,
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 7..25,
            kind: CancelTransaction,
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_for_exp() -> anyhow::Result<()> {
    // TODO: Add more program examples for the for body
    let parser = &mut Parser::new("for $i in 1..10 do print($i) end", 20);
    let result_a = parser.parse_for_exp()?;
    // let result_b = parser.parse_for_exp()?;

    info!(
        r#"input = {:?} | parse_for_exp = {:#?}"#,
        parser.lexer.string, result_a,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..32,
            kind: For {
                variable: Box::new(Ast {
                    span: 4..6,
                    kind: Variable("i"),
                }),
                iterator: Box::new(Ast {
                    span: 10..15,
                    kind: RangeOp(
                        Box::new(Ast {
                            span: 10..11,
                            kind: IntegerLiteral(1),
                        }),
                        Box::new(Ast {
                            span: 13..15,
                            kind: IntegerLiteral(10),
                        }),
                    ),
                }),
                body: Box::new(Ast {
                    span: 19..28,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 19..24,
                            kind: Identifier("print"),
                        }),
                        args: vec![Ast {
                            span: 25..27,
                            kind: FunctionArg {
                                name: None,
                                value: Box::new(Ast {
                                    span: 25..27,
                                    kind: Variable("i"),
                                }),
                            },
                        }],
                    },
                }),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_while_exp() -> anyhow::Result<()> {
    // TODO: Add more program examples for the while body
    let parser = &mut Parser::new("while $i = 1 do print($i) end", 20);
    let result_a = parser.parse_while_exp()?;
    // let result_b = parser.parse_while_exp()?;

    info!(
        r#"input = {:?} | parse_while_exp = {:#?}"#,
        parser.lexer.string, result_a,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..29,
            kind: While {
                condition: Box::new(Ast {
                    span: 6..12,
                    kind: IsOp(
                        Box::new(Ast {
                            span: 6..8,
                            kind: Variable("i"),
                        }),
                        Box::new(Ast {
                            span: 11..12,
                            kind: IntegerLiteral(1),
                        }),
                    ),
                }),
                body: Box::new(Ast {
                    span: 16..25,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 16..21,
                            kind: Identifier("print"),
                        }),
                        args: vec![Ast {
                            span: 22..24,
                            kind: FunctionArg {
                                name: None,
                                value: Box::new(Ast {
                                    span: 22..24,
                                    kind: Variable("i"),
                                }),
                            },
                        }],
                    },
                }),
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_partial_else_if_part() -> anyhow::Result<()> {
    // TODO: Add more program examples for the if else body
    let parser = &mut Parser::new("else if $a > 10 then print($a)", 20);
    let result_a = parser.parse_partial_else_if_part()?;
    // let result_b = parser.parse_partial_else_if_part()?;

    info!(
        r#"input = {:?} | parse_partial_else_if_part = {:#?}"#,
        parser.lexer.string, result_a,
    );

    assert!(result_a.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_if_else_exp() -> anyhow::Result<()> {
    let parser: &mut Parser = &mut Parser::new(
        r#"if $a > 10 then print($a) else print($a) end\
        IF $a > 10 THEN print($a) ELSE IF $a > 10 THEN print($a) ELSE print($a) END\
        IF $a > 10 THEN print($a) ELSE IF $a > 10 THEN print($a) END\
        IF $a > 10 THEN print($a) END
        "#,
        20,
    );
    let result_a = parser.parse_if_else_exp()?;
    let result_b = parser.parse_if_else_exp()?;
    let result_c = parser.parse_if_else_exp()?;
    let result_d = parser.parse_if_else_exp()?;

    info!(
        r#"input = {:?} | parse_if_else_exp parse_if_else_exp parse_if_else_exp parse_if_else_exp = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..44,
            kind: If {
                condition: Box::new(Ast {
                    span: 3..10,
                    kind: GreaterThanOp(
                        Box::new(Ast {
                            span: 3..5,
                            kind: Variable("a"),
                        }),
                        Box::new(Ast {
                            span: 8..10,
                            kind: IntegerLiteral(10),
                        }),
                    ),
                }),
                then: Box::new(Ast {
                    span: 16..25,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 16..21,
                            kind: Identifier("print"),
                        }),
                        args: vec![Ast {
                            span: 22..24,
                            kind: FunctionArg {
                                name: None,
                                value: Box::new(Ast {
                                    span: 22..24,
                                    kind: Variable("a"),
                                }),
                            },
                        }],
                    },
                }),
                else_ifs: vec![],
                r#else: Some(Box::new(Ast {
                    span: 31..40,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 31..36,
                            kind: Identifier("print"),
                        }),
                        args: vec![Ast {
                            span: 37..39,
                            kind: FunctionArg {
                                name: None,
                                value: Box::new(Ast {
                                    span: 37..39,
                                    kind: Variable("a"),
                                }),
                            },
                        }],
                    },
                })),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 54..129,
            kind: If {
                condition: Box::new(Ast {
                    span: 57..64,
                    kind: GreaterThanOp(
                        Box::new(Ast {
                            span: 57..59,
                            kind: Variable("a"),
                        }),
                        Box::new(Ast {
                            span: 62..64,
                            kind: IntegerLiteral(10),
                        }),
                    ),
                }),
                then: Box::new(Ast {
                    span: 70..79,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 70..75,
                            kind: Identifier("print"),
                        }),
                        args: vec![Ast {
                            span: 76..78,
                            kind: FunctionArg {
                                name: None,
                                value: Box::new(Ast {
                                    span: 76..78,
                                    kind: Variable("a"),
                                }),
                            },
                        }],
                    }
                }),
                else_ifs: vec![ElseIfPart {
                    condition: Box::new(Ast {
                        span: 88..95,
                        kind: GreaterThanOp(
                            Box::new(Ast {
                                span: 88..90,
                                kind: Variable("a"),
                            }),
                            Box::new(Ast {
                                span: 93..95,
                                kind: IntegerLiteral(10),
                            }),
                        ),
                    }),
                    body: Box::new(Ast {
                        span: 101..110,
                        kind: FunctionCall {
                            subject: Box::new(Ast {
                                span: 101..106,
                                kind: Identifier("print"),
                            }),
                            args: vec![Ast {
                                span: 107..109,
                                kind: FunctionArg {
                                    name: None,
                                    value: Box::new(Ast {
                                        span: 107..109,
                                        kind: Variable("a"),
                                    }),
                                },
                            }],
                        },
                    }),
                }],
                r#else: Some(Box::new(Ast {
                    span: 116..125,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 116..121,
                            kind: Identifier("print",),
                        }),
                        args: vec![Ast {
                            span: 122..124,
                            kind: FunctionArg {
                                name: None,
                                value: Box::new(Ast {
                                    span: 122..124,
                                    kind: Variable("a",),
                                }),
                            },
                        }],
                    },
                })),
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 139..199,
            kind: If {
                condition: Box::new(Ast {
                    span: 142..149,
                    kind: GreaterThanOp(
                        Box::new(Ast {
                            span: 142..144,
                            kind: Variable("a",),
                        }),
                        Box::new(Ast {
                            span: 147..149,
                            kind: IntegerLiteral(10,),
                        }),
                    ),
                }),
                then: Box::new(Ast {
                    span: 155..164,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 155..160,
                            kind: Identifier("print"),
                        }),
                        args: vec![Ast {
                            span: 161..163,
                            kind: FunctionArg {
                                name: None,
                                value: Box::new(Ast {
                                    span: 161..163,
                                    kind: Variable("a"),
                                }),
                            },
                        }],
                    },
                }),
                else_ifs: vec![ElseIfPart {
                    condition: Box::new(Ast {
                        span: 173..180,
                        kind: GreaterThanOp(
                            Box::new(Ast {
                                span: 173..175,
                                kind: Variable("a"),
                            }),
                            Box::new(Ast {
                                span: 178..180,
                                kind: IntegerLiteral(10),
                            }),
                        ),
                    }),
                    body: Box::new(Ast {
                        span: 186..195,
                        kind: FunctionCall {
                            subject: Box::new(Ast {
                                span: 186..191,
                                kind: Identifier("print"),
                            }),
                            args: vec![Ast {
                                span: 192..194,
                                kind: FunctionArg {
                                    name: None,
                                    value: Box::new(Ast {
                                        span: 192..194,
                                        kind: Variable("a"),
                                    }),
                                },
                            }],
                        },
                    }),
                }],
                r#else: None,
            },
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 209..238,
            kind: If {
                condition: Box::new(Ast {
                    span: 212..219,
                    kind: GreaterThanOp(
                        Box::new(Ast {
                            span: 212..214,
                            kind: Variable("a"),
                        }),
                        Box::new(Ast {
                            span: 217..219,
                            kind: IntegerLiteral(10),
                        }),
                    ),
                }),
                then: Box::new(Ast {
                    span: 225..234,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 225..230,
                            kind: Identifier("print"),
                        }),
                        args: vec![Ast {
                            span: 231..233,
                            kind: FunctionArg {
                                name: None,
                                value: Box::new(Ast {
                                    span: 231..233,
                                    kind: Variable("a"),
                                }),
                            },
                        },],
                    },
                }),
                else_ifs: vec![],
                r#else: None,
            },
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_partial_type_sig() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"std::comb[i32, option[i32]?]\
        ([i32]?, i32)\
        [[i32 10]?? 10]\
        [i32]\
        std::i32\
        i32??"#,
        20,
    );
    let result_a = parser.parse_partial_type_sig()?;
    let result_b = parser.parse_partial_type_sig()?;
    let result_c = parser.parse_partial_type_sig()?;
    let result_d = parser.parse_partial_type_sig()?;
    let result_e = parser.parse_partial_type_sig()?;
    let result_f = parser.parse_partial_type_sig()?;

    info!(
        r#"input = {:?} | parse_partial_type_sig parse_partial_type_sig parse_partial_type_sig parse_partial_type_sig parse_partial_type_sig parse_partial_type_sig = {:#?} {:#?} {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d, result_e, result_f,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());
    assert!(result_c.is_some());
    assert!(result_d.is_some());
    assert!(result_e.is_some());
    assert!(result_f.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_let_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"LET $age = 10\
        let $array type [i32 3] = [1, 2, 3]\
        let $array type [i32] = [1, 2, 3]\
        let $option type u32? = none\
        LET $option TYPE option[[i32]?] = none
        "#,
        20,
    );
    let result_a = parser.parse_let_exp()?;
    let result_b = parser.parse_let_exp()?;
    let result_c = parser.parse_let_exp()?;
    let result_d = parser.parse_let_exp()?;
    let result_e = parser.parse_let_exp()?;

    info!(
        r#"input = {:?} | parse_let_exp parse_let_exp parse_let_exp parse_let_exp parse_let_exp = {:#?} {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d, result_e,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..13,
            kind: Let {
                name: Box::new(Ast {
                    span: 4..8,
                    kind: Variable("age",),
                }),
                r#type: None,
                value: Box::new(Ast {
                    span: 11..13,
                    kind: IntegerLiteral(10,),
                }),
            },
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 23..58,
            kind: Let {
                name: Box::new(Ast {
                    span: 27..33,
                    kind: Variable("array"),
                }),
                r#type: Some(Box::new(Array {
                    r#type: Box::new(Basic(Box::new(Ast {
                        span: 40..43,
                        kind: Identifier("i32"),
                    }))),
                    length: Box::new(Ast {
                        span: 44..45,
                        kind: IntegerLiteral(3),
                    }),
                })),
                value: Box::new(Ast {
                    span: 49..58,
                    kind: ListLiteral(vec![
                        Ast {
                            span: 50..51,
                            kind: IntegerLiteral(1),
                        },
                        Ast {
                            span: 53..54,
                            kind: IntegerLiteral(2),
                        },
                        Ast {
                            span: 56..57,
                            kind: IntegerLiteral(3),
                        },
                    ]),
                }),
            },
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 68..101,
            kind: Let {
                name: Box::new(Ast {
                    span: 72..78,
                    kind: Variable("array"),
                }),
                r#type: Some(Box::new(List(Box::new(Basic(Box::new(Ast {
                    span: 85..88,
                    kind: Identifier("i32"),
                })))))),
                value: Box::new(Ast {
                    span: 92..101,
                    kind: ListLiteral(vec![
                        Ast {
                            span: 93..94,
                            kind: IntegerLiteral(1),
                        },
                        Ast {
                            span: 96..97,
                            kind: IntegerLiteral(2),
                        },
                        Ast {
                            span: 99..100,
                            kind: IntegerLiteral(3),
                        },
                    ]),
                }),
            },
        },)
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 111..139,
            kind: Let {
                name: Box::new(Ast {
                    span: 115..122,
                    kind: Variable("option"),
                }),
                r#type: Some(Box::new(Option(Box::new(Basic(Box::new(Ast {
                    span: 128..131,
                    kind: Identifier("u32"),
                })))))),
                value: Box::new(Ast {
                    span: 135..139,
                    kind: NoneLiteral,
                }),
            },
        },)
    );

    assert_eq!(
        result_e,
        Some(Ast {
            span: 149..187,
            kind: Let {
                name: Box::new(Ast {
                    span: 153..160,
                    kind: Variable("option"),
                }),
                r#type: Some(Box::new(Generic {
                    name: Box::new(Ast {
                        span: 166..172,
                        kind: Identifier("option"),
                    }),
                    parameters: vec![Option(Box::new(List(Box::new(Basic(Box::new(Ast {
                        span: 174..177,
                        kind: Identifier("i32"),
                    }))))))],
                })),
                value: Box::new(Ast {
                    span: 183..187,
                    kind: NoneLiteral,
                }),
            },
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_set_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"SET $var = 10\
        set $var += 10\
        set $var -= 10\
        set $var *= 10\
        set $var /= 10\
        SET $var %= 10\
        set $var &= 10\
        set $var |= 10\
        set $var ^= 10\
        "#,
        20,
    );
    let result_a = parser.parse_set_exp()?;
    let result_b = parser.parse_set_exp()?;
    let result_c = parser.parse_set_exp()?;
    let result_d = parser.parse_set_exp()?;
    let result_e = parser.parse_set_exp()?;
    let result_f = parser.parse_set_exp()?;
    let result_g = parser.parse_set_exp()?;
    let result_h = parser.parse_set_exp()?;
    let result_i = parser.parse_set_exp()?;

    info!(
        r#"input = {:?} | = {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?}"#,
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
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..13,
            kind: Set {
                variable: Box::new(Ast {
                    span: 4..8,
                    kind: Variable("var"),
                }),
                op: Direct,
                value: Box::new(Ast {
                    span: 11..13,
                    kind: IntegerLiteral(10),
                }),
            },
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 23..37,
            kind: Set {
                variable: Box::new(Ast {
                    span: 27..31,
                    kind: Variable("var"),
                }),
                op: Plus,
                value: Box::new(Ast {
                    span: 35..37,
                    kind: IntegerLiteral(10),
                }),
            },
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 47..61,
            kind: Set {
                variable: Box::new(Ast {
                    span: 51..55,
                    kind: Variable("var"),
                }),
                op: Minus,
                value: Box::new(Ast {
                    span: 59..61,
                    kind: IntegerLiteral(10),
                }),
            },
        },)
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 71..85,
            kind: Set {
                variable: Box::new(Ast {
                    span: 75..79,
                    kind: Variable("var"),
                }),
                op: Mul,
                value: Box::new(Ast {
                    span: 83..85,
                    kind: IntegerLiteral(10),
                }),
            },
        },)
    );

    assert_eq!(
        result_e,
        Some(Ast {
            span: 95..109,
            kind: Set {
                variable: Box::new(Ast {
                    span: 99..103,
                    kind: Variable("var"),
                }),
                op: Div,
                value: Box::new(Ast {
                    span: 107..109,
                    kind: IntegerLiteral(10),
                }),
            },
        })
    );

    assert_eq!(
        result_f,
        Some(Ast {
            span: 119..133,
            kind: Set {
                variable: Box::new(Ast {
                    span: 123..127,
                    kind: Variable("var"),
                }),
                op: Mod,
                value: Box::new(Ast {
                    span: 131..133,
                    kind: IntegerLiteral(10),
                }),
            },
        },)
    );

    assert_eq!(
        result_g,
        Some(Ast {
            span: 143..157,
            kind: Set {
                variable: Box::new(Ast {
                    span: 147..151,
                    kind: Variable("var",),
                }),
                op: BitAnd,
                value: Box::new(Ast {
                    span: 155..157,
                    kind: IntegerLiteral(10),
                }),
            },
        })
    );

    assert_eq!(
        result_h,
        Some(Ast {
            span: 167..181,
            kind: Set {
                variable: Box::new(Ast {
                    span: 171..175,
                    kind: Variable("var"),
                }),
                op: BitOr,
                value: Box::new(Ast {
                    span: 179..181,
                    kind: IntegerLiteral(10),
                }),
            },
        })
    );

    assert_eq!(
        result_i,
        Some(Ast {
            span: 191..205,
            kind: Set {
                variable: Box::new(Ast {
                    span: 195..199,
                    kind: Variable("var"),
                }),
                op: BitXor,
                value: Box::new(Ast {
                    span: 203..205,
                    kind: IntegerLiteral(10),
                }),
            },
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_exp() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"RELATE x -> of -> y SET { a: 1, b: 2 }\
        5 + 10..=20
        "#,
        20,
    );
    let result_a = parser.parse_exp()?;
    let result_b = parser.parse_exp()?;

    info!(
        r#"input = {:?} | parse_exp parse_exp = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..38,
            kind: Relate {
                relate_op: Box::new(Ast {
                    span: 7..19,
                    kind: RelateOp {
                        left: Box::new(Ast {
                            span: 7..8,
                            kind: SingleRelateId {
                                subject: Box::new(Ast {
                                    span: 7..8,
                                    kind: Identifier("x",),
                                }),
                                alias: None,
                            },
                        }),
                        l_op: Right,
                        edge: Box::new(Ast {
                            span: 12..14,
                            kind: RelateEdgeId {
                                subject: Box::new(Ast {
                                    span: 12..14,
                                    kind: Identifier("of"),
                                }),
                                depth: None,
                                alias: None,
                            },
                        }),
                        r_op: Right,
                        right: Box::new(Ast {
                            span: 18..19,
                            kind: SingleRelateId {
                                subject: Box::new(Ast {
                                    span: 18..19,
                                    kind: Identifier("y",),
                                }),
                                alias: None,
                            },
                        }),
                    },
                }),
                columns: vec![
                    Ast {
                        span: 26..27,
                        kind: Identifier("a"),
                    },
                    Ast {
                        span: 32..33,
                        kind: Identifier("b"),
                    },
                ],
                value: vec![
                    Ast {
                        span: 29..30,
                        kind: IntegerLiteral(1),
                    },
                    Ast {
                        span: 35..36,
                        kind: IntegerLiteral(2),
                    },
                ],
            },
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 48..59,
            kind: RangeInclusiveOp(
                Box::new(Ast {
                    span: 48..54,
                    kind: AdditionOp(
                        Box::new(Ast {
                            span: 48..49,
                            kind: IntegerLiteral(5,),
                        }),
                        Box::new(Ast {
                            span: 52..54,
                            kind: IntegerLiteral(10,),
                        }),
                    ),
                }),
                Box::new(Ast {
                    span: 57..59,
                    kind: IntegerLiteral(20,),
                }),
            ),
        })
    );

    Ok(())
}
