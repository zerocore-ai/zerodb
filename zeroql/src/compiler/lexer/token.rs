use std::fmt::Display;

use bitflags::bitflags;

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
    /// Terminator
    Terminator,

    /// A plain identifier like "foo".
    PlainIdentifier(&'a str),

    /// An escaped identifier like "`foo`".
    EscapedIdentifier(&'a str),

    /// A variable.
    Variable(&'a str),

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
    RegexLiteral(&'a str, RegexFlags),

    /// A module block.
    ModuleBlock(&'a str),

    /// "(" bracket.
    OpOpenParen,

    /// ")" bracket.
    OpCloseParen,

    /// "{" bracket.
    OpOpenBrace,

    /// "}" bracket.
    OpCloseBrace,

    /// "[" bracket.
    OpOpenSquareBracket,

    /// "]" bracket.
    OpCloseSquareBracket,

    /// "," operator.
    OpComma,

    /// "::" operator.
    OpScope,

    /// ":" operator.
    OpColon,

    /// "+=" operator.
    OpAssignPlus,

    /// "-=" operator.
    OpAssignMinus,

    /// "*=" operator.
    OpAssignMul,

    /// "/=" operator.
    OpAssignDiv,

    /// "%=" operator.
    OpAssignMod,

    /// "**=" operator.
    OpAssignPow,

    /// "<<=" operator.
    OpAssignShl,

    /// ">>=" operator.
    OpAssignShr,

    /// "&=" operator.
    OpAssignBitAnd,

    /// "|=" operator.
    OpAssignBitOr,

    /// "^=" operator.
    OpAssignBitXor,

    /// "~=" operator.
    OpAssignBitNot,

    /// "??=" operator.
    OpAssignNullCoalesce,

    /// "->>" operator.
    OpMultiArrowRight,

    /// "<<-" operator.
    OpMultiArrowLeft,

    /// "->" operator.
    OpArrowRight,

    /// "<-" operator.
    OpArrowLeft,

    /// "**" operator.
    OpPow,

    /// "+" operator.
    OpPlus,

    /// "-" operator.
    OpMinus,

    /// "×" operator.
    OpMulLexer,

    /// "/" operator.
    OpDiv,

    /// "%" operator.
    OpMod,

    /// "~" operator.
    OpMatchLexer,

    /// "!~" operator.
    OpNotMatchLexer,

    /// "<>" operator.
    OpSimilarity,

    /// "&&" operator.
    OpAndLexer,

    /// "||" operator.
    OpOrLexer,

    /// "==" operator.
    OpEq,

    /// "=" operator.
    OpIsLexer,

    /// "!=" operator.
    OpIsNotLexer,

    /// "!=" operator.
    OpNotLexer,

    /// "<=" operator.
    OpLte,

    /// ">=" operator.
    OpGte,

    /// "<" operator.
    OpLt,

    /// ">" operator.
    OpGt,

    /// "∋" operator.
    OpContainsLexer,

    /// "∌" operator.
    OpNotContainsLexer,

    /// "⊅" operator.
    OpContainsNoneLexer,

    /// "⊇" operator.
    OpContainsAllLexer,

    /// "⊃" operator.
    OpContainsAnyLexer,

    /// "??." operator.
    OpSafeNav,

    /// "??" operator.
    OpNullCoalesce,

    /// "<<" operator.
    OpShl,

    /// ">>" operator.
    OpShr,

    /// "&" operator.
    OpBitAnd,

    /// "|" operator.
    OpBitOr,

    /// "^" operator.
    OpBitXor,

    /// "..=" operator.
    OpRangeIncl,

    /// ".." operator.
    OpRange,

    /// "*" operator.
    OpStar,

    /// "." operator.
    OpDot,
}

bitflags! {
    /// Flags for a regular expression literal.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RegexFlags: u8 {
        /// Global flag.
        const G_GLOBAL = 0b00000001;

        /// Ignore case flag.
        const I_IGNORE_CASE = 0b00000010;

        /// Multiline flag.
        const M_MULTILINE = 0b00000100;

        /// Singleline flag.
        const S_SINGLELINE = 0b00001000;

        /// Unicode flag.
        const U_UNICODE = 0b00010000;

        /// Extended flag.
        const X_EXTENDED = 0b00100000;
    }
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

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<'a> Display for TokenKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Terminator => write!(f, ";"),
            TokenKind::PlainIdentifier(s) => write!(f, "{}", s),
            TokenKind::EscapedIdentifier(s) => write!(f, "`{}`", s),
            TokenKind::Variable(s) => write!(f, "${}", s),
            TokenKind::BinIntegerLiteral(s) => write!(f, "0b{}", s),
            TokenKind::OctIntegerLiteral(s) => write!(f, "0o{}", s),
            TokenKind::HexIntegerLiteral(s) => write!(f, "0x{}", s),
            TokenKind::DecIntegerLiteral(s) => write!(f, "{}", s),
            TokenKind::FloatLiteral(s) => write!(f, "{}", s),
            TokenKind::StringLiteral(s) => write!(f, "'{}'", s),
            TokenKind::RegexLiteral(s, flags) => {
                write!(f, "//{}//{}", s, flags)
            }
            TokenKind::ModuleBlock(s) => write!(f, "{}", s),
            TokenKind::OpOpenParen => write!(f, "("),
            TokenKind::OpCloseParen => write!(f, ")"),
            TokenKind::OpOpenBrace => write!(f, "{{"),
            TokenKind::OpCloseBrace => write!(f, "}}"),
            TokenKind::OpOpenSquareBracket => write!(f, "[["),
            TokenKind::OpCloseSquareBracket => write!(f, "]]"),
            TokenKind::OpComma => write!(f, ","),
            TokenKind::OpScope => write!(f, "::"),
            TokenKind::OpColon => write!(f, ":"),
            TokenKind::OpAssignPlus => write!(f, "+="),
            TokenKind::OpAssignMinus => write!(f, "-="),
            TokenKind::OpAssignMul => write!(f, "×="),
            TokenKind::OpAssignDiv => write!(f, "/="),
            TokenKind::OpAssignMod => write!(f, "%="),
            TokenKind::OpAssignPow => write!(f, "**="),
            TokenKind::OpAssignShl => write!(f, "<<="),
            TokenKind::OpAssignShr => write!(f, ">>="),
            TokenKind::OpAssignBitAnd => write!(f, "&="),
            TokenKind::OpAssignBitOr => write!(f, "|="),
            TokenKind::OpAssignBitXor => write!(f, "^="),
            TokenKind::OpAssignBitNot => write!(f, "~="),
            TokenKind::OpAssignNullCoalesce => write!(f, "??="),
            TokenKind::OpMultiArrowRight => write!(f, "->>"),
            TokenKind::OpMultiArrowLeft => write!(f, "<<-"),
            TokenKind::OpArrowRight => write!(f, "->"),
            TokenKind::OpArrowLeft => write!(f, "<-"),
            TokenKind::OpPow => write!(f, "**"),
            TokenKind::OpPlus => write!(f, "+"),
            TokenKind::OpMinus => write!(f, "-"),
            TokenKind::OpMulLexer => write!(f, "×"),
            TokenKind::OpDiv => write!(f, "/"),
            TokenKind::OpMod => write!(f, "%"),
            TokenKind::OpMatchLexer => write!(f, "~"),
            TokenKind::OpNotMatchLexer => write!(f, "!~"),
            TokenKind::OpSimilarity => write!(f, "<>"),
            TokenKind::OpAndLexer => write!(f, "&&"),
            TokenKind::OpOrLexer => write!(f, "||"),
            TokenKind::OpEq => write!(f, "=="),
            TokenKind::OpIsLexer => write!(f, "="),
            TokenKind::OpIsNotLexer => write!(f, "!="),
            TokenKind::OpNotLexer => write!(f, "!"),
            TokenKind::OpLte => write!(f, "<="),
            TokenKind::OpGte => write!(f, ">="),
            TokenKind::OpLt => write!(f, "<"),
            TokenKind::OpGt => write!(f, ">"),
            TokenKind::OpContainsLexer => write!(f, "∋"),
            TokenKind::OpNotContainsLexer => write!(f, "∌"),
            TokenKind::OpContainsNoneLexer => write!(f, "⊅"),
            TokenKind::OpContainsAllLexer => write!(f, "⊇"),
            TokenKind::OpContainsAnyLexer => write!(f, "⊃"),
            TokenKind::OpSafeNav => write!(f, "??."),
            TokenKind::OpNullCoalesce => write!(f, "??"),
            TokenKind::OpShl => write!(f, "<<"),
            TokenKind::OpShr => write!(f, ">>"),
            TokenKind::OpBitAnd => write!(f, "&"),
            TokenKind::OpBitOr => write!(f, "|"),
            TokenKind::OpBitXor => write!(f, "^"),
            TokenKind::OpRangeIncl => write!(f, "..="),
            TokenKind::OpRange => write!(f, ".."),
            TokenKind::OpStar => write!(f, "*"),
            TokenKind::OpDot => write!(f, "."),
        }
    }
}

impl From<&str> for RegexFlags {
    fn from(s: &str) -> Self {
        let mut flags = RegexFlags::empty();
        if s.contains('g') {
            flags.set(RegexFlags::G_GLOBAL, true);
        }
        if s.contains('i') {
            flags.set(RegexFlags::I_IGNORE_CASE, true);
        }
        if s.contains('m') {
            flags.set(RegexFlags::M_MULTILINE, true);
        }
        if s.contains('s') {
            flags.set(RegexFlags::S_SINGLELINE, true);
        }
        if s.contains('u') {
            flags.set(RegexFlags::U_UNICODE, true);
        }
        if s.contains('x') {
            flags.set(RegexFlags::X_EXTENDED, true);
        }
        flags
    }
}

impl Default for RegexFlags {
    fn default() -> Self {
        Self::empty()
    }
}

impl Display for RegexFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if self.contains(RegexFlags::G_GLOBAL) {
            s.push('g');
        }
        if self.contains(RegexFlags::I_IGNORE_CASE) {
            s.push('i');
        }
        if self.contains(RegexFlags::M_MULTILINE) {
            s.push('m');
        }
        if self.contains(RegexFlags::S_SINGLELINE) {
            s.push('s');
        }
        if self.contains(RegexFlags::U_UNICODE) {
            s.push('u');
        }
        if self.contains(RegexFlags::X_EXTENDED) {
            s.push('x');
        }
        write!(f, "{}", s)
    }
}
