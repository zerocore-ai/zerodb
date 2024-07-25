use zeroql_macros::{backtrack, memoize};

use crate::{
    ast::{Ast, AstKind},
    lexer::{Token, TokenKind},
};

use super::{Parser, ParserResult};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[backtrack(state = self.lexer.cursor, condition = |r| matches!(r, Ok(None)))]
#[memoize(cache = self.cache, salt = self.lexer.cursor)]
impl<'a> Parser<'a> {
    /// Parses a keyword.
    pub(super) fn parse_kw(&mut self, string: &'a str) -> ParserResult<Option<Ast<'a>>> {
        let token = self.eat_token()?;
        if let Some(Token {
            span,
            kind: TokenKind::PlainIdentifier(ident),
        }) = token
        {
            if ident == string.to_uppercase() || ident == string.to_lowercase() {
                return Ok(Some(Ast::new(span, AstKind::Temp)));
            }
        }

        Ok(None)
    }

    /// Parses two keywords in a row.
    pub(super) fn parse_kw2(
        &mut self,
        string_a: &'a str,
        string_b: &'a str,
    ) -> ParserResult<Option<Ast<'a>>> {
        if let Some(Ast { span: span_a, .. }) = self.parse_kw(string_a)? {
            if let Some(Ast { span: span_b, .. }) = self.parse_kw(string_b)? {
                return Ok(Some(Ast::new(span_a.start..span_b.end, AstKind::Temp)));
            }
        }

        Ok(None)
    }

