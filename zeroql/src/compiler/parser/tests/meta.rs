use tracing::info;

use crate::{
    ast::{Ast, AstKind::*},
    parser::Parser,
};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test]
fn test_continuation_brackets() -> anyhow::Result<()> {
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
fn test_continuation_comma_assignment() -> anyhow::Result<()> {
    let _parser = &mut Parser::new(
        r#"CREATE person SET name = "John Doe",
        age = 30

        LET $variable =
        [1, 2, 3]

        UPDATE person SET age +=
        0x01
        "#,
        20,
    );

    // let result = parser.parse_create_exp()?;
    // let result = parser.parse_terminator()?;
    // let result = parser.parse_let_stmt()?;
    // let result = parser.parse_terminator()?;
    // let result = parser.parse_update_exp()?;

    // info!(
    //     r#"input = {:?} | parse_create_exp parse_terminator parse_let_stmt parse_terminator parse_update_exp = {:#?} {:#?} {:#?} {:#?} {:#?}"#,
    //     parser.lexer.string, result_a, result_b, result_c, result_d, result_e,
    // );

    // assert_eq!(result_a, None);
    // assert_eq!(result_b, None);
    // assert_eq!(result_c, None);
    // assert_eq!(result_d, None);
    // assert_eq!(result_e, None);

    Ok(())
}
