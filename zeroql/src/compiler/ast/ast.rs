use crate::{lexer::RegexFlags, Span};

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
    /// For intermediate nodes that won't make it into the final AST.
    Temp,

    /// An identifier.
    Identifier(&'a str),

    /// A binary integer literal.
    IntegerLiteral(u128),

    /// A floating-point literal.
    FloatLiteral(f64),

    /// A string literal.
    StringLiteral(&'a str),

    /// A byte string literal.
    ByteStringLiteral(&'a str),

    /// A regular expression literal.
    RegexLiteral(&'a str, RegexFlags),

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

    /// ...
    Op(),
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
}
