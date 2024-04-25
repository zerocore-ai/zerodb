use crate::Span;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    /// The span of the token in the input string.
    pub span: Span,

    /// The kind of the token.
    pub kind: TokenKind<'a>,
}

/// The kind of a token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind<'a> {
    /// An identifier.
    Identifier(&'a str),

    /// A binary integer literal.
    BinIntegerLiteral(&'a str),

    /// An octal integer literal.
    OctIntegerLiteral(&'a str),

    /// A hexadecimal integer literal.
    HexIntegerLiteral(&'a str),

    /// A decimal integer literal.
    DecIntegerLiteral(&'a str),

    /// A floating-point literal.
    FloatLiteral(&'a str),

    /// A string literal.
    StringLiteral(&'a str),

    /// A regular expression literal.
    RegexLiteral(&'a str),

    /// A symbol literal.
    SymbolLiteral(&'a str),

    /// A boolean literal.
    BooleanLiteral(bool),

    /// Keyword `type`
    KeywordType,

    /// Keyword `trait`
    KeywordTrait,

    /// Keyword `import`
    KeywordImport,

    /// Keyword `export`
    KeywordExport,

    /// Keyword `let`
    KeywordLet,

    /// Keyword `in`
    KeywordIn,

    /// Keyword `transaction`
    KeywordTransaction,

    /// Keyword `if`
    KeywordIf,

    /// Keyword `else`
    KeywordElse,

    /// Keyword `for`
    KeywordFor,

    /// Keyword `while`
    KeywordWhile,

    /// Keyword `continue`
    KeywordContinue,

    /// Keyword `break`
    KeywordBreak,

    /// Keyword `return`
    KeywordReturn,

    /// Keyword `match`
    KeywordMatch,

    /// Keyword `fun`
    KeywordFun,

    /// Operator `+`
    OpPlus,

    /// Operator `-`
    OpMinus,

    /// Operator `*`
    OpMul,

    /// Operator `/`
    OpDiv,

    /// Operator `%`
    OpMod,

    /// Operator `^`
    OpPow,

    /// Operator `.`
    OpDot,

    /// Operator `::`
    OpScope,

    /// Operator `->`
    OpArrow,

    /// Operator `-!>`
    OpRelateNeg,

    /// Operator `=`
    OpAssign,

    /// Operator `+=`
    OpAssignAdd,

    /// Operator `-=`
    OpAssignSub,

    /// Operator `*=`
    OpAssignMul,

    /// Operator `/=`
    OpAssignDiv,

    /// Operator `%=`
    OpAssignMod,

    /// Operator `^=`
    OpAssignPow,

    /// Operator `&&`
    OpAnd,

    /// Operator `||`
    OpOr,

    /// Operator `!`
    OpNot,

    /// Operator `==`
    OpEq,

    /// Operator `!=`
    OpNe,

    /// Operator `<`
    OpLt,

    /// Operator `<=`
    OpLe,

    /// Operator `>`
    OpGt,

    /// Operator `>=`
    OpGe,

    /// Operator `&`
    OpBitAnd,

    /// Operator `|`
    OpBitOr,

    /// Operator `~`
    OpBitNot,

    /// Operator `<<`
    OpBitShl,

    /// Operator `>>`
    OpBitShr,

    /// Operator `&=`
    OpAssignBitAnd,

    /// Operator `|=`
    OpAssignBitOr,

    /// Operator `<<=`
    OpAssignBitShl,

    /// Operator `>>=`
    OpAssignBitShr,

    /// Operator `..`
    OpRange,

    /// Operator `..=`
    OpRangeInclusive,

    /// Operator `...`
    OpEllipsis,

    /// Operator `|>`
    OpPipe,

    /// Operator `,`
    OpComma,

    /// Operator `:`
    OpColon,

    /// Operator `;`
    OpSemicolon,

    /// Operator `(`
    OpLParen,

    /// Operator `)`
    OpRParen,

    /// Operator `[`
    OpLBracket,

    /// Operator `]`
    OpRBracket,

    /// Operator `{`
    OpLBrace,

    /// Operator `}`
    OpRBrace,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a> Token<'a> {
    /// Creates a new token with the given span and kind.
    pub fn new(span: Span, kind: TokenKind<'a>) -> Self {
        Self { span, kind }
    }

    /// Returns the span of the token.
    pub fn span(&self) -> Span {
        self.span.clone()
    }

    /// Returns the kind of the token.
    pub fn kind(&self) -> &TokenKind<'a> {
        &self.kind
    }
}
