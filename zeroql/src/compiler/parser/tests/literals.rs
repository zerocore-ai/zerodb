use tracing::info;

use crate::{
    ast::{Ast, AstKind},
    lexer::RegexFlags,
    parser::Parser,
};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test]
fn test_parser_identifier() -> anyhow::Result<()> {
    let parser = &mut Parser::new("_0world `_0world`", 10);
    let result_a = parser.parse_identifier()?;
    let result_b = parser.parse_identifier()?;

    info!(
        "input = {:?} | parse_identifier parse_identifier = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );

    assert_eq!(
        result_a,
        Some(Ast::new(0..7, AstKind::Identifier("_0world")))
    );
    assert_eq!(
        result_b,
        Some(Ast::new(8..17, AstKind::Identifier("_0world")))
    );

    Ok(())
}

#[test_log::test]
fn test_parser_variable() -> anyhow::Result<()> {
    let parser = &mut Parser::new("$_0world", 10);
    let result = parser.parse_variable()?;

    info!(
        "input = {:?} | parse_variable = {:?}",
        parser.lexer.string, result
    );

    assert_eq!(result, Some(Ast::new(0..8, AstKind::Variable("_0world"))));

    Ok(())
}

#[test_log::test]
fn test_parser_boolean_lit() -> anyhow::Result<()> {
    let parser = &mut Parser::new("true TRUE FALSE false", 10);
    let result_a = parser.parse_boolean_lit()?;
    let result_b = parser.parse_boolean_lit()?;
    let result_c = parser.parse_boolean_lit()?;
    let result_d = parser.parse_boolean_lit()?;

    info!(
        "input = {:?} | parse_boolean_lit parse_boolean_lit parse_boolean_lit parse_boolean_lit = {:?} {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c, result_d
    );

    assert_eq!(
        result_a,
        Some(Ast::new(0..4, AstKind::BooleanLiteral(true)))
    );

    assert_eq!(
        result_b,
        Some(Ast::new(5..9, AstKind::BooleanLiteral(true)))
    );

    assert_eq!(
        result_c,
        Some(Ast::new(10..15, AstKind::BooleanLiteral(false)))
    );

    assert_eq!(
        result_d,
        Some(Ast::new(16..21, AstKind::BooleanLiteral(false)))
    );

    Ok(())
}

#[test_log::test]
fn test_parser_none_lit() -> anyhow::Result<()> {
    let parser = &mut Parser::new("none NONE", 10);
    let result_a = parser.parse_none_lit()?;
    let result_b = parser.parse_none_lit()?;

    info!(
        "input = {:?} | parse_none_lit parse_none_lit = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );

    assert_eq!(result_a, Some(Ast::new(0..4, AstKind::NoneLiteral)));
    assert_eq!(result_b, Some(Ast::new(5..9, AstKind::NoneLiteral)));

    Ok(())
}

#[test_log::test]
fn test_parser_raw_lit() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"\
        0b0_111 \
        0o01234567 \
        0x01_23_45_67_89_ab_cd_ef \
        0_123_456_789 \
        0_123. \
        .0_123 \
        0_12.3E0_123 \
        0.1e0 \
        'Hello, World!' \
        //[a-zA-Z_][a-zA-Z0-9_]*//xmig \
        b"Hello, World!"\
        TRUE \
        NONE \
        "#,
        10,
    );

    let result_a = parser.parse_raw_lit()?;
    let result_b = parser.parse_raw_lit()?;
    let result_c = parser.parse_raw_lit()?;
    let result_d = parser.parse_raw_lit()?;
    let result_e = parser.parse_raw_lit()?;
    let result_f = parser.parse_raw_lit()?;
    let result_g = parser.parse_raw_lit()?;
    let result_h = parser.parse_raw_lit()?;
    let result_i = parser.parse_raw_lit()?;
    let result_j = parser.parse_raw_lit()?;
    let result_k = parser.parse_raw_lit()?;
    let result_l = parser.parse_raw_lit()?;
    let result_m = parser.parse_raw_lit()?;

    info!(
        "input = {:?} | {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
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
        result_m
    );

    assert_eq!(
        result_a,
        Some(Ast::new(10..17, AstKind::IntegerLiteral(0b0_111)))
    );

    assert_eq!(
        result_b,
        Some(Ast::new(28..38, AstKind::IntegerLiteral(0o01234567)))
    );

    assert_eq!(
        result_c,
        Some(Ast::new(
            49..74,
            AstKind::IntegerLiteral(0x01_23_45_67_89_ab_cd_ef)
        ))
    );

    assert_eq!(
        result_d,
        Some(Ast::new(85..98, AstKind::IntegerLiteral(0_123_456_789)))
    );

    assert_eq!(
        result_e,
        Some(Ast::new(109..115, AstKind::FloatLiteral(0_123.0)))
    );

    assert_eq!(
        result_f,
        Some(Ast::new(126..132, AstKind::FloatLiteral(0.0_123)))
    );

    assert_eq!(
        result_g,
        Some(Ast::new(143..155, AstKind::FloatLiteral(0_12.30E0_123)))
    );

    assert_eq!(
        result_h,
        Some(Ast::new(166..171, AstKind::FloatLiteral(0.1)))
    );

    assert_eq!(
        result_i,
        Some(Ast::new(
            182..197,
            AstKind::StringLiteral(r#"Hello, World!"#)
        ))
    );

    assert_eq!(
        result_j,
        Some(Ast::new(
            208..238,
            AstKind::RegexLiteral {
                pattern: r#"[a-zA-Z_][a-zA-Z0-9_]*"#,
                flags: RegexFlags::X_EXTENDED
                    | RegexFlags::I_IGNORE_CASE
                    | RegexFlags::M_MULTILINE
                    | RegexFlags::G_GLOBAL,
            }
        ))
    );

    assert_eq!(
        result_k,
        Some(Ast::new(
            249..265,
            AstKind::ByteStringLiteral(r#"Hello, World!"#)
        ))
    );

    assert_eq!(
        result_l,
        Some(Ast::new(275..279, AstKind::BooleanLiteral(true)))
    );

    assert_eq!(result_m, Some(Ast::new(290..294, AstKind::NoneLiteral)));

    Ok(())
}

#[test_log::test]
fn test_parser_list_lit() -> anyhow::Result<()> {
    // TODO: Need more op examples
    // TODO: Need nested examples
    let parser = &mut Parser::new(
        r#"[] [1] [1, "Hello",] [.1, NONE, //[a-zA-Z_][a-zA-Z0-9_]*//xmig]"#,
        10,
    );
    let result_a = parser.parse_list_lit()?;
    let result_b = parser.parse_list_lit()?;
    let result_c = parser.parse_list_lit()?;
    let result_d = parser.parse_list_lit()?;

    info!(
        "input = {:?} | parse_list_lit parse_list_lit parse_list_lit parse_list_lit = {:?} {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c, result_d
    );

    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::ListLiteral(vec![]))));
    assert_eq!(
        result_b,
        Some(Ast::new(
            3..6,
            AstKind::ListLiteral(vec![Ast::new(4..5, AstKind::IntegerLiteral(1))])
        ))
    );
    assert_eq!(
        result_c,
        Some(Ast::new(
            7..20,
            AstKind::ListLiteral(vec![
                Ast::new(8..9, AstKind::IntegerLiteral(1)),
                Ast::new(11..18, AstKind::StringLiteral("Hello"))
            ])
        ))
    );
    assert_eq!(
        result_d,
        Some(Ast::new(
            21..63,
            AstKind::ListLiteral(vec![
                Ast::new(22..24, AstKind::FloatLiteral(0.1)),
                Ast::new(26..30, AstKind::NoneLiteral),
                Ast::new(
                    32..62,
                    AstKind::RegexLiteral {
                        pattern: r#"[a-zA-Z_][a-zA-Z0-9_]*"#,
                        flags: RegexFlags::X_EXTENDED
                            | RegexFlags::I_IGNORE_CASE
                            | RegexFlags::M_MULTILINE
                            | RegexFlags::G_GLOBAL,
                    }
                )
            ])
        ))
    );

    Ok(())
}

