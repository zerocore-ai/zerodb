use logos::Logos;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Lexer
#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    /// Whitespaces and comments
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"#.*", logos::skip)]
    Whitespace,

    /// Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier(&'a str),

    /// Binary integer literals
    #[regex(r"0b[01]+(_?[01])*")]
    BinIntegerLiteral(&'a str),

    /// Octal integer literals
    #[regex(r"0o[0-7]+(_?[0-7])*")]
    OctIntegerLiteral(&'a str),

    /// Hexadecimal integer literals
    #[regex(r"0x[0-9a-fA-F]+(_?[0-9a-fA-F])*")]
    HexIntegerLiteral(&'a str),

    /// Decimal integer literals
    #[regex(r"\d(_?\d)*")]
    DecIntegerLiteral(&'a str),

    /// Float literals
    #[regex(r"\.\d(_?\d)*([eE][+-]?\d(_?\d)*)?|\d(_?\d)*\.(\d(_?\d)*)?([eE][+-]?\d(_?\d)*)?|\d(_?\d)*([eE][+-]?\d(_?\d)*)")]
    FloatLiteral(&'a str),

    /// String literals
    #[regex(r"'([^'\\]|\\t|\\n|\\r|\\\\)*'", |lex| lex.slice().trim_matches('\'').to_string())]
    #[regex(r#""([^"\\]|\\t|\\n|\\r|\\\\)*""#, |lex| lex.slice().trim_matches('"').to_string())]
    StringLiteral(String),

    /// Regex literals
    #[regex(r#"//[^/\n]+//"#, |lex| lex.slice().trim_matches('/').to_string())]
    RegexLiteral(String),

    /// Symbol literals
    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().trim_start_matches('@'))]
    SymbolLiteral(&'a str),

    /// Boolean literals
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    BooleanLiteral(bool),

    /// Keyword `type`
    #[token("type")]
    KeywordType,

    /// Keyword `trait`
    #[token("trait")]
    KeywordTrait,

    /// Keyword `import`
    #[token("import")]
    KeywordImport,

    /// Keyword `export`
    #[token("export")]
    KeywordExport,

    /// Keyword `let`
    #[token("let")]
    KeywordLet,

    /// Keyword `in`
    #[token("in")]
    KeywordIn,

    /// Keyword `transaction`
    #[token("transaction")]
    KeywordTransaction,

    /// Keyword `if`
    #[token("if")]
    KeywordIf,

    /// Keyword `else`
    #[token("else")]
    KeywordElse,

    /// Keyword `for`
    #[token("for")]
    KeywordFor,

    /// Keyword `while`
    #[token("while")]
    KeywordWhile,

    /// Keyword `continue`
    #[token("continue")]
    KeywordContinue,

    /// Keyword `break`
    #[token("break")]
    KeywordBreak,

    /// Keyword `match`
    #[token("match")]
    KeywordMatch,

    /// Keyword `fun`
    #[token("fun")]
    KeywordFun,

    /// Keyword `return`
    #[token("return")]
    KeywordReturn,

    /// Operator `+`
    #[token("+")]
    OpPlus,

    /// Operator `-`
    #[token("-")]
    OpMinus,

    /// Operator `*`
    #[token("*")]
    OpMul,

    /// Operator `/`
    #[token("/")]
    OpDiv,

    /// Operator `%`
    #[token("%")]
    OpMod,

    /// Operator `^`
    #[token("^")]
    OpPow,

    /// Operator `.`
    #[token(".")]
    OpDot,

    /// Operator `::`
    #[token("::")]
    OpScope,

    /// Operator `->`
    #[token("->")]
    OpRelate,

    /// Operator `-!>`
    #[token("-!>")]
    OpRelateNeg,

    /// Operator `=`
    #[token("=")]
    OpAssign,

    /// Operator `+=`
    #[token("+=")]
    OpAssignAdd,

    /// Operator `-=`
    #[token("-=")]
    OpAssignSub,

    /// Operator `*=`
    #[token("*=")]
    OpAssignMul,

    /// Operator `/=`
    #[token("/=")]
    OpAssignDiv,

    /// Operator `%=`
    #[token("%=")]
    OpAssignMod,

    /// Operator `^=`
    #[token("^=")]
    OpAssignPow,

    /// Operator `&&`
    #[token("&&")]
    OpAnd,

    /// Operator `||`
    #[token("||")]
    OpOr,

    /// Operator `!`
    #[token("!")]
    OpNot,

    /// Operator `==`
    #[token("==")]
    OpEq,

    /// Operator `!=`
    #[token("!=")]
    OpNe,

    /// Operator `<`
    #[token("<")]
    OpLt,

    /// Operator `<=`
    #[token("<=")]
    OpLe,

    /// Operator `>`
    #[token(">")]
    OpGt,

    /// Operator `>=`
    #[token(">=")]
    OpGe,

    /// Operator `&`
    #[token("&")]
    OpBitAnd,

    /// Operator `|`
    #[token("|")]
    OpBitOr,

    /// Operator `~`
    #[token("~")]
    OpBitNot,

    /// Operator `<<`
    #[token("<<")]
    OpBitShl,

    /// Operator `>>`
    #[token(">>")]
    OpBitShr,

    /// Operator `&=`
    #[token("&=")]
    OpAssignBitAnd,

    /// Operator `|=`
    #[token("|=")]
    OpAssignBitOr,

    /// Operator `~=`
    #[token("~=")]
    OpAssignBitNot,

    /// Operator `<<=`
    #[token("<<=")]
    OpAssignBitShl,

    /// Operator `>>=`
    #[token(">>=")]
    OpAssignBitShr,

    /// Operator `..`
    #[token("..")]
    OpRange,

    /// Operator `..=`
    #[token("..=")]
    OpRangeInclusive,

    /// Operator `=>`
    #[token("=>")]
    OpArrow,

    /// Operator `...`
    #[token("...")]
    OpEllipsis,

    /// Operator `|>`
    #[token("|>")]
    OpPipe,
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_identifier() {
        // Identifier
        let mut lexer = Token::lexer("hello World_0 _world _0world");

        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("hello"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("World_0"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("_world"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("_0world"))));
    }

    #[test]
    fn test_lexer_bin_integer() {
        // Binary integer
        let mut lexer = Token::lexer("0b0 0b1 0b10 0b11 0b1_000 0b0_111");

        assert_eq!(lexer.next(), Some(Ok(Token::BinIntegerLiteral("0b0"))));
        assert_eq!(lexer.next(), Some(Ok(Token::BinIntegerLiteral("0b1"))));
        assert_eq!(lexer.next(), Some(Ok(Token::BinIntegerLiteral("0b10"))));
        assert_eq!(lexer.next(), Some(Ok(Token::BinIntegerLiteral("0b11"))));
        assert_eq!(lexer.next(), Some(Ok(Token::BinIntegerLiteral("0b1_000"))));
        assert_eq!(lexer.next(), Some(Ok(Token::BinIntegerLiteral("0b0_111"))));
    }

    #[test]
    fn test_lexer_oct_integer() {
        // Octal integer
        let mut lexer = Token::lexer("0o0 0o01234567 0o01_23_45_67");

        assert_eq!(lexer.next(), Some(Ok(Token::OctIntegerLiteral("0o0"))));
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::OctIntegerLiteral("0o01234567")))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::OctIntegerLiteral("0o01_23_45_67")))
        );
    }

    #[test]
    fn test_lexer_hex_integer() {
        // Hexadecimal integer
        let mut lexer = Token::lexer("0x0 0x0123456789abcdef 0x01_23_45_67_89_ab_cd_ef");

        assert_eq!(lexer.next(), Some(Ok(Token::HexIntegerLiteral("0x0"))));
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::HexIntegerLiteral("0x0123456789abcdef")))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::HexIntegerLiteral("0x01_23_45_67_89_ab_cd_ef")))
        );
    }

    #[test]
    fn test_lexer_dec_integer() {
        // Decimal integer
        let mut lexer = Token::lexer("0 1234 1_234 0_123_456_789");

        assert_eq!(lexer.next(), Some(Ok(Token::DecIntegerLiteral("0"))));
        assert_eq!(lexer.next(), Some(Ok(Token::DecIntegerLiteral("1234"))));
        assert_eq!(lexer.next(), Some(Ok(Token::DecIntegerLiteral("1_234"))));
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::DecIntegerLiteral("0_123_456_789")))
        );
    }

    #[test]
    fn test_lexer_float() {
        // Float with a final dot
        let mut lexer = Token::lexer("1. 0. 1_000. 0_123.");

        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("1."))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("0."))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("1_000."))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("0_123."))));

        // Float with a dot followed by exponent part
        let mut lexer = Token::lexer("1.e1 0.e+0 1_000.e-1_000 0_123.E0_123");

        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("1.e1"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("0.e+0"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("1_000.e-1_000"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("0_123.E0_123"))));

        // Float with a leading dot
        let mut lexer = Token::lexer(".1 .0 .1_000 .0_123");

        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(".1"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(".0"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(".1_000"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(".0_123"))));

        // Float with a leading dot and exponent part
        let mut lexer = Token::lexer(".1e1 .0e+0 .1_000e-1_000 .0_123E0_123");

        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(".1e1"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(".0e+0"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(".1_000e-1_000"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(".0_123E0_123"))));

        // Float with a dot followed by a fraction part
        let mut lexer = Token::lexer("1.1 0.0 1_000.1_000 0_123.0_123");

        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("1.1"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("0.0"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("1_000.1_000"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("0_123.0_123"))));

        // Float with a dot followed by a fraction part and exponent part
        let mut lexer = Token::lexer("1.1e1 0.0e+0 1_000.1_000e-1_000 0_123.0_123E0_123");

        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("1.1e1"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("0.0e+0"))));
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::FloatLiteral("1_000.1_000e-1_000")))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::FloatLiteral("0_123.0_123E0_123")))
        );

        // Float without fraction part followed by exponent part
        let mut lexer = Token::lexer("1e1 0e+0 1_000e-1_000 0_123E0_123");

        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("1e1"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("0e+0"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("1_000e-1_000"))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral("0_123E0_123"))));
    }

    #[test]
    fn test_lexer_string() {
        // Single-quoted string
        let mut lexer = Token::lexer("'Hello, World!'");

        assert_eq!(
            lexer.next(),
            Some(Ok(Token::StringLiteral("Hello, World!".to_string())))
        );

        // Double-quoted string
        let mut lexer = Token::lexer(r#""Hello, World!""#);

        assert_eq!(
            lexer.next(),
            Some(Ok(Token::StringLiteral("Hello, World!".to_string())))
        );
    }

    #[test]
    fn test_lexer_regex() {
        // Regex
        let mut lexer = Token::lexer(r"//[a-zA-Z_][a-zA-Z0-9_]*//");

        assert_eq!(
            lexer.next(),
            Some(Ok(Token::RegexLiteral(
                r"[a-zA-Z_][a-zA-Z0-9_]*".to_string()
            )))
        );
    }

    #[test]
    fn test_lexer_symbol() {
        // Symbol
        let mut lexer = Token::lexer("@hello @World_0 @_world @_0world");

        assert_eq!(lexer.next(), Some(Ok(Token::SymbolLiteral("hello"))));
        assert_eq!(lexer.next(), Some(Ok(Token::SymbolLiteral("World_0"))));
        assert_eq!(lexer.next(), Some(Ok(Token::SymbolLiteral("_world"))));
        assert_eq!(lexer.next(), Some(Ok(Token::SymbolLiteral("_0world"))));
    }

    #[test]
    fn test_lexer_boolean() {
        // Boolean
        let mut lexer = Token::lexer("true false");

        assert_eq!(lexer.next(), Some(Ok(Token::BooleanLiteral(true))));
        assert_eq!(lexer.next(), Some(Ok(Token::BooleanLiteral(false))));
    }

    #[test]
    fn test_lexer_keyword() {
        // Keyword
        let mut lexer = Token::lexer(
            "type trait import export let in transaction if else for while continue break match fun return",
        );

        assert_eq!(lexer.next(), Some(Ok(Token::KeywordType)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordTrait)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordImport)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordExport)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordLet)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordIn)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordTransaction)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordIf)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordElse)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordFor)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordWhile)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordContinue)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordBreak)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordMatch)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordFun)));
        assert_eq!(lexer.next(), Some(Ok(Token::KeywordReturn)));
    }

    #[test]
    fn test_lexer_operator() {
        // Operator
        let mut lexer = Token::lexer(
            "+ - * / % ^ . :: -> -!> = += -= *= /= %= ^= && || ! == != < <= > >= & | ~ << >> &= |= ~= <<= >>= .. ..= => ... |>",
        );

        assert_eq!(lexer.next(), Some(Ok(Token::OpPlus)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpMinus)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpMul)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpDiv)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpMod)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpPow)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpDot)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpScope)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpRelate)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpRelateNeg)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssign)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignAdd)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignSub)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignMul)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignDiv)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignMod)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignPow)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAnd)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpOr)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpNot)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpEq)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpNe)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpLt)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpLe)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpGt)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpGe)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpBitAnd)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpBitOr)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpBitNot)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpBitShl)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpBitShr)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignBitAnd)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignBitOr)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignBitNot)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignBitShl)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpAssignBitShr)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpRange)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpRangeInclusive)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpArrow)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpEllipsis)));
        assert_eq!(lexer.next(), Some(Ok(Token::OpPipe)));
    }
}
