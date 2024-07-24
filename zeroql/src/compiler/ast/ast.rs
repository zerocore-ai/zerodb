use crate::Span;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The abstract syntax tree (AST) of the zeroql language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast<'a> {
    /// The span of the AST node in the input string.
    pub span: Span,

    /// The kind of the AST node.
    pub kind: AstKind<'a>,
}

/// The kind of an AST node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstKind<'a> {
    /// An identifier.
    Identifier(&'a str),

    // /// A binary integer literal.
    // BinIntegerLiteral(&'a str),

    // /// An octal integer literal.
    // OctIntegerLiteral(&'a str),

    // /// A hexadecimal integer literal.
    // HexIntegerLiteral(&'a str),
    /// A decimal integer literal.
    DecIntegerLiteral(&'a str),

    // /// A floating-point literal.
    // FloatLiteral(&'a str),
    /// A string literal.
    StringLiteral(&'a str),
    // /// A regular expression literal.
    // RegexLiteral(&'a str),

    // /// A symbol literal.
    // SymbolLiteral(&'a str),

    // /// A boolean literal.
    // BooleanLiteral(bool),
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a> Ast<'a> {
    /// Creates a new AST node.
    pub fn new(span: Span, kind: AstKind<'a>) -> Self {
        Self { span, kind }
    }
}
