use std::fmt::Display;

use crate::{
    lexer::RegexFlags,
    parser::Combinator,
    sema::{Symbols, VersionedSchema},
    Span,
};

use super::AnalysisTag;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The abstract syntax tree (AST) of the zeroql language.
#[derive(Debug, Clone, PartialEq)]
pub struct Ast<'a> {
    /// The span of the AST node in the input string.
    pub span: Span,

    /// The kind of the AST node.
    pub kind: AstKind<'a>,

    /// The tag of the AST node.
    pub tag: Option<AnalysisTag>,
}

/// The kind of an AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum AstKind<'a> {
    /// For intermediate nodes representing partial syntax that may not necessarily make it into
    /// the final AST.
    Temp(Option<Box<Combinator<Ast<'a>>>>),

    /// A wildcard expression.
    Wildcard,

    /// An identifier.
    Identifier(&'a str),

    /// A variable identifier.
    Variable(&'a str),

    /// A target id operation.
    IdOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A scoped identifier.
    ScopedIdentifier(Vec<Ast<'a>>),

    /// A binary integer literal.
    IntegerLiteral(u128),

    /// A floating-point literal.
    FloatLiteral(f64),

    /// A string literal.
    StringLiteral(&'a str),

    /// A byte string literal.
    ByteStringLiteral(&'a str),

    /// A regular expression literal.
    RegexLiteral {
        /// The pattern of the regular expression.
        pattern: &'a str,

        /// The flags of the regular expression.
        flags: RegexFlags,
    },

    /// A boolean literal.
    BooleanLiteral(bool),

    /// A none literal.
    NoneLiteral,

    /// A module block.
    ModuleBlock(&'a str),

    /// A list literal.
    ListLiteral(Vec<Ast<'a>>),

    /// A tuple literal.
    TupleLiteral(Vec<Ast<'a>>),

    /// An object literal.
    ObjectLiteral(Vec<(Ast<'a>, Ast<'a>)>),

    /// An index operation.
    Index {
        /// The subject of the index operation.
        subject: Box<Ast<'a>>,

        /// The index of the index operation.
        index: Box<Ast<'a>>,
    },

    /// A function argument.
    FunctionArg {
        /// The name of the function argument.
        name: Option<Box<Ast<'a>>>,

        /// The value of the function argument.
        value: Box<Ast<'a>>,
    },

    /// A function call operation.
    FunctionCall {
        /// The subject of the function call operation.
        subject: Box<Ast<'a>>,

        /// The arguments of the function call operation.
        args: Vec<Ast<'a>>,
    },

    /// A logical not operation.
    LogicalNotOp(Box<Ast<'a>>),

    /// A bitwise not operation.
    BitwiseNotOp(Box<Ast<'a>>),

    /// A plus sign operation.
    PlusSignOp(Box<Ast<'a>>),

    /// A minus sign operation.
    MinusSignOp(Box<Ast<'a>>),

    /// An access operation.
    DotAccessOp {
        /// The subject of the access operation.
        subject: Box<Ast<'a>>,

        /// The field of the access operation.
        field: Box<Ast<'a>>,
    },

    /// A safe navigation access operation.
    SafeNavigationAccessOp {
        /// The subject of the safe navigation access operation.
        subject: Box<Ast<'a>>,

        /// The field of the safe navigation access operation.
        field: Box<Ast<'a>>,
    },

    /// An access operation with `*` as the field.
    DotAccessWildcardOp {
        /// The subject of the access operation.
        subject: Box<Ast<'a>>,
    },

    /// A exponentiation operation.
    ExponentiationOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A multiplication operation.
    MultiplicationOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A division operation.
    DivisionOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A modulus operation.
    ModulusOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A addition operation.
    AdditionOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A subtraction operation.
    SubtractionOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A left shift operation.
    LeftShiftOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A right shift operation.
    RightShiftOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A match operation.
    MatchOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A not match operation.
    NotMatchOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A similarity operation.
    SimilarityOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A less than operation.
    LessThanOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A greater than operation.
    GreaterThanOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A less than or equal to operation.
    LessThanEqualToOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A greater than or equal to operation.
    GreaterThanEqualToOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// An in operation.
    InOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A not in operation.
    NotInOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A contains operation.
    ContainsOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A not contains operation.
    NotContainsOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A contains none operation.
    ContainsNoneOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A contains all operation.
    ContainsAllOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A contains any operation.
    ContainsAnyOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A equal to operation.
    EqualToOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A is operation.
    IsOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A is not operation.
    IsNotOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A bitwise and operation.
    BitwiseAndOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A bitwise xor operation.
    BitwiseXorOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A bitwise or operation.
    BitwiseOrOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A logical and operation.
    LogicalAndOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A logical or operation.
    LogicalOrOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A null coalest operation.
    NullCoalesceOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A range operation.
    RangeOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A range inclusive operation.
    RangeInclusiveOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A relate id operation.
    SingleRelateId {
        /// The subject of the relate id operation.
        subject: Box<Ast<'a>>,

        /// The alias of the relate id operation.
        alias: Option<Box<Ast<'a>>>,
    },

    /// A single relate edge id operation.
    RelateEdgeId {
        /// The subject of the single relate edge id operation.
        subject: Box<Ast<'a>>,

        /// The depth of the relate edge id operation.
        depth: Option<Box<Ast<'a>>>,

        /// The alias of the relate edge id operation.
        alias: Option<Box<Ast<'a>>>,
    },

    /// A relate operation.
    RelateOp {
        /// The left subject of the relate operation.
        left: Box<Ast<'a>>,

        /// The left operator of the relate operation.
        l_op: RelateArrow,

        /// The edge of the relate operation.
        edge: Box<Ast<'a>>,

        /// The right operator of the relate operation.
        r_op: RelateArrow,

        /// The right subject of the relate operation.
        right: Box<Ast<'a>>,
    },

    /// An alias operation.
    AliasOp {
        /// The subject of the alias operation.
        subject: Box<Ast<'a>>,

        /// The alias of the alias operation.
        alias: Box<Ast<'a>>,
    },

    /// An `CREATE` expression.
    Create {
        /// The subject of the create operation.
        subject: Box<Ast<'a>>,

        /// Columns of the create operation.
        columns: Vec<Ast<'a>>,

        /// A vector of value tuples.
        values: Vec<Vec<Ast<'a>>>,
    },

    /// A `RELATE` expression.
    Relate {
        /// The associated relate operation.
        relate_op: Box<Ast<'a>>,

        /// The columns of the relate operation.
        columns: Vec<Ast<'a>>,

        /// The values of the relate operation.
        value: Vec<Ast<'a>>,
    },

    /// A `DELETE` expression.
    Delete {
        /// The target of the delete operation.
        target: Box<Ast<'a>>,

        /// The where guard of the delete operation.
        where_guard: Option<Box<Ast<'a>>>,
    },

    /// An `UPDATE` expression.
    Update {
        /// The target of the update operation.
        target: Box<Ast<'a>>,

        /// The where guard of the update operation.
        where_guard: Option<Box<Ast<'a>>>,

        /// The column operations of the update operation.
        column_ops: Vec<(Ast<'a>, UpdateAssign, Ast<'a>)>,
    },

    /// A `SELECT` expression.
    Select {
        /// The fields of the select operation.
        fields: Vec<SelectColumn<'a>>,

        /// The fields to omit from the select operation.
        omit: Vec<Ast<'a>>,

        /// The from clause of the select operation.
        from: Vec<Ast<'a>>,

        /// The transforms of the select operation.
        transforms: Vec<SelectTransform<'a>>,
    },

    /// A `REMOVE NAMESPACE` expression.
    RemoveNamespace {
        /// The subject of the remove namespace operation.
        subject: Box<Ast<'a>>,

        /// The namespace existing flag.
        if_exists: bool,
    },

    /// A `REMOVE DATABASE` expression.
    RemoveDatabase {
        /// The subject of the remove database operation.
        subject: Box<Ast<'a>>,

        /// The database existing flag.
        if_exists: bool,

        /// The namespace the database belongs to.
        namespace: Option<Box<Ast<'a>>>,
    },

    /// A `REMOVE TABLE` expression.
    RemoveTable {
        /// The subject of the remove table operation.
        subject: Box<Ast<'a>>,

        /// The table existing flag.
        if_exists: bool,

        /// The database the table belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `REMOVE EDGE` expression.
    RemoveEdge {
        /// The subject of the remove edge operation.
        subject: Box<Ast<'a>>,

        /// The edge existing flag.
        if_exists: bool,

        /// The database the edge belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `REMOVE TYPE` expression.
    RemoveType {
        /// The subject of the remove type operation.
        subject: Box<Ast<'a>>,

        /// The type existing flag.
        if_exists: bool,

        /// The database the type belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `REMOVE ENUM` expression.
    RemoveEnum {
        /// The subject of the remove enum operation.
        subject: Box<Ast<'a>>,

        /// The enum existing flag.
        if_exists: bool,

        /// The database the enum belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `REMOVE INDEX` expression.
    RemoveIndex {
        /// The subject of the remove index operation.
        subject: Box<Ast<'a>>,

        /// The index existing flag.
        if_exists: bool,

        /// The table the index belongs to.
        table: Box<Ast<'a>>,

        /// The database the index belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `REMOVE MODULE` expression.
    RemoveModule {
        /// The subject of the remove module operation.
        subject: Box<Ast<'a>>,

        /// The module existing flag.
        if_exists: bool,

        /// The database the module belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `REMOVE PARAM` expression.
    RemoveParam {
        /// The subject of the remove parameter operation.
        subject: Box<Ast<'a>>,

        /// The parameter existing flag.
        if_exists: bool,

        /// The database the parameter belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `DESCRIBE NAMESPACE` expression.
    DescribeNamespace {
        /// The subject of the describe namespace operation.
        subject: Box<Ast<'a>>,

        /// The namespace existing flag.
        if_exists: bool,
    },

    /// A `DESCRIBE DATABASE` expression.
    DescribeDatabase {
        /// The subject of the describe database operation.
        subject: Box<Ast<'a>>,

        /// The database existing flag.
        if_exists: bool,

        /// The namespace the database belongs to.
        namespace: Option<Box<Ast<'a>>>,
    },

    /// A `DESCRIBE TABLE` expression.
    DescribeTable {
        /// The subject of the describe table operation.
        subject: Box<Ast<'a>>,

        /// The table existing flag.
        if_exists: bool,

        /// The database the table belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `DESCRIBE EDGE` expression.
    DescribeEdge {
        /// The subject of the describe edge operation.
        subject: Box<Ast<'a>>,

        /// The edge existing flag.
        if_exists: bool,

        /// The database the edge belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `DESCRIBE TYPE` expression.
    DescribeType {
        /// The subject of the describe type operation.
        subject: Box<Ast<'a>>,

        /// The type existing flag.
        if_exists: bool,

        /// The database the type belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `DESCRIBE ENUM` expression.
    DescribeEnum {
        /// The subject of the describe enum operation.
        subject: Box<Ast<'a>>,

        /// The enum existing flag.
        if_exists: bool,

        /// The database the enum belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `REMOVE INDEX` expression.
    DescribeIndex {
        /// The subject of the describe index operation.
        subject: Box<Ast<'a>>,

        /// The index existing flag.
        if_exists: bool,

        /// The table the index belongs to.
        table: Box<Ast<'a>>,

        /// The database the index belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `DESCRIBE MODULE` expression.
    DescribeModule {
        /// The subject of the describe module operation.
        subject: Box<Ast<'a>>,

        /// The module existing flag.
        if_exists: bool,

        /// The database the module belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `DESCRIBE PARAM` expression.
    DescribeParam {
        /// The subject of the describe parameter operation.
        subject: Box<Ast<'a>>,

        /// The parameter existing flag.
        if_exists: bool,

        /// The database the parameter belongs to.
        database: Option<Box<Ast<'a>>>,
    },

    /// A `BEGIN TRANSACTION` expression.
    BeginTransaction,

    /// A `COMMIT TRANSACTION` expression.
    CommitTransaction,

    /// A `CANCEL TRANSACTION` expression.
    CancelTransaction,

    /// A `FOR` expression.
    For {
        /// The variable of the for expression.
        variable: Box<Ast<'a>>,

        /// The iterator of the for expression.
        iterator: Box<Ast<'a>>,

        /// The body of the for expression.
        body: Box<Ast<'a>>,
    },

    /// A `WHILE` expression.
    While {
        /// The condition of the while expression.
        condition: Box<Ast<'a>>,

        /// The body of the while expression.
        body: Box<Ast<'a>>,
    },

    /// An `IF` expression.
    If {
        /// The condition of the if expression.
        condition: Box<Ast<'a>>,

        /// The then branch of the if expression.
        then: Box<Ast<'a>>,

        /// The else if parts of the if expression.
        else_ifs: Vec<ElseIfPart<'a>>,

        /// The else branch of the if expression.
        r#else: Option<Box<Ast<'a>>>,
    },

    /// A `LET` expression.
    Let {
        /// The name of the let expression.
        name: Box<Ast<'a>>,

        /// The type of the let expression.
        r#type: Option<Box<TypeSig<'a>>>,

        /// The value of the let expression.
        value: Box<Ast<'a>>,
    },

    /// A `SET` expression.
    Set {
        /// The variable of the set expression.
        variable: Box<Ast<'a>>,

        /// The assignment operator of the set expression.
        op: UpdateAssign,

        /// The value of the set expression.
        value: Box<Ast<'a>>,
    },

    /// A `DEFINE NAMESPACE` statement.
    DefineNamespace {
        /// The name of the namespace.
        name: Box<Ast<'a>>,

        /// The if not exists flag.
        if_not_exists: bool,
    },

    /// A `DEFINE DATABASE` statement.
    DefineDatabase {
        /// The name of the database.
        name: Box<Ast<'a>>,

        /// The if not exists flag.
        if_not_exists: bool,

        /// The namespace the database belongs to.
        namespace: Option<Box<Ast<'a>>>,
    },

    /// A `DEFINE TABLE` statement.
    DefineTable {
        /// The name of the table.
        name: Box<Ast<'a>>,

        /// The if not exists flag.
        if_not_exists: bool,

        /// The database the table belongs to.
        database: Option<Box<Ast<'a>>>,

        /// The fields of the table.
        fields: Vec<Field<'a>>,
    },

    /// A `DEFINE EDGE` statement.
    DefineEdge {
        /// The name of the edge.
        name: Box<Ast<'a>>,

        /// The if not exists flag.
        if_not_exists: bool,

        /// The database the edge belongs to.
        database: Option<Box<Ast<'a>>>,

        /// The fields of the edge.
        fields: Vec<Field<'a>>,
    },

    /// A `DEFINE TYPE` statement.
    DefineType {
        /// The name of the type.
        name: Box<Ast<'a>>,

        /// The if not exists flag.
        if_not_exists: bool,

        /// The database the type belongs to.
        database: Option<Box<Ast<'a>>>,

        /// The fields of the type.
        fields: Vec<(Ast<'a>, TypeSig<'a>)>,
    },

    /// A `DEFINE ENUM` statement.
    DefineEnum {
        /// The name of the enum.
        name: Box<Ast<'a>>,

        /// The if not exists flag.
        if_not_exists: bool,

        /// The database the enum belongs to.
        database: Option<Box<Ast<'a>>>,

        /// The variants of the enum.
        variants: Vec<Ast<'a>>,
    },

    /// A `DEFINE INDEX` statement.
    DefineIndex {
        /// The name of the index.
        name: Box<Ast<'a>>,

        /// The if not exists flag.
        if_not_exists: bool,

        /// The database the index belongs to.
        database: Option<Box<Ast<'a>>>,

        /// The table the index belongs to.
        table: Box<Ast<'a>>,

        /// The columns of the index.
        columns: Vec<Ast<'a>>,

        /// The function to call when the index is created.
        function: Option<Box<Ast<'a>>>,
    },

    /// A `DEFINE MODULE` statement.
    DefineModule {
        /// The name of the module.
        name: Box<Ast<'a>>,

        /// The if not exists flag.
        if_not_exists: bool,

        /// The database the module belongs to.
        database: Option<Box<Ast<'a>>>,

        /// The code block of the module.
        block: Box<Ast<'a>>,
    },

    /// A `DEFINE PARAM` statement.
    DefineParam {
        /// The name of the parameter.
        name: Box<Ast<'a>>,

        /// The if not exists flag.
        if_not_exists: bool,

        /// The database the parameter belongs to.
        database: Option<Box<Ast<'a>>>,

        /// The type of the parameter.
        r#type: Option<TypeSig<'a>>,

        /// The value of the parameter.
        value: Box<Ast<'a>>,
    },

    /// A `USE` statement.
    Use {
        /// The database the parameter belongs to.
        database: Box<Ast<'a>>,
    },

    /// A `BREAK` statement.
    Break,

    /// A `CONTINUE` statement.
    Continue,

    /// A program.
    Program(Vec<Ast<'a>>),
}

/// A field of a table or edge.
#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a> {
    /// The name of the field.
    pub name: Box<Ast<'a>>,

    /// The type of the field.
    pub r#type: TypeSig<'a>,

    /// The default value of the field.
    pub default: Option<Box<Ast<'a>>>,

    /// The assertions of the field.
    pub assertions: Vec<Ast<'a>>,

    /// Whether the field is readonly.
    pub readonly: bool,

    /// Whether the field is unique.
    pub unique: bool,
}

/// A type signature.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeSig<'a> {
    /// An array type, e.g. `[i32: 10]`.
    Array {
        /// The type of the array type.
        r#type: Box<TypeSig<'a>>,

        /// The length of the array type.
        length: Box<Ast<'a>>,
    },

    /// A list type, e.g. `[i32]`.
    List(Box<TypeSig<'a>>),

    /// A tuple type, e.g. `(i32, bool)`.
    Tuple(Vec<TypeSig<'a>>),

    /// An option type, e.g. `person?`.
    Option(Box<TypeSig<'a>>),

    /// A generic type, e.g. `map<k, v>`.
    Generic {
        /// The name of the generic type.
        name: Box<Ast<'a>>,

        /// The parameters of the generic type.
        parameters: Vec<TypeSig<'a>>,
    },

    /// A basic type, e.g. `i32`.
    Basic(Box<Ast<'a>>),
}

/// A partial else if part of an if expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfPart<'a> {
    /// The condition of the else if part.
    pub condition: Box<Ast<'a>>,

    /// The body of the else if part.
    pub body: Box<Ast<'a>>,
}

/// A column or fold column of a `SELECT` expression.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectColumn<'a> {
    /// A column of the select operation.
    Column(Box<Ast<'a>>),

    /// A fold of the select operation.
    Fold {
        /// The subject of the fold.
        subject: Box<Ast<'a>>,

        /// The alias of the fold.
        alias: Option<Box<Ast<'a>>>,
    },
}

/// The transform of a select operation.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectTransform<'a> {
    /// A `WITH NO INDEX` transform.
    WithNoIndex,

    /// A `WITH INDEXES` transform.
    WithIndexes(Vec<Ast<'a>>),

    /// A `WHERE` guard.
    WhereGuard(Box<Ast<'a>>),

    /// A `GROUP BY` transform.
    GroupBy(Vec<Ast<'a>>),

    /// A `ORDER BY` transform.
    OrderBy {
        /// The fields to order by.
        fields: Vec<Ast<'a>>,

        /// The direction of the ordering.
        direction: Direction,
    },

    /// A `LIMIT TO` transform.
    LimitTo(Box<Ast<'a>>),

    /// A `START AT` transform.
    StartAt(Box<Ast<'a>>),
}

/// The direction of an ordering.
#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    /// An ascending ordering.
    Ascending,

    /// A descending ordering.
    Descending,
}

/// The arrow direction of a relate operation.
#[derive(Debug, Clone, PartialEq)]
pub enum RelateArrow {
    /// A `<-` arrow.
    Left,

    /// A `->` arrow.
    Right,

    /// A `<<-` arrow.
    MultiLeft,

    /// A `->>` arrow.
    MultiRight,
}

/// The assignment operator of an update operation.
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateAssign {
    /// A direct assignment.
    Direct,

    /// A `+=` assignment.
    Plus,

    /// A `-=` assignment.
    Minus,

    /// A `*=` assignment.
    Mul,

    /// A `/=` assignment.
    Div,

    /// A `%=` assignment.
    Mod,

    /// A `**=` assignment.
    Pow,

    /// A `&=` assignment.
    BitAnd,

    /// A `|=` assignment.
    BitOr,

    /// A `^=` assignment.
    BitXor,

    /// A `~=` assignment.
    BitNot,

    /// A `<<=` assignment.
    Shl,

    /// A `>>=` assignment.
    Shr,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a> Ast<'a> {
    /// Creates a new AST node.
    pub fn new(span: Span, kind: AstKind<'a>) -> Self {
        Self {
            span,
            kind,
            tag: Default::default(),
        }
    }

    /// Gets the span of the AST node.
    pub fn get_span(&self) -> Span {
        self.span.clone()
    }

    /// Unwraps the AST node from the temporary kind.
    pub(crate) fn unwrap_temp(self) -> Combinator<Ast<'a>> {
        match self.kind {
            AstKind::Temp(x) => *x.unwrap(),
            _ => panic!("AstKind::Temp expected"),
        }
    }

    /// Sets the symbols of the AST node.
    pub fn set_tag_symbols(&mut self, symbols: Symbols) {
        if let Some(tag) = self.tag.as_mut() {
            tag.set_symbols(symbols);
        } else {
            let mut tag = AnalysisTag::default();
            tag.set_symbols(symbols);
            self.tag = Some(tag);
        }
    }

    /// Sets the schema of the AST node.
    pub fn set_tag_schema(&mut self, schema: VersionedSchema) {
        if let Some(tag) = self.tag.as_mut() {
            tag.set_schema(schema);
        } else {
            let mut tag = AnalysisTag::default();
            tag.set_schema(schema);
            self.tag = Some(tag);
        }
    }

    /// Gets the tag of the AST node.
    pub fn get_tag(&self) -> Option<&AnalysisTag> {
        self.tag.as_ref()
    }
}

impl<'a> AstKind<'a> {
    /// Unwraps the AST node from the program kind.
    pub fn unwrap_program(self) -> Vec<Ast<'a>> {
        match self {
            AstKind::Program(statements) => statements,
            _ => panic!("AstKind::Program expected"),
        }
    }
}

impl<'a> Display for AstKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
