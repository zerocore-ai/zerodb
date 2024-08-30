use crate::{
    ast::{Ast, AstKind::*, ElseIfPart, SelectColumn, SelectTransform, TypeSig},
    sema::{
        error::SemaResult, symbols::Symbols, DatabaseSchema, SchemaMeta, SemaError, SymbolMeta,
        VersionedSchema,
    },
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `NameResolver` pass resolves variable declarations, schema definitions and their usage.
///
/// This pass ensures that all variables and schema components are defined before being used.
/// While it enforces that schema definitions cannot be duplicated, it allows variables to be
/// shadowed.
///
/// In this context, "schema" refers to top-level structures that can be stored within a database.
/// This includes tables, edges, types, enums, indexes, modules, parameters and databases themselves.
///
/// ## Note
///
/// This pass does not analyze names of members of structs, enums, or tables. It only concerns itself
/// with names of top-level schema items.
///
/// Variable shadowing is permitted, allowing variables to be redefined within the same and nested
/// scopes. However, schema definitions must remain unique.
///
// TODO: We should support relative path when specifying `ON TABLE/DATABASE`. Right they are treated
// like absolute paths. We need canonicalization.
// TODO: When module imports become supported. Need to think about what that means for top-level names.
// TODO: Scoped identifiers not properly supported yet.
// TODO: Support removed schema item within conditionals being in superposition of existing and not-existing states.
pub struct NameResolver {
    /// Symbol table for the current scope.
    current_symbols: Symbols,

    /// The current schema version.
    current_schema: Option<VersionedSchema>,

    /// Whether the current schema needs to be tagged.
    has_untagged_schema: bool,

    /// Whether the current symbols needs to be tagged.
    has_untagged_symbols: bool,

    /// The current database set by the `USE` statement.
    /// It starts out at root.
    current_database: Path,

    /// Used to get information about the schemas that are already persisted by the database.
    db_schema: DatabaseSchema,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl NameResolver {
    /// Creates a new name resolution pass.
    pub fn new() -> Self {
        Self {
            current_symbols: Symbols::default(),
            current_schema: None,
            has_untagged_schema: false,
            has_untagged_symbols: true,
            current_database: Path::default(),
            db_schema: DatabaseSchema::default(),
        }
    }

    /// Runs name resolution analysis on the given ast.
    #[inline]
    pub fn analyze(&mut self, ast: &mut Ast) -> SemaResult<()> {
        self.analyze_with_options(ast, true)
    }

    /// Runs name resolution analysis on the given ast.
    ///
    /// If `analyze_identifier` is true, the identifier is analyzed for schema item existence.
    /// This is useful in statements like SELECT where we need to analyze sub-expressions
    /// in the column selection part but not the identifiers because they are not top-level
    /// schema items.
    fn analyze_with_options(&mut self, ast: &mut Ast, analyze_identifiers: bool) -> SemaResult<()> {
        // Tag the schema to the ast node if any
        self.tag_schema(ast);

        match &mut ast.kind {
            // == Variable Declarations ==
            Let {
                name,
                value,
                r#type,
            } => {
                // Get the name of the variable
                let name = ast_as!(**name, Variable(name));

                // Insert the variable into the current scope. TODO: How to do shadowing?
                self.current_symbols
                    .insert(name.to_string(), SymbolMeta::default());

                // Analyze the type
                if let Some(r#type) = r#type {
                    self.analyze_type(r#type)?;
                }

                // Analyze the value
                self.analyze(value)?;
            }
            For {
                variable,
                iterator,
                body,
            } => {
                self.analyze_scope(
                    |r| r.analyze(iterator),
                    |r| {
                        // Get the name of the for variable
                        let name = ast_as!(**variable, Variable(name));

                        // Insert the variable into the current scope
                        r.current_symbols
                            .insert(name.to_string(), SymbolMeta::default());

                        // Analyze the body
                        r.analyze(body)
                    },
                )?;
            }

            // == Variable Scopes ==
            While { condition, body } => {
                self.analyze_scope(|r| r.analyze(condition), |r| r.analyze(body))?;
            }
            If {
                condition,
                then,
                else_ifs,
                r#else,
            } => {
                self.analyze_scope(|r| r.analyze(condition), |r| r.analyze(then))?;

                for ElseIfPart { condition, body } in else_ifs {
                    self.analyze_scope(|r| r.analyze(condition), |r| r.analyze(body))?;
                }

                if let Some(r#else) = r#else {
                    self.analyze_scope(|_| Ok(()), |r| r.analyze(r#else))?;
                }
            }
            Program(asts) => {
                // Tag the symbols to the ast node if any
                if let Some(ast) = asts.first_mut() {
                    self.tag_symbols(ast);
                }

                // Analyze the asts
                for ast in asts {
                    self.analyze(ast)?;
                }
            }

            // == Schema Definitions ==
            DefineDatabase {
                name,
                if_not_exists,
                namespace,
                ..
            } => {
                self.register_schema_item_definition(
                    name,
                    namespace,
                    *if_not_exists,
                    SchemaMeta::Database(),
                )?;
            }
            DefineTable {
                name,
                if_not_exists,
                database,
                ..
            } => {
                self.register_schema_item_definition(
                    name,
                    database,
                    *if_not_exists,
                    SchemaMeta::Table(),
                )?;
            }
            DefineEdge {
                name,
                if_not_exists,
                database,
                ..
            } => {
                self.register_schema_item_definition(
                    name,
                    database,
                    *if_not_exists,
                    SchemaMeta::Edge(),
                )?;
            }
            DefineType {
                name,
                if_not_exists,
                database,
                ..
            } => {
                self.register_schema_item_definition(
                    name,
                    database,
                    *if_not_exists,
                    SchemaMeta::Type(),
                )?;
            }
            DefineEnum {
                name,
                if_not_exists,
                database,
                ..
            } => {
                self.register_schema_item_definition(
                    name,
                    database,
                    *if_not_exists,
                    SchemaMeta::Enum(),
                )?;
            }
            DefineIndex {
                name,
                if_not_exists,
                database,
                ..
            } => {
                self.register_schema_item_definition(
                    name,
                    database,
                    *if_not_exists,
                    SchemaMeta::Index(),
                )?;
            }
            DefineModule {
                name,
                if_not_exists,
                database,
                ..
            } => {
                self.register_schema_item_definition(
                    name,
                    database,
                    *if_not_exists,
                    SchemaMeta::Module(),
                )?;
            }
            DefineParam {
                name,
                if_not_exists,
                database,
                ..
            } => {
                self.register_schema_item_definition(
                    name,
                    database,
                    *if_not_exists,
                    SchemaMeta::Param(),
                )?;
            }

            // == Usage ==
            Identifier(name) => {
                if analyze_identifiers {
                    // Check if the identifier is a schema item
                    self.check_schema_item_exists(name, &None)?;
                }
            }
            Variable(name) => {
                // Check if the variable or param exists
                self.check_variable_or_param_exists(name)?;
            }
            IdOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            ScopedIdentifier(identifiers) => {
                if let [ast] = &mut identifiers[..] {
                    self.analyze(ast)?
                }
            }

            ListLiteral(asts) => {
                for ast in asts {
                    self.analyze(ast)?;
                }
            }
            TupleLiteral(asts) => {
                for ast in asts {
                    self.analyze(ast)?;
                }
            }
            ObjectLiteral(asts) => {
                for (_, ast) in asts {
                    self.analyze(ast)?;
                }
            }
            Index { subject, index } => {
                self.analyze(subject)?;
                self.analyze(index)?;
            }
            FunctionArg { value, .. } => {
                self.analyze(value)?;
            }
            FunctionCall { subject, args } => {
                self.analyze(subject)?;
                for arg in args {
                    self.analyze(arg)?;
                }
            }
            LogicalNotOp(ast) => {
                self.analyze(ast)?;
            }
            BitwiseNotOp(ast) => {
                self.analyze(ast)?;
            }
            PlusSignOp(ast) => {
                self.analyze(ast)?;
            }
            MinusSignOp(ast) => {
                self.analyze(ast)?;
            }
            DotAccessOp { subject, .. } => {
                self.analyze(subject)?;
            }
            SafeNavigationAccessOp { subject, .. } => {
                self.analyze(subject)?;
            }
            DotAccessWildcardOp { subject, .. } => {
                self.analyze(subject)?;
            }
            ExponentiationOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            MultiplicationOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            DivisionOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            ModulusOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            AdditionOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            SubtractionOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            LeftShiftOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            RightShiftOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            MatchOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            NotMatchOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            SimilarityOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            LessThanOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            GreaterThanOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            LessThanEqualToOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            GreaterThanEqualToOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            InOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            NotInOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            ContainsOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            NotContainsOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            ContainsNoneOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            ContainsAllOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            ContainsAnyOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            EqualToOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            IsOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            IsNotOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            BitwiseAndOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            BitwiseXorOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            BitwiseOrOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            LogicalAndOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            LogicalOrOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            NullCoalesceOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            RangeOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            RangeInclusiveOp(a, b) => {
                self.analyze(a)?;
                self.analyze(b)?;
            }
            SingleRelateId { subject, .. } => {
                self.analyze(subject)?;
            }
            RelateEdgeId { subject, .. } => {
                self.analyze(subject)?;
            }
            RelateOp {
                left, edge, right, ..
            } => {
                self.analyze(left)?;
                self.analyze(edge)?;
                self.analyze(right)?;
            }
            AliasOp { subject, .. } => {
                self.analyze(subject)?;
            }
            Create {
                subject, values, ..
            } => {
                // Analyze the subject.
                self.analyze(subject)?;

                // Analyze the values
                for value in values {
                    for ast in value {
                        self.analyze(ast)?;
                    }
                }
            }
            Relate {
                relate_op, value, ..
            } => {
                // Analyze the relate op
                self.analyze(relate_op)?;

                // Analyze the value
                for ast in value {
                    self.analyze(ast)?;
                }
            }
            Delete {
                target,
                where_guard,
            } => {
                // Analyze the target
                self.analyze(target)?;

                // Analyze the where guard
                if let Some(where_guard) = where_guard {
                    self.analyze(where_guard)?;
                }
            }
            Update {
                target,
                where_guard,
                column_ops,
            } => {
                // Analyze the target
                self.analyze(target)?;

                // Analyze the where guard
                if let Some(where_guard) = where_guard {
                    self.analyze(where_guard)?;
                }

                // Analyze the column ops
                for (_, _, value) in column_ops {
                    self.analyze(value)?;
                }
            }
            Select {
                fields,
                omit,
                from,
                transforms,
            } => {
                // Analyze the fields
                for field in fields {
                    match field {
                        SelectColumn::Column(ast) => self.analyze_with_options(ast, false)?,
                        SelectColumn::Fold { subject, .. } => {
                            self.analyze_with_options(subject, false)?
                        }
                    }
                }

                // Analyze the omit
                for omit in omit {
                    self.analyze_with_options(omit, false)?;
                }

                // Analyze the from
                for ast in from {
                    self.analyze(ast)?;
                }

                // Analyze the transforms
                for transform in transforms {
                    match transform {
                        SelectTransform::WhereGuard(ast) => {
                            self.analyze_with_options(ast, false)?
                        }
                        SelectTransform::WithIndexes(asts) => {
                            for ast in asts {
                                self.analyze_with_options(ast, false)?
                            }
                        }
                        SelectTransform::GroupBy(asts) => {
                            for ast in asts {
                                self.analyze_with_options(ast, false)?
                            }
                        }
                        SelectTransform::LimitTo(ast) => self.analyze_with_options(ast, false)?,
                        SelectTransform::StartAt(ast) => self.analyze_with_options(ast, false)?,
                        SelectTransform::OrderBy { .. } | SelectTransform::WithNoIndex => {}
                    }
                }
            }
            RemoveDatabase {
                subject,
                if_exists,
                namespace,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, namespace)?
                }
            }
            RemoveTable {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            RemoveEdge {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            RemoveType {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            RemoveEnum {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            RemoveIndex {
                subject,
                if_exists,
                database,
                ..
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            RemoveModule {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            RemoveParam {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            DescribeDatabase {
                subject,
                if_exists,
                namespace,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, namespace)?
                }
            }
            DescribeTable {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            DescribeEdge {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            DescribeType {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            DescribeEnum {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            DescribeIndex {
                subject,
                if_exists,
                database,
                ..
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            DescribeModule {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            DescribeParam {
                subject,
                if_exists,
                database,
            } => {
                if !*if_exists {
                    self.analyze_schema_item(subject, database)?
                }
            }
            Set {
                variable, value, ..
            } => {
                // Check if the variable exists
                let var = ast_as!(**variable, Variable(name));
                self.check_variable_or_param_exists(var)?;

                // Analyze the value
                self.analyze(value)?;
            }
            Use { database, .. } => {
                // Check if the database actually exists
                let database_path = ast_as!(**database, Identifier(name)).parse()?;
                let db_not_exists = !self.current_schema.as_ref().map_or(false, |s| {
                    matches!(s.get(&database_path), Some(SchemaMeta::Database()))
                }) && !matches!(
                    self.db_schema.get(&database_path),
                    Some(SchemaMeta::Database())
                );

                if db_not_exists {
                    return Err(SemaError::UndefinedDatabase(database_path));
                }

                // Set the current database
                self.current_database = database_path;
            }
            Wildcard
            | Temp(_)
            | IntegerLiteral(_)
            | FloatLiteral(_)
            | StringLiteral(_)
            | ByteStringLiteral(_)
            | RegexLiteral { .. }
            | BooleanLiteral(_)
            | ModuleBlock(_)
            | NoneLiteral
            | Continue
            | DescribeNamespace { .. }
            | DefineNamespace { .. }
            | RemoveNamespace { .. }
            | BeginTransaction
            | CommitTransaction
            | CancelTransaction
            | Break => {}
        }

        Ok(())
    }

    /// Tags the ast with the current schema
    fn tag_schema(&mut self, ast: &mut Ast) {
        if self.has_untagged_schema {
            ast.set_tag_schema(self.current_schema.clone().unwrap());
            self.has_untagged_schema = false;
        }
    }

    /// Tags the ast with the current symbol table
    fn tag_symbols(&mut self, ast: &mut Ast) {
        if self.has_untagged_symbols {
            ast.set_tag_symbols(self.current_symbols.clone());
            self.has_untagged_symbols = false;
        }
    }

    /// Analyzes a scope.
    fn analyze_scope(
        &mut self,
        pre_fn: impl FnOnce(&mut Self) -> SemaResult<()>,
        body_fn: impl FnOnce(&mut Self) -> SemaResult<()>,
    ) -> SemaResult<()> {
        // Call the pre function
        pre_fn(self)?;

        // Create a new symbol table for the new scope
        let symbols = Symbols::with_parent(&self.current_symbols);

        // Set the new symbol table as the current scope
        self.current_symbols = symbols;

        // Set untagged symbols
        self.has_untagged_symbols = true;

        // Call the body function
        body_fn(self)?;

        // Reset the current scope
        self.current_symbols = self.current_symbols.parent().unwrap().clone();

        Ok(())
    }

    fn analyze_schema_item(&self, name: &Ast, in_database: &Option<Box<Ast>>) -> SemaResult<()> {
        let schema_name = ast_as!(name, Identifier(name));
        self.check_schema_item_exists(schema_name, in_database)
    }

    fn check_schema_item_exists(
        &self,
        name: &str,
        in_database: &Option<Box<Ast>>,
    ) -> SemaResult<()> {
        // Construct schema item path.
        let item_path = self.create_schema_item_path(name, in_database)?;

        // Check if the schema item exists in the current schema or database schema
        let item_not_exists =
            !self.current_schema.as_ref().map_or(false, |s| {
                matches!(s.get(&item_path), Some(SchemaMeta::Database()))
            }) && !matches!(self.db_schema.get(&item_path), Some(SchemaMeta::Database()));

        if item_not_exists {
            return Err(SemaError::UndefinedSchemaItem(item_path));
        }

        Ok(())
    }

    /// Checks if a variable or parameter exists
    pub fn check_variable_or_param_exists(&mut self, name: &str) -> SemaResult<()> {
        if !self.current_symbols.contains(name) {
            let param_path = self.create_schema_item_path(name, &None)?;
            if !matches!(self.db_schema.get(&param_path), Some(SchemaMeta::Param())) {
                return Err(SemaError::UndefinedVariableOrParam(name.to_owned()));
            }
        }

        Ok(())
    }

    /// Checks if database exists

    /// Registers a schema item definition
    fn register_schema_item_definition(
        &mut self,
        name: &Ast,
        in_database: &Option<Box<Ast>>,
        if_not_exists: bool,
        meta: SchemaMeta,
    ) -> SemaResult<()> {
        // Construct the schema name
        let schema_name = ast_as!(name, Identifier(name));
        let schema_path = self.create_schema_item_path(schema_name, in_database)?;

        // Check if the table already exists in the database schema
        if self.db_schema.contains(&schema_path) && !if_not_exists {
            return Err(SemaError::DuplicateSchemaItemDefinition(
                schema_path.clone(),
            ));
        }

        // Create the schema
        let schema = VersionedSchema::new(schema_path, meta, self.current_schema.as_ref());

        // Set the current schema
        self.current_schema = Some(schema);
        self.has_untagged_schema = true;

        Ok(())
    }

    fn create_schema_item_path(
        &self,
        name: &str,
        in_database: &Option<Box<Ast>>,
    ) -> SemaResult<Path> {
        let segment = name.parse()?;

        // Try and use the specified database first
        if let Some(database) = in_database {
            let mut path: Path = ast_as!(**database, Identifier(database)).parse()?;
            path.push(segment);
            return Ok(path);
        }

        // Otherwise, use current set database.
        let mut path = self.current_database.clone();
        path.push(segment);

        Ok(path)
    }

    fn analyze_type(&mut self, _typesig: &TypeSig) -> SemaResult<()> {
        Ok(()) // TODO: Analyze the type
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for NameResolver {
    fn default() -> Self {
        Self::new()
    }
}

//--------------------------------------------------------------------------------------------------
// Macros
//--------------------------------------------------------------------------------------------------

macro_rules! ast_as {
    ($ast:expr, $name:ident ( $($param:ident),* )) => {
        if let Ast {
            kind: $name ($($param),*),
            ..
        } = $ast
        {
            ($($param),*)
        } else {
            return Err($crate::sema::error::SemaError::UnexpectedAstKind($ast.kind.to_string()));
        }
    };
    ($ast:expr, $name:ident { $($param:ident),* }) => {
        if let Ast {
            kind: $name { $($param),* , .. },
            ..
        } = $ast
        {
            ($($param),*)
        } else {
            return Err($crate::sema::error::SemaError::UnexpectedAstKind($ast.kind.to_string()));
        }
    };
}

pub(crate) use ast_as;
use zeroutils_path::Path;

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use tracing::info;

    use crate::parser::Parser;

    use super::*;

    #[test_log::test]
    fn test_sema_name_resolution_variable() -> anyhow::Result<()> {
        // Accessing variables within the same scope
        let mut ast = Parser::new("LET $a = 5; LET $b = 10; SET $a += 1; $a - $b", 50)
            .parse_program()?
            .unwrap();

        let mut resolver = NameResolver::new();
        resolver.analyze(&mut ast)?;

        let statements = ast.kind.unwrap_program();
        let first_ast_symbols = statements[0].get_tag().unwrap().get_symbols().unwrap();

        assert_eq!(first_ast_symbols, &{
            let symbols = Symbols::default();
            symbols.insert("a".to_owned(), SymbolMeta::default());
            symbols.insert("b".to_owned(), SymbolMeta::default());
            symbols
        });

        // Accessing variables from parent scope
        let mut ast = Parser::new(
            r#"
            LET $a = 5;
            FOR $i IN 1..10 DO
                LET $b = 10;
                WHILE $b > 0 DO
                    LET $c = 15;
                    IF $a == 5 THEN
                        LET $d = 20;
                        SET $a += $i;
                        SET $b -= 1;
                        SET $c -= 1;
                        SET $d -= 1;
                    END
                END
            END
            "#,
            50,
        )
        .parse_program()?
        .unwrap();

        let mut resolver = NameResolver::new();
        resolver.analyze(&mut ast)?;

        info!("ast = {:#?}", ast);
        info!("symbols = {:#?}", resolver.current_symbols);

        // // Shadowing variables within the same scope

        // // Shadowing variables in nested scopes

        // // Fails - Using undefined variable
        // let mut ast = Parser::new("LET $a = 5; LET $b = 10; SET $a += 1; $a - $b + $c", 50)
        //     .parse_program()?
        //     .unwrap();

        // let mut resolver = NameResolver::new();
        // let result = resolver.analyze(&mut ast);

        // assert_eq!(
        //     result,
        //     Err(SemaError::UndefinedVariableOrParam("c".to_owned()))
        // );

        // // Fails - Using undefined variable

        Ok(())
    }
}
