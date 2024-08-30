use tracing::info;

use crate::{
    ast::{Ast, AstKind::*, Field, TypeSig::*},
    parser::Parser,
};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test]
fn test_parser_partial_if_not_exists() -> anyhow::Result<()> {
    let parser = &mut Parser::new("if not exists IF NOT EXISTS", 20);
    let result_a = parser.parse_partial_if_not_exists()?;
    let result_b = parser.parse_partial_if_not_exists()?;

    info!(
        r#"input = {:?} | parse_partial_if_not_exists parse_partial_if_not_exists = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_define_namespace_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "DEFINE NAMESPACE IF NOT EXIST `ns` define ns my_ns if not exists DEFINE NAMESPACE my_ns",
        20,
    );
    let result_a = parser.parse_define_namespace_stmt()?;
    let result_b = parser.parse_define_namespace_stmt()?;
    let result_c = parser.parse_define_namespace_stmt()?;

    info!(
        r#"input = {:?} | parse_define_namespace_stmt parse_define_namespace_stmt parse_define_namespace_stmt= {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..34,
            kind: DefineNamespace {
                name: Box::new(Ast {
                    span: 30..34,
                    kind: Identifier("ns"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 35..64,
            kind: DefineNamespace {
                name: Box::new(Ast {
                    span: 45..50,
                    kind: Identifier("my_ns"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 65..87,
            kind: DefineNamespace {
                name: Box::new(Ast {
                    span: 82..87,
                    kind: Identifier("my_ns"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
            },
            tag: Default::default(),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_define_database_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "DEFINE DATABASE IF NOT EXIST `db` ON NAMESPACE `ns` define db my_db if not exists DEFINE DATABASE my_db",
        20,
    );
    let result_a = parser.parse_define_database_stmt()?;
    let result_b = parser.parse_define_database_stmt()?;
    let result_c = parser.parse_define_database_stmt()?;

    info!(
        r#"input = {:?} | parse_define_database_stmt parse_define_database_stmt parse_define_database_stmt= {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..51,
            kind: DefineDatabase {
                name: Box::new(Ast {
                    span: 29..33,
                    kind: Identifier("db",),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                namespace: Some(Box::new(Ast {
                    span: 47..51,
                    kind: Identifier("ns",),
                    tag: Default::default(),
                })),
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 52..81,
            kind: DefineDatabase {
                name: Box::new(Ast {
                    span: 62..67,
                    kind: Identifier("my_db",),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                namespace: None,
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 82..103,
            kind: DefineDatabase {
                name: Box::new(Ast {
                    span: 98..103,
                    kind: Identifier("my_db",),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                namespace: None,
            },
            tag: Default::default(),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_partial_field_type() -> anyhow::Result<()> {
    let parser = &mut Parser::new("type result[option[string]] TYPE u8?", 20);
    let result_a = parser.parse_partial_field_type()?;
    let result_b = parser.parse_partial_field_type()?;

    info!(
        r#"input = {:?} | parse_partial_field_type parse_partial_field_type = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_field_value() -> anyhow::Result<()> {
    let parser = &mut Parser::new("value 100 VALUE 0..0x100", 20);
    let result_a = parser.parse_partial_field_value()?;
    let result_b = parser.parse_partial_field_value()?;

    info!(
        r#"input = {:?} | parse_partial_field_value parse_partial_field_value = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_table_field_assert() -> anyhow::Result<()> {
    let parser = &mut Parser::new("assert 100 ASSERT 0..0x100", 20);
    let result_a = parser.parse_partial_table_field_assert()?;
    let result_b = parser.parse_partial_table_field_assert()?;

    info!(
        r#"input = {:?} | parse_partial_table_field_assert parse_partial_table_field_assert = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_table_field() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"name type string? value 100 assert 100 readonly unique\
        val ASSERT val > 10 UNIQUE TYPE u8
        "#,
        20,
    );
    let result_a = parser.parse_partial_table_field()?;
    let result_b = parser.parse_partial_table_field()?;

    info!(
        r#"input = {:?} | parse_partial_table_field parse_partial_table_field = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_table_fields() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"FIELDS name type string? value 100 assert 100 readonly unique, val ASSERT $value > 10 UNIQUE TYPE u8\
        FIELDS name TYPE string, age TYPE u8"#,
        20,
    );
    let result_a = parser.parse_partial_table_fields()?;
    let result_b = parser.parse_partial_table_fields()?;

    info!(
        r#"input = {:?} | parse_partial_table_fields parse_partial_table_fields = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_define_table_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"DEFINE TABLE `table` ON DATABASE `db` FIELDS name TYPE string?, age ASSERT $value > 10 ASSERT $value < 100 TYPE u8? UNIQUE READONLY IF NOT EXISTS\
        DEFINE TABLE my_table\
        DEFINE TABLE IF NOT EXISTS `table` ON DATABASE `db`\
        DEFINE TABLE `table` ON DATABASE `db` FIELDS name TYPE string?\
        "#,
        20,
    );
    let result_a = parser.parse_define_table_stmt()?;
    let result_b = parser.parse_define_table_stmt()?;
    let result_c = parser.parse_define_table_stmt()?;
    let result_d = parser.parse_define_table_stmt()?;

    info!(
        r#"input = {:?} | parse_define_table_stmt parse_define_table_stmt parse_define_table_stmt parse_define_table_stmt = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..145,
            kind: DefineTable {
                name: Box::new(Ast {
                    span: 13..20,
                    kind: Identifier("table"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: Some(Box::new(Ast {
                    span: 33..37,
                    kind: Identifier("db"),
                    tag: Default::default(),
                })),
                fields: vec![
                    Field {
                        name: Box::new(Ast {
                            span: 45..49,
                            kind: Identifier("name"),
                            tag: Default::default(),
                        }),
                        r#type: Option(Box::new(Basic(Box::new(Ast {
                            span: 55..61,
                            kind: Identifier("string"),
                            tag: Default::default(),
                        })))),
                        default: None,
                        assertions: vec![],
                        readonly: false,
                        unique: false,
                    },
                    Field {
                        name: Box::new(Ast {
                            span: 64..67,
                            kind: Identifier("age",),
                            tag: Default::default(),
                        }),
                        r#type: Option(Box::new(Basic(Box::new(Ast {
                            span: 112..114,
                            kind: Identifier("u8"),
                            tag: Default::default(),
                        })))),
                        default: None,
                        assertions: vec![
                            Ast {
                                span: 75..86,
                                kind: GreaterThanOp(
                                    Box::new(Ast {
                                        span: 75..81,
                                        kind: Variable("value"),
                                        tag: Default::default(),
                                    }),
                                    Box::new(Ast {
                                        span: 84..86,
                                        kind: IntegerLiteral(10),
                                        tag: Default::default(),
                                    }),
                                ),
                                tag: Default::default(),
                            },
                            Ast {
                                span: 94..106,
                                kind: LessThanOp(
                                    Box::new(Ast {
                                        span: 94..100,
                                        kind: Variable("value"),
                                        tag: Default::default(),
                                    }),
                                    Box::new(Ast {
                                        span: 103..106,
                                        kind: IntegerLiteral(100),
                                        tag: Default::default(),
                                    }),
                                ),
                                tag: Default::default(),
                            },
                        ],
                        readonly: true,
                        unique: true,
                    },
                ],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 155..176,
            kind: DefineTable {
                name: Box::new(Ast {
                    span: 168..176,
                    kind: Identifier("my_table"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: None,
                fields: vec![],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 186..237,
            kind: DefineTable {
                name: Box::new(Ast {
                    span: 213..220,
                    kind: Identifier("table"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: Some(Box::new(Ast {
                    span: 233..237,
                    kind: Identifier("db"),
                    tag: Default::default(),
                })),
                fields: vec![],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 247..309,
            kind: DefineTable {
                name: Box::new(Ast {
                    span: 260..267,
                    kind: Identifier("table"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: Some(Box::new(Ast {
                    span: 280..284,
                    kind: Identifier("db"),
                    tag: Default::default(),
                }),),
                fields: vec![Field {
                    name: Box::new(Ast {
                        span: 292..296,
                        kind: Identifier("name"),
                        tag: Default::default(),
                    }),
                    r#type: Option(Box::new(Basic(Box::new(Ast {
                        span: 302..308,
                        kind: Identifier("string"),
                        tag: Default::default(),
                    })))),
                    default: None,
                    assertions: vec![],
                    readonly: false,
                    unique: false,
                }],
            },
            tag: Default::default(),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_define_edge_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"DEFINE EDGE `edge` ON DATABASE `db` FIELDS name TYPE string?, age ASSERT $value > 10 ASSERT $value < 100 TYPE u8? UNIQUE READONLY IF NOT EXISTS\
        DEFINE EDGE my_edge\
        DEFINE EDGE IF NOT EXISTS `edge` ON DATABASE `db`\
        DEFINE EDGE `edge` ON DATABASE `db` FIELDS name TYPE string?\
        "#,
        20,
    );
    let result_a = parser.parse_define_edge_stmt()?;
    let result_b = parser.parse_define_edge_stmt()?;
    let result_c = parser.parse_define_edge_stmt()?;
    let result_d = parser.parse_define_edge_stmt()?;

    info!(
        r#"input = {:?} | parse_define_edge_stmt parse_define_edge_stmt parse_define_edge_stmt parse_define_edge_stmt = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..143,
            kind: DefineEdge {
                name: Box::new(Ast {
                    span: 12..18,
                    kind: Identifier("edge"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: Some(Box::new(Ast {
                    span: 31..35,
                    kind: Identifier("db"),
                    tag: Default::default(),
                })),
                fields: vec![
                    Field {
                        name: Box::new(Ast {
                            span: 43..47,
                            kind: Identifier("name"),
                            tag: Default::default(),
                        }),
                        r#type: Option(Box::new(Basic(Box::new(Ast {
                            span: 53..59,
                            kind: Identifier("string"),
                            tag: Default::default(),
                        })))),
                        default: None,
                        assertions: vec![],
                        readonly: false,
                        unique: false,
                    },
                    Field {
                        name: Box::new(Ast {
                            span: 62..65,
                            kind: Identifier("age"),
                            tag: Default::default(),
                        }),
                        r#type: Option(Box::new(Basic(Box::new(Ast {
                            span: 110..112,
                            kind: Identifier("u8"),
                            tag: Default::default(),
                        })))),
                        default: None,
                        assertions: vec![
                            Ast {
                                span: 73..84,
                                kind: GreaterThanOp(
                                    Box::new(Ast {
                                        span: 73..79,
                                        kind: Variable("value"),
                                        tag: Default::default(),
                                    }),
                                    Box::new(Ast {
                                        span: 82..84,
                                        kind: IntegerLiteral(10),
                                        tag: Default::default(),
                                    }),
                                ),
                                tag: Default::default(),
                            },
                            Ast {
                                span: 92..104,
                                kind: LessThanOp(
                                    Box::new(Ast {
                                        span: 92..98,
                                        kind: Variable("value"),
                                        tag: Default::default(),
                                    }),
                                    Box::new(Ast {
                                        span: 101..104,
                                        kind: IntegerLiteral(100),
                                        tag: Default::default(),
                                    }),
                                ),
                                tag: Default::default(),
                            },
                        ],
                        readonly: true,
                        unique: true,
                    },
                ],
            },
            tag: Default::default(),
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 153..172,
            kind: DefineEdge {
                name: Box::new(Ast {
                    span: 165..172,
                    kind: Identifier("my_edge"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: None,
                fields: vec![],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 182..231,
            kind: DefineEdge {
                name: Box::new(Ast {
                    span: 208..214,
                    kind: Identifier("edge"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: Some(Box::new(Ast {
                    span: 227..231,
                    kind: Identifier("db"),
                    tag: Default::default(),
                })),
                fields: vec![],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 241..301,
            kind: DefineEdge {
                name: Box::new(Ast {
                    span: 253..259,
                    kind: Identifier("edge"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: Some(Box::new(Ast {
                    span: 272..276,
                    kind: Identifier("db"),
                    tag: Default::default(),
                })),
                fields: vec![Field {
                    name: Box::new(Ast {
                        span: 284..288,
                        kind: Identifier("name",),
                        tag: Default::default(),
                    }),
                    r#type: Option(Box::new(Basic(Box::new(Ast {
                        span: 294..300,
                        kind: Identifier("string"),
                        tag: Default::default(),
                    })))),
                    default: None,
                    assertions: vec![],
                    readonly: false,
                    unique: false,
                }],
            },
            tag: Default::default(),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_partial_type_field() -> anyhow::Result<()> {
    let parser = &mut Parser::new("name TYPE string? age TYPE option[u8]", 20);
    let result_a = parser.parse_partial_type_field()?;
    let result_b = parser.parse_partial_type_field()?;

    info!(
        r#"input = {:?} | parse_partial_type_field parse_partial_type_field = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_type_fields() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        "FIELDS name TYPE string?, age TYPE option[u8] FIELDS name TYPE string?",
        20,
    );
    let result_a = parser.parse_partial_type_fields()?;
    let result_b = parser.parse_partial_type_fields()?;

    info!(
        r#"input = {:?} | parse_partial_type_fields parse_partial_type_fields = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_define_type_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"DEFINE TYPE `type` FIELDS name TYPE string?, age TYPE option[u8] IF NOT EXISTS\
        DEFINE TYPE my_type\
        DEFINE TYPE IF NOT EXISTS `type`\
        DEFINE TYPE `type` FIELDS name TYPE string?\
        "#,
        20,
    );
    let result_a = parser.parse_define_type_stmt()?;
    let result_b = parser.parse_define_type_stmt()?;
    let result_c = parser.parse_define_type_stmt()?;
    let result_d = parser.parse_define_type_stmt()?;

    info!(
        r#"input = {:?} | parse_define_type_stmt parse_define_type_stmt parse_define_type_stmt parse_define_type_stmt = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..78,
            kind: DefineType {
                name: Box::new(Ast {
                    span: 12..18,
                    kind: Identifier("type"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: None,
                fields: vec![
                    (
                        Ast {
                            span: 26..30,
                            kind: Identifier("name"),
                            tag: Default::default(),
                        },
                        Option(Box::new(Basic(Box::new(Ast {
                            span: 36..42,
                            kind: Identifier("string"),
                            tag: Default::default(),
                        })))),
                    ),
                    (
                        Ast {
                            span: 45..48,
                            kind: Identifier("age"),
                            tag: Default::default(),
                        },
                        Generic {
                            name: Box::new(Ast {
                                span: 54..60,
                                kind: Identifier("option"),
                                tag: Default::default(),
                            }),
                            parameters: vec![Basic(Box::new(Ast {
                                span: 61..63,
                                kind: Identifier("u8"),
                                tag: Default::default(),
                            }))],
                        },
                    ),
                ],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 88..107,
            kind: DefineType {
                name: Box::new(Ast {
                    span: 100..107,
                    kind: Identifier("my_type"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: None,
                fields: vec![],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 117..149,
            kind: DefineType {
                name: Box::new(Ast {
                    span: 143..149,
                    kind: Identifier("type"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: None,
                fields: vec![],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 159..202,
            kind: DefineType {
                name: Box::new(Ast {
                    span: 171..177,
                    kind: Identifier("type"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: None,
                fields: vec![(
                    Ast {
                        span: 185..189,
                        kind: Identifier("name"),
                        tag: Default::default(),
                    },
                    Option(Box::new(Basic(Box::new(Ast {
                        span: 195..201,
                        kind: Identifier("string"),
                        tag: Default::default(),
                    })))),
                )],
            },
            tag: Default::default(),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_partial_enum_variants() -> anyhow::Result<()> {
    let parser = &mut Parser::new("VARIANTS a, b, c variants a, b, c", 20);
    let result_a = parser.parse_partial_enum_variants()?;
    let result_b = parser.parse_partial_enum_variants()?;

    info!(
        r#"input = {:?} | parse_partial_enum_variants parse_partial_enum_variants = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_define_enum_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"DEFINE ENUM `enum` VARIANTS a, b, c IF NOT EXISTS\
        DEFINE ENUM my_enum\
        DEFINE ENUM IF NOT EXISTS `enum`\
        DEFINE ENUM `enum` VARIANTS a, b, c\
        "#,
        20,
    );
    let result_a = parser.parse_define_enum_stmt()?;
    let result_b = parser.parse_define_enum_stmt()?;
    let result_c = parser.parse_define_enum_stmt()?;
    let result_d = parser.parse_define_enum_stmt()?;

    info!(
        r#"input = {:?} | parse_define_enum_stmt parse_define_enum_stmt parse_define_enum_stmt parse_define_enum_stmt = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..49,
            kind: DefineEnum {
                name: Box::new(Ast {
                    span: 12..18,
                    kind: Identifier("enum"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: None,
                variants: vec![
                    Ast {
                        span: 28..29,
                        kind: Identifier("a"),
                        tag: Default::default(),
                    },
                    Ast {
                        span: 31..32,
                        kind: Identifier("b"),
                        tag: Default::default(),
                    },
                    Ast {
                        span: 34..35,
                        kind: Identifier("c"),
                        tag: Default::default(),
                    },
                ],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 59..78,
            kind: DefineEnum {
                name: Box::new(Ast {
                    span: 71..78,
                    kind: Identifier("my_enum"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: None,
                variants: vec![],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 88..120,
            kind: DefineEnum {
                name: Box::new(Ast {
                    span: 114..120,
                    kind: Identifier("enum"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: None,
                variants: vec![],
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 130..165,
            kind: DefineEnum {
                name: Box::new(Ast {
                    span: 142..148,
                    kind: Identifier("enum"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: None,
                variants: vec![
                    Ast {
                        span: 158..159,
                        kind: Identifier("a"),
                        tag: Default::default(),
                    },
                    Ast {
                        span: 161..162,
                        kind: Identifier("b"),
                        tag: Default::default(),
                    },
                    Ast {
                        span: 164..165,
                        kind: Identifier("c"),
                        tag: Default::default(),
                    },
                ],
            },
            tag: Default::default(),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_partial_index_fields() -> anyhow::Result<()> {
    let parser = &mut Parser::new("FIELDS name, age fields name", 20);
    let result_a = parser.parse_partial_index_fields()?;
    let result_b = parser.parse_partial_index_fields()?;

    info!(
        r#"input = {:?} | parse_partial_index_fields parse_partial_index_fields = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_partial_index_with() -> anyhow::Result<()> {
    let parser = &mut Parser::new("WITH std::foo(a = $value) with foo($value)", 20);
    let result_a = parser.parse_partial_index_with()?;
    let result_b = parser.parse_partial_index_with()?;

    info!(
        r#"input = {:?} | parse_partial_index_with parse_partial_index_with = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert!(result_a.is_some());
    assert!(result_b.is_some());

    Ok(())
}

#[test_log::test]
fn test_parser_define_index_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"DEFINE INDEX `index` FIELDS name, age WITH std::foo(a = $value) ON TABLE `table` ON DATABASE `database` \
        DEFINE INDEX IF NOT EXISTS `index` ON TABLE `table` FIELDS name\
        "#,
        20,
    );
    let result_a = parser.parse_define_index_stmt()?;
    let result_b = parser.parse_define_index_stmt()?;

    info!(
        r#"input = {:?} | parse_define_index_stmt parse_define_index_stmt = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..103,
            kind: DefineIndex {
                name: Box::new(Ast {
                    span: 13..20,
                    kind: Identifier("index"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: Some(Box::new(Ast {
                    span: 93..103,
                    kind: Identifier("database"),
                    tag: Default::default(),
                })),
                table: Box::new(Ast {
                    span: 73..80,
                    kind: Identifier("table"),
                    tag: Default::default(),
                }),
                columns: vec![
                    Ast {
                        span: 28..32,
                        kind: Identifier("name"),
                        tag: Default::default(),
                    },
                    Ast {
                        span: 34..37,
                        kind: Identifier("age"),
                        tag: Default::default(),
                    },
                ],
                function: Some(Box::new(Ast {
                    span: 43..63,
                    kind: FunctionCall {
                        subject: Box::new(Ast {
                            span: 43..51,
                            kind: ScopedIdentifier(vec![
                                Ast {
                                    span: 43..46,
                                    kind: Identifier("std"),
                                    tag: Default::default(),
                                },
                                Ast {
                                    span: 48..51,
                                    kind: Identifier("foo"),
                                    tag: Default::default(),
                                },
                            ]),
                            tag: Default::default(),
                        }),
                        args: vec![Ast {
                            span: 52..62,
                            kind: FunctionArg {
                                name: Some(Box::new(Ast {
                                    span: 52..53,
                                    kind: Identifier("a"),
                                    tag: Default::default(),
                                })),
                                value: Box::new(Ast {
                                    span: 56..62,
                                    kind: Variable("value"),
                                    tag: Default::default(),
                                }),
                            },
                            tag: Default::default(),
                        }],
                    },
                    tag: Default::default(),
                })),
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 114..177,
            kind: DefineIndex {
                name: Box::new(Ast {
                    span: 141..148,
                    kind: Identifier("index"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: None,
                table: Box::new(Ast {
                    span: 158..165,
                    kind: Identifier("table"),
                    tag: Default::default(),
                }),
                columns: vec![Ast {
                    span: 173..177,
                    kind: Identifier("name"),
                    tag: Default::default(),
                }],
                function: None,
            },
            tag: Default::default(),
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_define_module_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"DEFINE MODULE m WITH\
        function add(a, b) {
            return a + b;
        }
        END IF NOT EXISTS\
        DEFINE MODULE m WITH function foo() { return 20; } END\
        "#,
        20,
    );
    let result_a = parser.parse_define_module_stmt()?;
    let result_b = parser.parse_define_module_stmt()?;

    info!(
        r#"input = {:?} | parse_define_index_stmt parse_define_index_stmt = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert_eq!(result_a, Some(
        Ast {
            span: 0..112,
            kind: DefineModule {
                name: Box::new(Ast {
                    span: 14..15,
                    kind: Identifier("m"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: None,
                block: Box::new(Ast{
                    span: 20..95,
                    kind: ModuleBlock("\\\n        function add(a, b) {\n            return a + b;\n        }\n        ",),
                    tag: Default::default(),
                }),
            },
            tag: Default::default(),
        },
    ));

    assert_eq!(
        result_b,
        Some(Ast {
            span: 122..176,
            kind: DefineModule {
                name: Box::new(Ast {
                    span: 136..137,
                    kind: Identifier("m"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: None,
                block: Box::new(Ast {
                    span: 142..173,
                    kind: ModuleBlock(" function foo() { return 20; } "),
                    tag: Default::default(),
                }),
            },
            tag: Default::default(),
        },)
    );

    Ok(())
}

#[test_log::test]
fn test_parser_define_param_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"DEFINE PARAM p TYPE string? VALUE "hello" ON DATABASE my_db IF NOT EXISTS\
        DEFINE PARAM p VALUE "world"
        "#,
        20,
    );
    let result_a = parser.parse_define_param_stmt()?;
    let result_b = parser.parse_define_param_stmt()?;

    info!(
        r#"input = {:?} | parse_define_index_stmt parse_define_index_stmt = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..73,
            kind: DefineParam {
                name: Box::new(Ast {
                    span: 13..14,
                    kind: Identifier("p"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
                database: Some(Box::new(Ast {
                    span: 54..59,
                    kind: Identifier("my_db"),
                    tag: Default::default(),
                })),
                r#type: Some(Option(Box::new(Basic(Box::new(Ast {
                    span: 20..26,
                    kind: Identifier("string"),
                    tag: Default::default(),
                }))))),
                value: Box::new(Ast {
                    span: 34..41,
                    kind: StringLiteral("hello"),
                    tag: Default::default(),
                }),
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 83..111,
            kind: DefineParam {
                name: Box::new(Ast {
                    span: 96..97,
                    kind: Identifier("p"),
                    tag: Default::default(),
                }),
                if_not_exists: false,
                database: None,
                r#type: None,
                value: Box::new(Ast {
                    span: 104..111,
                    kind: StringLiteral("world"),
                    tag: Default::default(),
                }),
            },
            tag: Default::default(),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_define_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"DEFINE NAMESPACE IF NOT EXIST `ns`\
        DEFINE DATABASE IF NOT EXIST `db` ON NAMESPACE `ns`\
        DEFINE TABLE `table` ON DATABASE `db` FIELDS name TYPE string?, age ASSERT $value > 10 ASSERT $value < 100 TYPE u8? UNIQUE READONLY IF NOT EXISTS\
        DEFINE EDGE `edge` ON DATABASE `db` FIELDS name TYPE string?, age ASSERT $value > 10 ASSERT $value < 100 TYPE u8? UNIQUE READONLY IF NOT EXISTS\
        DEFINE TYPE `type` FIELDS name TYPE string?, age TYPE option[u8] IF NOT EXISTS\
        DEFINE ENUM `enum` ON DATABASE `database` VARIANTS a, b, c IF NOT EXISTS\
        DEFINE INDEX `index` FIELDS name, age WITH std::foo(a = $value) ON TABLE `table` ON DATABASE `database`\
        DEFINE MODULE m IF NOT EXIST WITH function foo() { return 20; } END ON DATABASE `database`\
        DEFINE PARAM p TYPE string? VALUE "hello" ON DATABASE my_db IF NOT EXISTS
        "#,
        20,
    );
    let result_a = parser.parse_define_stmt()?;
    let result_b = parser.parse_define_stmt()?;
    let result_c = parser.parse_define_stmt()?;
    let result_d = parser.parse_define_stmt()?;
    let result_e = parser.parse_define_stmt()?;
    let result_f = parser.parse_define_stmt()?;
    let result_g = parser.parse_define_stmt()?;
    let result_h = parser.parse_define_stmt()?;
    let result_i = parser.parse_define_stmt()?;

    info!(
        r#"input = {:?} | parse_define_stmt = {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?}"#,
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
fn test_parser_use_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new("USE DATABASE `db` use db my_db", 20);
    let result_a = parser.parse_use_stmt()?;
    let result_b = parser.parse_use_stmt()?;

    info!(
        r#"input = {:?} | parse_use_stmt parse_use_stmt  = {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..17,
            kind: Use {
                database: Box::new(Ast {
                    span: 13..17,
                    kind: Identifier("db"),
                    tag: Default::default(),
                }),
            },
            tag: Default::default(),
        },)
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 18..30,
            kind: Use {
                database: Box::new(Ast {
                    span: 25..30,
                    kind: Identifier("my_db"),
                    tag: Default::default(),
                }),
            },
            tag: Default::default(),
        })
    );

    Ok(())
}

#[test_log::test]
fn test_parser_stmt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"DEFINE NAMESPACE `ns` IF NOT EXISTS\
        use db d\
        break\
        continue\
        "#,
        20,
    );
    let result_a = parser.parse_stmt()?;
    let result_b = parser.parse_stmt()?;
    let result_c = parser.parse_stmt()?;
    let result_d = parser.parse_stmt()?;

    info!(
        r#"input = {:?} | parse_stmt parse_stmt parse_stmt parse_stmt = {:#?} {:#?} {:#?} {:#?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d,
    );

    assert_eq!(
        result_a,
        Some(Ast {
            span: 0..35,
            kind: DefineNamespace {
                name: Box::new(Ast {
                    span: 17..21,
                    kind: Identifier("ns"),
                    tag: Default::default(),
                }),
                if_not_exists: true,
            },
            tag: Default::default(),
        })
    );

    assert_eq!(
        result_b,
        Some(Ast {
            span: 45..53,
            kind: Use {
                database: Box::new(Ast {
                    span: 52..53,
                    kind: Identifier("d",),
                    tag: Default::default(),
                }),
            },
            tag: Default::default(),
        },)
    );

    assert_eq!(
        result_c,
        Some(Ast {
            span: 63..68,
            kind: Break,
            tag: Default::default(),
        },)
    );

    assert_eq!(
        result_d,
        Some(Ast {
            span: 78..86,
            kind: Continue,
            tag: Default::default(),
        },)
    );

    Ok(())
}