    /// Parses the `kw_create` rule.
    ///
    /// ```txt
    /// kw_create =
    ///     | plain_identifier["create"]
    ///     | plain_identifier["CREATE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_create(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("create")
    }
    /// Parses the `kw_relate` rule.
    ///
    /// ```txt
    /// kw_relate =
    ///     | plain_identifier["relate"]
    ///     | plain_identifier["RELATE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_relate(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("relate")
    }

    /// Parses the `kw_delete` rule.
    ///
    /// ```txt
    /// kw_delete =
    ///     | plain_identifier["delete"]
    ///     | plain_identifier["DELETE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_delete(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("delete")
    }

    /// Parses the `kw_update` rule.
    ///
    /// ```txt
    /// kw_update =
    ///     | plain_identifier["update"]
    ///     | plain_identifier["UPDATE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_update(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("update")
    }

    /// Parses the `kw_select` rule.
    ///
    /// ```txt
    /// kw_select =
    ///     | plain_identifier["select"]
    ///     | plain_identifier["SELECT"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_select(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("select")
    }

    /// Parses the `kw_fold` rule.
    ///
    /// ```txt
    /// kw_fold =
    ///     | plain_identifier["fold"]
    ///     | plain_identifier["FOLD"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_fold(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("fold")
    }

    /// Parses the `kw_omit` rule.
    ///
    /// ```txt
    /// kw_omit =
    ///     | plain_identifier["omit"]
    ///     | plain_identifier["OMIT"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_omit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("omit")
    }

    /// Parses the `kw_break` rule.
    ///
    /// ```txt
    /// kw_break =
    ///     | plain_identifier["break"]
    ///     | plain_identifier["BREAK"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_break(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("break")
    }

    /// Parses the `kw_continue` rule.
    ///
    /// ```txt
    /// kw_continue =
    ///     | plain_identifier["continue"]
    ///     | plain_identifier["CONTINUE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_continue(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("continue")
    }

    /// Parses the `kw_set` rule.
    ///
    /// ```txt
    /// kw_set =
    ///     | plain_identifier["set"]
    ///     | plain_identifier["SET"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_set(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("set")
    }

    /// Parses the `kw_begin` rule.
    ///
    /// ```txt
    /// kw_begin =
    ///     | plain_identifier["begin"]
    ///     | plain_identifier["BEGIN"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_begin(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("begin")
    }

    /// Parses the `kw_transaction` rule.
    ///
    /// ```txt
    /// kw_transaction =
    ///     | plain_identifier["transaction"]
    ///     | plain_identifier["TRANSACTION"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_transaction(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("transaction")
    }

    /// Parses the `kw_commit` rule.
    ///
    /// ```txt
    /// kw_commit =
    ///     | plain_identifier["commit"]
    ///     | plain_identifier["COMMIT"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_commit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("commit")
    }

    /// Parses the `kw_cancel` rule.
    ///
    /// ```txt
    /// kw_cancel =
    ///     | plain_identifier["cancel"]
    ///     | plain_identifier["CANCEL"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_cancel(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("cancel")
    }

    /// Parses the `kw_define` rule.
    ///
    /// ```txt
    /// kw_define =
    ///     | plain_identifier["define"]
    ///     | plain_identifier["DEFINE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_define(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("define")
    }

    /// Parses the `kw_redefine` rule.
    ///
    /// ```txt
    /// kw_redefine =
    ///     | plain_identifier["redefine"]
    ///     | plain_identifier["REDEFINE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_redefine(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("redefine")
    }

    /// Parses the `kw_remove` rule.
    ///
    /// ```txt
    /// kw_remove =
    ///     | plain_identifier["remove"]
    ///     | plain_identifier["REMOVE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_remove(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("remove")
    }

    /// Parses the `kw_describe` rule.
    ///
    /// ```txt
    /// kw_describe =
    ///     | plain_identifier["describe"]
    ///     | plain_identifier["DESCRIBE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_describe(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("describe")
    }

    /// Parses the `kw_namespace` rule.
    ///
    /// ```txt
    /// kw_namespace =
    ///     | plain_identifier["namespace"]
    ///     | plain_identifier["NAMESPACE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_namespace(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("namespace")
    }

    /// Parses the `kw_ns` rule.
    ///
    /// ```txt
    /// kw_ns =
    ///     | plain_identifier["ns"]
    ///     | plain_identifier["NS"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_ns(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("ns")
    }

    /// Parses the `kw_database` rule.
    ///
    /// ```txt
    /// kw_database =
    ///     | plain_identifier["database"]
    ///     | plain_identifier["DATABASE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_database(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("database")
    }

    /// Parses the `kw_db` rule.
    ///
    /// ```txt
    /// kw_db =
    ///     | plain_identifier["db"]
    ///     | plain_identifier["DB"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_db(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("db")
    }

    /// Parses the `kw_table` rule.
    ///
    /// ```txt
    /// kw_table =
    ///     | plain_identifier["table"]
    ///     | plain_identifier["TABLE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_table(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("table")
    }

    /// Parses the `kw_fields` rule.
    ///
    /// ```txt
    /// kw_fields =
    ///     | plain_identifier["fields"]
    ///     | plain_identifier["FIELDS"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_fields(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("fields")
    }

    /// Parses the `kw_index` rule.
    ///
    /// ```txt
    /// kw_index =
    ///     | plain_identifier["index"]
    ///     | plain_identifier["INDEX"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_index(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("index")
    }

    /// Parses the `kw_indices` rule.
    ///
    /// ```txt
    /// kw_indices =
    ///     | plain_identifier["indices"]
    ///     | plain_identifier["INDICES"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_indices(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("indices")
    }

    /// Parses the `kw_type` rule.
    ///
    /// ```txt
    /// kw_type =
    ///     | plain_identifier["type"]
    ///     | plain_identifier["TYPE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_type(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("type")
    }

    /// Parses the `kw_enum` rule.
    ///
    /// ```txt
    /// kw_enum =
    ///     | plain_identifier["enum"]
    ///     | plain_identifier["ENUM"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_enum(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("enum")
    }

    /// Parses the `kw_edge` rule.
    ///
    /// ```txt
    /// kw_edge =
    ///     | plain_identifier["edge"]
    ///     | plain_identifier["EDGE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_edge(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("edge")
    }

    /// Parses the `kw_module` rule.
    ///
    /// ```txt
    /// kw_module =
    ///     | plain_identifier["module"]
    ///     | plain_identifier["MODULE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_module(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("module")
    }

    /// Parses the `kw_param` rule.
    ///
    /// ```txt
    /// kw_param =
    ///     | plain_identifier["param"]
    ///     | plain_identifier["PARAM"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_param(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("param")
    }

    /// Parses the `kw_mod` rule.
    ///
    /// ```txt
    /// kw_mod =
    ///     | plain_identifier["mod"]
    ///     | plain_identifier["MOD"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_mod(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("mod")
    }

    /// Parses the `kw_value` rule.
    ///
    /// ```txt
    /// kw_value =
    ///     | plain_identifier["value"]
    ///     | plain_identifier["VALUE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_value(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("value")
    }

    /// Parses the `kw_values` rule.
    ///
    /// ```txt
    /// kw_values =
    ///     | plain_identifier["values"]
    ///     | plain_identifier["VALUES"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_values(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("values")
    }

    /// Parses the `kw_variant` rule.
    ///
    /// ```txt
    /// kw_variant =
    ///     | plain_identifier["variant"]
    ///     | plain_identifier["VARIANT"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_variant(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("variant")
    }

    /// Parses the `kw_variants` rule.
    ///
    /// ```txt
    /// kw_variants =
    ///     | plain_identifier["variants"]
    ///     | plain_identifier["VARIANTS"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_variants(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("variants")
    }

    /// Parses the `kw_assert` rule.
    ///
    /// ```txt
    /// kw_assert =
    ///     | plain_identifier["assert"]
    ///     | plain_identifier["ASSERT"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_assert(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("assert")
    }

    /// Parses the `kw_readonly` rule.
    ///
    /// ```txt
    /// kw_readonly =
    ///     | plain_identifier["readonly"]
    ///     | plain_identifier["READONLY"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_readonly(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("readonly")
    }

    /// Parses the `kw_unique` rule.
    ///
    /// ```txt
    /// kw_unique =
    ///     | plain_identifier["unique"]
    ///     | plain_identifier["UNIQUE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_unique(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("unique")
    }

    /// Parses the `kw_use` rule.
    ///
    /// ```txt
    /// kw_use =
    ///     | plain_identifier["use"]
    ///     | plain_identifier["USE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_use(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("use")
    }

    /// Parses the `kw_if` rule.
    ///
    /// ```txt
    /// kw_if =
    ///     | plain_identifier["if"]
    ///     | plain_identifier["IF"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_if(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("if")
    }

    /// Parses the `kw_else` rule.
    ///
    /// ```txt
    /// kw_else =
    ///     | plain_identifier["else"]
    ///     | plain_identifier["ELSE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_else(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("else")
    }

    /// Parses the `kw_for` rule.
    ///
    /// ```txt
    /// kw_for =
    ///     | plain_identifier["for"]
    ///     | plain_identifier["FOR"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_for(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("for")
    }

    /// Parses the `kw_then` rule.
    ///
    /// ```txt
    /// kw_then =
    ///     | plain_identifier["then"]
    ///     | plain_identifier["THEN"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_then(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("then")
    }

    /// Parses the `kw_do` rule.
    ///
    /// ```txt
    /// kw_do =
    ///     | plain_identifier["do"]
    ///     | plain_identifier["DO"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_do(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("do")
    }

    /// Parses the `kw_end` rule.
    ///
    /// ```txt
    /// kw_end =
    ///     | plain_identifier["end"]
    ///     | plain_identifier["END"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_end(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("end")
    }

    /// Parses the `kw_not` rule.
    ///
    /// ```txt
    /// kw_not =
    ///     | plain_identifier["not"]
    ///     | plain_identifier["NOT"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_not(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("not")
    }

    /// Parses the `kw_exists` rule.
    ///
    /// ```txt
    /// kw_exists =
    ///     | plain_identifier["exists"]
    ///     | plain_identifier["EXISTS"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_exists(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("exists")
    }

    /// Parses the `kw_with` rule.
    ///
    /// ```txt
    /// kw_with =
    ///     | plain_identifier["with"]
    ///     | plain_identifier["WITH"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_with(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("with")
    }

    /// Parses the `kw_on` rule.
    ///
    /// ```txt
    /// kw_on =
    ///     | plain_identifier["on"]
    ///     | plain_identifier["ON"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_on(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("on")
    }

    /// Parses the `kw_let` rule.
    ///
    /// ```txt
    /// kw_let =
    ///     | plain_identifier["let"]
    ///     | plain_identifier["LET"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_let(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("let")
    }

    /// Parses the `kw_where` rule.
    ///
    /// ```txt
    /// kw_where =
    ///     | plain_identifier["where"]
    ///     | plain_identifier["WHERE"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_where(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("where")
    }

    /// Parses the `kw_group` rule.
    ///
    /// ```txt
    /// kw_group =
    ///     | plain_identifier["group"]
    ///     | plain_identifier["GROUP"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_group(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("group")
    }

    /// Parses the `kw_limit` rule.
    ///
    /// ```txt
    /// kw_limit =
    ///     | plain_identifier["limit"]
    ///     | plain_identifier["LIMIT"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_limit(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("limit")
    }

    /// Parses the `kw_start` rule.
    ///
    /// ```txt
    /// kw_start =
    ///     | plain_identifier["start"]
    ///     | plain_identifier["START"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_start(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("start")
    }

    /// Parses the `kw_order` rule.
    ///
    /// ```txt
    /// kw_order =
    ///     | plain_identifier["order"]
    ///     | plain_identifier["ORDER"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_order(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("order")
    }

    /// Parses the `kw_by` rule.
    ///
    /// ```txt
    /// kw_by =
    ///     | plain_identifier["by"]
    ///     | plain_identifier["BY"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_by(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("by")
    }

    /// Parses the `kw_at` rule.
    ///
    /// ```txt
    /// kw_at =
    ///     | plain_identifier["at"]
    ///     | plain_identifier["AT"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_at(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("at")
    }

    /// Parses the `kw_to` rule.
    ///
    /// ```txt
    /// kw_to =
    ///     | plain_identifier["to"]
    ///     | plain_identifier["TO"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_to(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("to")
    }

    /// Parses the `kw_no` rule.
    ///
    /// ```txt
    /// kw_no =
    ///     | plain_identifier["no"]
    ///     | plain_identifier["NO"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_no(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("no")
    }

    /// Parses the `kw_asc` rule.
    ///
    /// ```txt
    /// kw_asc =
    ///     | plain_identifier["asc"]
    ///     | plain_identifier
    pub fn parse_kw_asc(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("asc")
    }

    /// Parses the `kw_desc` rule.
    ///
    /// ```txt
    /// kw_desc =
    ///     | plain_identifier["desc"]
    ///     | plain_identifier["DESC"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_desc(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("desc")
    }

    /// Parses the `kw_as` rule.
    ///
    /// ```txt
    /// kw_as =
    ///     | plain_identifier["as"]
    ///     | plain_identifier["AS"]
    /// ```
    #[backtrack]
    #[memoize]
    pub fn parse_kw_as(&mut self) -> ParserResult<Option<Ast<'a>>> {
        self.parse_kw("as")
    }
}