#[test_log::test]
fn test_parser_object_lit() -> anyhow::Result<()> {
    // TODO: Need more op examples
    // TODO: Need nested examples
    let parser = &mut Parser::new(
        r#"{} {a: 1,} {a: 1, b: "Hello", c: true} {a: 1, b: //[a-zA-Z_][a-zA-Z0-9_]*//xmig, c: none,}"#,
        10,
    );
    let result_a = parser.parse_object_lit()?;
    let result_b = parser.parse_object_lit()?;
    let result_c = parser.parse_object_lit()?;
    let result_d = parser.parse_object_lit()?;

    info!(
        "input = {:?} | parse_object_lit parse_object_lit parse_object_lit parse_object_lit = {:?} {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c, result_d
    );

    assert_eq!(
        result_a,
        Some(Ast::new(0..2, AstKind::ObjectLiteral(vec![])))
    );
    assert_eq!(
        result_b,
        Some(Ast::new(
            3..10,
            AstKind::ObjectLiteral(vec![(
                Ast::new(4..5, AstKind::Identifier("a")),
                Ast::new(7..8, AstKind::IntegerLiteral(1))
            )])
        ))
    );
    assert_eq!(
        result_c,
        Some(Ast::new(
            11..38,
            AstKind::ObjectLiteral(vec![
                (
                    Ast::new(12..13, AstKind::Identifier("a")),
                    Ast::new(15..16, AstKind::IntegerLiteral(1)),
                ),
                (
                    Ast::new(18..19, AstKind::Identifier("b")),
                    Ast::new(21..28, AstKind::StringLiteral("Hello")),
                ),
                (
                    Ast::new(30..31, AstKind::Identifier("c")),
                    Ast::new(33..37, AstKind::BooleanLiteral(true)),
                )
            ])
        ))
    );
    assert_eq!(
        result_d,
        Some(Ast::new(
            39..90,
            AstKind::ObjectLiteral(vec![
                (
                    Ast::new(40..41, AstKind::Identifier("a")),
                    Ast::new(43..44, AstKind::IntegerLiteral(1))
                ),
                (
                    Ast::new(46..47, AstKind::Identifier("b")),
                    Ast::new(
                        49..79,
                        AstKind::RegexLiteral {
                            pattern: r#"[a-zA-Z_][a-zA-Z0-9_]*"#,
                            flags: RegexFlags::X_EXTENDED
                                | RegexFlags::I_IGNORE_CASE
                                | RegexFlags::M_MULTILINE
                                | RegexFlags::G_GLOBAL,
                        }
                    )
                ),
                (
                    Ast::new(81..82, AstKind::Identifier("c")),
                    Ast::new(84..88, AstKind::NoneLiteral)
                )
            ])
        ))
    );

    Ok(())
}

