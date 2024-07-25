use tracing::info;

use crate::{
    ast::{Ast, AstKind},
    lexer::TokenKind::*,
    parser::Parser,
};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test]
fn test_parser_operators() -> anyhow::Result<()> {
    let parser = &mut Parser::new("+()-/", 10);
    let result_a = parser.parse_tok(OpPlus)?;
    let result_b = parser.parse_tok(OpOpenParen)?;
    let result_c = parser.parse_tok(OpCloseParen)?;
    let result_d = parser.parse_tok(OpMinus)?;
    let result_e = parser.parse_tok(OpDiv)?;

    info!("input = {:?} | parse_tok(OpPlus) parse_tok(OpOpenParen) parse_tok(OpCloseParen) parse_tok(OpMinus) parse_tok(OpDiv) = {:?} {:?} {:?} {:?} {:?}", parser.lexer.string, result_a, result_b, result_c, result_d, result_e);

    assert_eq!(result_a, Some(Ast::new(0..1, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(1..2, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(2..3, AstKind::Temp)));
    assert_eq!(result_d, Some(Ast::new(3..4, AstKind::Temp)));
    assert_eq!(result_e, Some(Ast::new(4..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("*×", 10);
    let result_a = parser.parse_op_mul()?;
    let result_b = parser.parse_op_mul()?;

    info!(
        "input = {:?} | parse_op_mul() parse_op_mul() = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );

    assert_eq!(result_a, Some(Ast::new(0..1, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(1..3, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("&& and AND", 10);
    let result_a = parser.parse_op_and()?;
    let result_b = parser.parse_op_and()?;
    let result_c = parser.parse_op_and()?;

    info!(
        "input = {:?} | parse_op_and() parse_op_and() parse_op_and() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..6, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(7..10, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("|| or OR", 10);
    let result_a = parser.parse_op_or()?;
    let result_b = parser.parse_op_or()?;
    let result_c = parser.parse_op_or()?;

    info!(
        "input = {:?} | parse_op_or() parse_op_or() parse_op_or() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(6..8, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("= is IS", 10);
    let result_a = parser.parse_op_is()?;
    let result_b = parser.parse_op_is()?;
    let result_c = parser.parse_op_is()?;

    info!(
        "input = {:?} | parse_op_is() parse_op_is() parse_op_is() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..1, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(2..4, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(5..7, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("!= is NOT IS not", 10);
    let result_a = parser.parse_op_is_not()?;
    let result_b = parser.parse_op_is_not()?;
    let result_c = parser.parse_op_is_not()?;

    info!(
        "input = {:?} | parse_op_is_not() parse_op_is_not() parse_op_is_not() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..9, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(10..16, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("! not NOT", 10);
    let result_a = parser.parse_op_not()?;
    let result_b = parser.parse_op_not()?;
    let result_c = parser.parse_op_not()?;

    info!(
        "input = {:?} | parse_op_not() parse_op_not() parse_op_not() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..1, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(2..5, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(6..9, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("in IN", 10);
    let result_a = parser.parse_op_in()?;
    let result_b = parser.parse_op_in()?;

    info!(
        "input = {:?} | parse_op_in() parse_op_in() = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );

    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("not IN NOT in", 10);
    let result_a = parser.parse_op_not_in()?;
    let result_b = parser.parse_op_not_in()?;

    info!(
        "input = {:?} | parse_op_not_in() parse_op_not_in() = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );

    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("∋ contains CONTAINS", 10);
    let result_a = parser.parse_op_contains()?;
    let result_b = parser.parse_op_contains()?;
    let result_c = parser.parse_op_contains()?;

    info!(
        "input = {:?} | parse_op_contains() parse_op_contains() parse_op_contains() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..12, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(13..21, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("∌ not CONTAINS NOT contains", 10);
    let result_a = parser.parse_op_not_contains()?;
    let result_b = parser.parse_op_not_contains()?;
    let result_c = parser.parse_op_not_contains()?;

    info!(
        "input = {:?} | parse_op_not_contains() parse_op_not_contains() = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );

    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..16, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(17..29, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("⊅ contains NONE CONTAINS none", 10);
    let result_a = parser.parse_op_contains_none()?;
    let result_b = parser.parse_op_contains_none()?;
    let result_c = parser.parse_op_contains_none()?;

    info!(
        "input = {:?} | parse_op_contains_none() parse_op_contains_none() parse_op_contains_none() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..17, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(18..31, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("⊇ contains ALL CONTAINS all", 10);
    let result_a = parser.parse_op_contains_all()?;
    let result_b = parser.parse_op_contains_all()?;
    let result_c = parser.parse_op_contains_all()?;

    info!(
        "input = {:?} | parse_op_contains_all() parse_op_contains_all() parse_op_contains_all() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..16, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(17..29, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("⊃ contains ANY CONTAINS any", 10);
    let result_a = parser.parse_op_contains_any()?;
    let result_b = parser.parse_op_contains_any()?;
    let result_c = parser.parse_op_contains_any()?;

    info!(
        "input = {:?} | parse_op_contains_any() parse_op_contains_any() parse_op_contains_any() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..16, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(17..29, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("~ match MATCH", 10);
    let result_a = parser.parse_op_match()?;
    let result_b = parser.parse_op_match()?;
    let result_c = parser.parse_op_match()?;

    info!(
        "input = {:?} | parse_op_match() parse_op_match() parse_op_match() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..1, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(2..7, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(8..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("!~ not match NOT MATCH", 10);
    let result_a = parser.parse_op_not_match()?;
    let result_b = parser.parse_op_not_match()?;
    let result_c = parser.parse_op_not_match()?;

    info!(
        "input = {:?} | parse_op_not_match() parse_op_not_match() parse_op_not_match() = {:?} {:?} {:?}",
        parser.lexer.string, result_a, result_b, result_c
    );

    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..12, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(13..22, AstKind::Temp)));

    Ok(())
}
