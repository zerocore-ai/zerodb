use crate::{lexer::RegexFlags, parser::Combinator, Span};

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

    /// A `??=` assignment.
    NullCoalesce,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a> Ast<'a> {
    /// Creates a new AST node.
    pub fn new(span: Span, kind: AstKind<'a>) -> Self {
        Self { span, kind }
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
}