#[test_log::test]
fn test_parser_tuple_lit() -> anyhow::Result<()> {
    // TODO: Need more op examples
    // TODO: Need nested examples
    let parser = &mut Parser::new(r#"() (1,) (0b1011, "Hello",) (1., "Hello", true,)"#, 10);
    let result_a = parser.parse_tuple_lit()?;
    let result_b = parser.parse_tuple_lit()?;
    let result_c = parser.parse_tuple_lit()?;
    let result_d = parser.parse_tuple_lit()?;

    info!(
        "input = {:?} | parse_tuple_lit parse_tuple_lit parse_tuple_lit parse_tuple_lit = {:?} {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c, result_d
    );

    assert_eq!(
        result_a,
        Some(Ast::new(0..2, AstKind::TupleLiteral(vec![])))
    );
    assert_eq!(
        result_b,
        Some(Ast::new(
            3..7,
            AstKind::TupleLiteral(vec![Ast::new(4..5, AstKind::IntegerLiteral(1))])
        ))
    );
    assert_eq!(
        result_c,
        Some(Ast::new(
            8..26,
            AstKind::TupleLiteral(vec![
                Ast::new(9..15, AstKind::IntegerLiteral(0b1011)),
                Ast::new(17..24, AstKind::StringLiteral("Hello"))
            ])
        ))
    );
    assert_eq!(
        result_d,
        Some(Ast::new(
            27..47,
            AstKind::TupleLiteral(vec![
                Ast::new(28..30, AstKind::FloatLiteral(1.0)),
                Ast::new(32..39, AstKind::StringLiteral("Hello")),
                Ast::new(41..45, AstKind::BooleanLiteral(true)),
            ])
        ))
    );

    // Fail Cases
    let parser = &mut Parser::new("(1)", 10);
    let result = parser.parse_tuple_lit()?;

    info!(
        "input = {:?} | parse_tuple_lit = {:?}",
        parser.lexer.string, result
    );

    assert_eq!(result, None);

    Ok(())
}

#[test_log::test]
fn test_parser_lit() -> anyhow::Result<()> {
    let parser = &mut Parser::new(
        r#"0x0123456789abcdef [1,] (true, //.+//gimsux,) {a: NONE,}"#,
        10,
    );
    let result_a = parser.parse_lit()?;
    let result_b = parser.parse_lit()?;
    let result_c = parser.parse_lit()?;
    let result_d = parser.parse_lit()?;

    info!(
        "input = {:?} | parse_lit parse_lit parse_lit parse_lit parse_lit = {:?} {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c, result_d
    );

    assert_eq!(
        result_a,
        Some(Ast::new(0..18, AstKind::IntegerLiteral(0x0123456789abcdef)))
    );
    assert_eq!(
        result_b,
        Some(Ast::new(
            19..23,
            AstKind::ListLiteral(vec![Ast::new(20..21, AstKind::IntegerLiteral(1))])
        ))
    );
    assert_eq!(
        result_c,
        Some(Ast::new(
            24..45,
            AstKind::TupleLiteral(vec![
                Ast::new(25..29, AstKind::BooleanLiteral(true)),
                Ast::new(
                    31..43,
                    AstKind::RegexLiteral {
                        pattern: ".+",
                        flags: RegexFlags::X_EXTENDED
                            | RegexFlags::I_IGNORE_CASE
                            | RegexFlags::M_MULTILINE
                            | RegexFlags::G_GLOBAL
                            | RegexFlags::S_SINGLELINE
                            | RegexFlags::U_UNICODE,
                    }
                )
            ])
        ))
    );

    Ok(())
}
