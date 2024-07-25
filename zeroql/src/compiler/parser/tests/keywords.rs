use tracing::info;

use crate::{
    ast::{Ast, AstKind},
    parser::Parser,
};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test]
fn test_parser_keywords() -> anyhow::Result<()> {
    let parser = &mut Parser::new("superfluous SUPERFLUOUS superfluous SUPERFLUOUS", 10);
    let result_a = parser.parse_kw("SUPERFLUOUS")?;
    let result_b = parser.parse_kw("superfluous")?;
    let result_c = parser.parse_kw("superfluous")?;
    let result_d = parser.parse_kw("SUPERFLUOUS")?;
    info!(
        r#"input = {:?} | parse_kw("SUPERFLUOUS") parse_kw("superfluous") parse_kw("superfluous") parse_kw("SUPERFLUOUS") = {:?} {:?} {:?} {:?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d
    );
    assert_eq!(result_a, Some(Ast::new(0..11, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(12..23, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(24..35, AstKind::Temp)));
    assert_eq!(result_d, Some(Ast::new(36..47, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("SMOOTH bean smooth BEAN SMOOTH BEAN smooth bean", 10);
    let result_a = parser.parse_kw2("SMOOTH", "BEAN")?;
    let result_b = parser.parse_kw2("smooth", "bean")?;
    let result_c = parser.parse_kw2("SMOOTH", "bean")?;
    let result_d = parser.parse_kw2("smooth", "BEAN")?;
    info!(
        r#"input = {:?} | parse_kw2("SMOOTH", "BEAN") parse_kw2("smooth", "bean") parse_kw2("SMOOTH", "bean") parse_kw2("smooth", "BEAN") = {:?} {:?} {:?} {:?}"#,
        parser.lexer.string, result_a, result_b, result_c, result_d
    );
    assert_eq!(result_a, Some(Ast::new(0..11, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(12..23, AstKind::Temp)));
    assert_eq!(result_c, Some(Ast::new(24..35, AstKind::Temp)));
    assert_eq!(result_d, Some(Ast::new(36..47, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("create CREATE", 10);
    let result_a = parser.parse_kw_create()?;
    let result_b = parser.parse_kw_create()?;
    info!(
        "input = {:?} | parse_kw_create parse_kw_create = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("relate RELATE", 10);
    let result_a = parser.parse_kw_relate()?;
    let result_b = parser.parse_kw_relate()?;
    info!(
        "input = {:?} | parse_kw_relate parse_kw_relate = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("delete DELETE", 10);
    let result_a = parser.parse_kw_delete()?;
    let result_b = parser.parse_kw_delete()?;
    info!(
        "input = {:?} | parse_kw_delete parse_kw_delete = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("update UPDATE", 10);
    let result_a = parser.parse_kw_update()?;
    let result_b = parser.parse_kw_update()?;
    info!(
        "input = {:?} | parse_kw_update parse_kw_update = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("select SELECT", 10);
    let result_a = parser.parse_kw_select()?;
    let result_b = parser.parse_kw_select()?;
    info!(
        "input = {:?} | parse_kw_select parse_kw_select = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("fold FOLD", 10);
    let result_a = parser.parse_kw_fold()?;
    let result_b = parser.parse_kw_fold()?;
    info!(
        "input = {:?} | parse_kw_fold parse_kw_fold = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..4, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(5..9, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("omit OMIT", 10);
    let result_a = parser.parse_kw_omit()?;
    let result_b = parser.parse_kw_omit()?;
    info!(
        "input = {:?} | parse_kw_omit parse_kw_omit = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..4, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(5..9, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("break BREAK", 10);
    let result_a = parser.parse_kw_break()?;
    let result_b = parser.parse_kw_break()?;
    info!(
        "input = {:?} | parse_kw_break parse_kw_break = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("continue CONTINUE", 10);
    let result_a = parser.parse_kw_continue()?;
    let result_b = parser.parse_kw_continue()?;
    info!(
        "input = {:?} | parse_kw_continue parse_kw_continue = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..8, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(9..17, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("set SET", 10);
    let result_a = parser.parse_kw_set()?;
    let result_b = parser.parse_kw_set()?;
    info!(
        "input = {:?} | parse_kw_set parse_kw_set = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..7, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("begin BEGIN", 10);
    let result_a = parser.parse_kw_begin()?;
    let result_b = parser.parse_kw_begin()?;
    info!(
        "input = {:?} | parse_kw_begin parse_kw_begin = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("transaction TRANSACTION", 10);
    let result_a = parser.parse_kw_transaction()?;
    let result_b = parser.parse_kw_transaction()?;
    info!(
        "input = {:?} | parse_kw_transaction parse_kw_transaction = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..11, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(12..23, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("commit COMMIT", 10);
    let result_a = parser.parse_kw_commit()?;
    let result_b = parser.parse_kw_commit()?;
    info!(
        "input = {:?} | parse_kw_commit parse_kw_commit = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("cancel CANCEL", 10);
    let result_a = parser.parse_kw_cancel()?;
    let result_b = parser.parse_kw_cancel()?;
    info!(
        "input = {:?} | parse_kw_cancel parse_kw_cancel = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("define DEFINE", 10);
    let result_a = parser.parse_kw_define()?;
    let result_b = parser.parse_kw_define()?;
    info!(
        "input = {:?} | parse_kw_define parse_kw_define = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("redefine REDEFINE", 10);
    let result_a = parser.parse_kw_redefine()?;
    let result_b = parser.parse_kw_redefine()?;
    info!(
        "input = {:?} | parse_kw_redefine parse_kw_redefine = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..8, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(9..17, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("remove REMOVE", 10);
    let result_a = parser.parse_kw_remove()?;
    let result_b = parser.parse_kw_remove()?;
    info!(
        "input = {:?} | parse_kw_remove parse_kw_remove = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("describe DESCRIBE", 10);
    let result_a = parser.parse_kw_describe()?;
    let result_b = parser.parse_kw_describe()?;
    info!(
        "input = {:?} | parse_kw_describe parse_kw_describe = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..8, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(9..17, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("namespace NAMESPACE", 10);
    let result_a = parser.parse_kw_namespace()?;
    let result_b = parser.parse_kw_namespace()?;
    info!(
        "input = {:?} | parse_kw_namespace parse_kw_namespace = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..9, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(10..19, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("ns NS", 10);
    let result_a = parser.parse_kw_ns()?;
    let result_b = parser.parse_kw_ns()?;
    info!(
        "input = {:?} | parse_kw_ns parse_kw_ns = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("database DATABASE", 10);
    let result_a = parser.parse_kw_database()?;
    let result_b = parser.parse_kw_database()?;
    info!(
        "input = {:?} | parse_kw_database parse_kw_database = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..8, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(9..17, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("db DB", 10);
    let result_a = parser.parse_kw_db()?;
    let result_b = parser.parse_kw_db()?;
    info!(
        "input = {:?} | parse_kw_db parse_kw_db = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("table TABLE", 10);
    let result_a = parser.parse_kw_table()?;
    let result_b = parser.parse_kw_table()?;
    info!(
        "input = {:?} | parse_kw_table parse_kw_table = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("fields FIELDS", 10);
    let result_a = parser.parse_kw_fields()?;
    let result_b = parser.parse_kw_fields()?;
    info!(
        "input = {:?} | parse_kw_fields parse_kw_fields = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("index INDEX", 10);
    let result_a = parser.parse_kw_index()?;
    let result_b = parser.parse_kw_index()?;
    info!(
        "input = {:?} | parse_kw_index parse_kw_index = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("indices INDICES", 10);
    let result_a = parser.parse_kw_indices()?;
    let result_b = parser.parse_kw_indices()?;
    info!(
        "input = {:?} | parse_kw_indices parse_kw_indices = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..7, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(8..15, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("type TYPE", 10);
    let result_a = parser.parse_kw_type()?;
    let result_b = parser.parse_kw_type()?;
    info!(
        "input = {:?} | parse_kw_type parse_kw_type = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..4, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(5..9, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("enum ENUM", 10);
    let result_a = parser.parse_kw_enum()?;
    let result_b = parser.parse_kw_enum()?;
    info!(
        "input = {:?} | parse_kw_enum parse_kw_enum = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..4, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(5..9, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("edge EDGE", 10);
    let result_a = parser.parse_kw_edge()?;
    let result_b = parser.parse_kw_edge()?;
    info!(
        "input = {:?} | parse_kw_edge parse_kw_edge = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..4, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(5..9, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("module MODULE", 10);
    let result_a = parser.parse_kw_module()?;
    let result_b = parser.parse_kw_module()?;
    info!(
        "input = {:?} | parse_kw_module parse_kw_module = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("param PARAM", 10);
    let result_a = parser.parse_kw_param()?;
    let result_b = parser.parse_kw_param()?;
    info!(
        "input = {:?} | parse_kw_param parse_kw_param = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("mod MOD", 10);
    let result_a = parser.parse_kw_mod()?;
    let result_b = parser.parse_kw_mod()?;
    info!(
        "input = {:?} | parse_kw_mod parse_kw_mod = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..7, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("value VALUE", 10);
    let result_a = parser.parse_kw_value()?;
    let result_b = parser.parse_kw_value()?;
    info!(
        "input = {:?} | parse_kw_value parse_kw_value = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("values VALUES", 10);
    let result_a = parser.parse_kw_values()?;
    let result_b = parser.parse_kw_values()?;
    info!(
        "input = {:?} | parse_kw_values parse_kw_values = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("variant VARIANT", 10);
    let result_a = parser.parse_kw_variant()?;
    let result_b = parser.parse_kw_variant()?;
    info!(
        "input = {:?} | parse_kw_variant parse_kw_variant = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..7, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(8..15, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("variants VARIANTS", 10);
    let result_a = parser.parse_kw_variants()?;
    let result_b = parser.parse_kw_variants()?;
    info!(
        "input = {:?} | parse_kw_variants parse_kw_variants = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..8, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(9..17, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("assert ASSERT", 10);
    let result_a = parser.parse_kw_assert()?;
    let result_b = parser.parse_kw_assert()?;
    info!(
        "input = {:?} | parse_kw_assert parse_kw_assert = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("readonly READONLY", 10);
    let result_a = parser.parse_kw_readonly()?;
    let result_b = parser.parse_kw_readonly()?;
    info!(
        "input = {:?} | parse_kw_readonly parse_kw_readonly = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..8, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(9..17, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("unique UNIQUE", 10);
    let result_a = parser.parse_kw_unique()?;
    let result_b = parser.parse_kw_unique()?;
    info!(
        "input = {:?} | parse_kw_unique parse_kw_unique = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("use USE", 10);
    let result_a = parser.parse_kw_use()?;
    let result_b = parser.parse_kw_use()?;
    info!(
        "input = {:?} | parse_kw_use parse_kw_use = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..7, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("if IF", 10);
    let result_a = parser.parse_kw_if()?;
    let result_b = parser.parse_kw_if()?;
    info!(
        "input = {:?} | parse_kw_if parse_kw_if = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("else ELSE", 10);
    let result_a = parser.parse_kw_else()?;
    let result_b = parser.parse_kw_else()?;
    info!(
        "input = {:?} | parse_kw_else parse_kw_else = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..4, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(5..9, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("for FOR", 10);
    let result_a = parser.parse_kw_for()?;
    let result_b = parser.parse_kw_for()?;
    info!(
        "input = {:?} | parse_kw_for parse_kw_for = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..7, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("then THEN", 10);
    let result_a = parser.parse_kw_then()?;
    let result_b = parser.parse_kw_then()?;
    info!(
        "input = {:?} | parse_kw_then parse_kw_then = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..4, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(5..9, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("do DO", 10);
    let result_a = parser.parse_kw_do()?;
    let result_b = parser.parse_kw_do()?;
    info!(
        "input = {:?} | parse_kw_do parse_kw_do = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("end END", 10);
    let result_a = parser.parse_kw_end()?;
    let result_b = parser.parse_kw_end()?;
    info!(
        "input = {:?} | parse_kw_end parse_kw_end = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..7, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("not NOT", 10);
    let result_a = parser.parse_kw_not()?;
    let result_b = parser.parse_kw_not()?;
    info!(
        "input = {:?} | parse_kw_not parse_kw_not = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..7, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("exists EXISTS", 10);
    let result_a = parser.parse_kw_exists()?;
    let result_b = parser.parse_kw_exists()?;
    info!(
        "input = {:?} | parse_kw_exists parse_kw_exists = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..6, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(7..13, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("with WITH", 10);
    let result_a = parser.parse_kw_with()?;
    let result_b = parser.parse_kw_with()?;
    info!(
        "input = {:?} | parse_kw_with parse_kw_with = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..4, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(5..9, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("on ON", 10);
    let result_a = parser.parse_kw_on()?;
    let result_b = parser.parse_kw_on()?;
    info!(
        "input = {:?} | parse_kw_on parse_kw_on = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("let LET", 10);
    let result_a = parser.parse_kw_let()?;
    let result_b = parser.parse_kw_let()?;
    info!(
        "input = {:?} | parse_kw_let parse_kw_let = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..7, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("where WHERE", 10);
    let result_a = parser.parse_kw_where()?;
    let result_b = parser.parse_kw_where()?;
    info!(
        "input = {:?} | parse_kw_where parse_kw_where = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("group GROUP", 10);
    let result_a = parser.parse_kw_group()?;
    let result_b = parser.parse_kw_group()?;
    info!(
        "input = {:?} | parse_kw_group parse_kw_group = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("limit LIMIT", 10);
    let result_a = parser.parse_kw_limit()?;
    let result_b = parser.parse_kw_limit()?;
    info!(
        "input = {:?} | parse_kw_limit parse_kw_limit = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("start START", 10);
    let result_a = parser.parse_kw_start()?;
    let result_b = parser.parse_kw_start()?;
    info!(
        "input = {:?} | parse_kw_start parse_kw_start = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("order ORDER", 10);
    let result_a = parser.parse_kw_order()?;
    let result_b = parser.parse_kw_order()?;
    info!(
        "input = {:?} | parse_kw_order parse_kw_order = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..5, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(6..11, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("by BY", 10);
    let result_a = parser.parse_kw_by()?;
    let result_b = parser.parse_kw_by()?;
    info!(
        "input = {:?} | parse_kw_by parse_kw_by = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("at AT", 10);
    let result_a = parser.parse_kw_at()?;
    let result_b = parser.parse_kw_at()?;
    info!(
        "input = {:?} | parse_kw_at parse_kw_at = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("to TO", 10);
    let result_a = parser.parse_kw_to()?;
    let result_b = parser.parse_kw_to()?;
    info!(
        "input = {:?} | parse_kw_to parse_kw_to = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("no NO", 10);
    let result_a = parser.parse_kw_no()?;
    let result_b = parser.parse_kw_no()?;
    info!(
        "input = {:?} | parse_kw_no parse_kw_no = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );

    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("asc ASC", 10);

    let result_a = parser.parse_kw_asc()?;
    let result_b = parser.parse_kw_asc()?;
    info!(
        "input = {:?} | parse_kw_asc parse_kw_asc = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..3, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(4..7, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("desc DESC", 10);
    let result_a = parser.parse_kw_desc()?;
    let result_b = parser.parse_kw_desc()?;
    info!(
        "input = {:?} | parse_kw_desc parse_kw_desc = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );

    assert_eq!(result_a, Some(Ast::new(0..4, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(5..9, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    let parser = &mut Parser::new("as AS", 10);
    let result_a = parser.parse_kw_as()?;
    let result_b = parser.parse_kw_as()?;
    info!(
        "input = {:?} | parse_kw_as parse_kw_as = {:?} {:?}",
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, Some(Ast::new(0..2, AstKind::Temp)));
    assert_eq!(result_b, Some(Ast::new(3..5, AstKind::Temp)));

    //----------------------------------------------------------------------------------------------

    // Fail Case
    let parser = &mut Parser::new("superFLUOUS", 10);
    let result_a = parser.parse_kw("SUPERFLUOUS")?;
    let result_b = parser.parse_kw("superfluous")?;
    info!(
        r#"input = {:?} | parse_kw("SUPERFLUOUS") parse_kw("superfluous") = {:?} {:?}"#,
        parser.lexer.string, result_a, result_b
    );
    assert_eq!(result_a, None);
    assert_eq!(result_b, None);

    Ok(())
}
