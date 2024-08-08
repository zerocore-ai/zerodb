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

    /// An access operation.
    Access {
        /// The subject of the access operation.
        subject: Box<Ast<'a>>,

        /// The field of the access operation.
        field: Box<Ast<'a>>,
    },

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
    DotAccessOp(Box<Ast<'a>>, Box<Ast<'a>>),

    /// A safe navigation access operation.
    SafeNavigationAccessOp(Box<Ast<'a>>, Box<Ast<'a>>),

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

    /// A relate edge id operation.
    RelateEdgeId(Box<Ast<'a>>, Option<Box<Ast<'a>>>),

    /// A relate edge id not operation.
    RelateEdgeIdNotOp(Box<Ast<'a>>),

    /// A relate operation.
    RelateOp(
        Box<Ast<'a>>,
        RelateArrow,
        Box<Ast<'a>>,
        RelateArrow,
        Box<Ast<'a>>,
    ),

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

        /// The where guard of the relate operation.
        where_guard: Option<Box<Ast<'a>>>,
    },
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
