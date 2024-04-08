use crate::{Lexer, Token, TokenKind};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_lexer_identifier() {
    // Identifier
    let mut lexer = Lexer::from("hello World_0 _world _0world");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..5, TokenKind::Identifier("hello"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(6..13, TokenKind::Identifier("World_0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(14..20, TokenKind::Identifier("_world"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(21..28, TokenKind::Identifier("_0world"))
    );
    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_bin_integer() {
    // Binary integer
    let mut lexer = Lexer::from("0b0 0b1 0b10 0b11  0b1_000  0b0_111");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..3, TokenKind::BinIntegerLiteral("0b0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..7, TokenKind::BinIntegerLiteral("0b1"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(8..12, TokenKind::BinIntegerLiteral("0b10"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(13..17, TokenKind::BinIntegerLiteral("0b11"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(19..26, TokenKind::BinIntegerLiteral("0b1_000"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(28..35, TokenKind::BinIntegerLiteral("0b0_111"))
    );
    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_oct_integer() {
    // Octal integer
    let mut lexer = Lexer::from("0o0 0o01234567 \t0o01_23_45_67");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..3, TokenKind::OctIntegerLiteral("0o0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..14, TokenKind::OctIntegerLiteral("0o01234567"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(16..29, TokenKind::OctIntegerLiteral("0o01_23_45_67"))
    );
    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_hex_integer() {
    // Hexadecimal integer
    let mut lexer = Lexer::from("0x0\t0x0123456789abcdef\n  0x01_23_45_67_89_ab_cd_ef");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..3, TokenKind::HexIntegerLiteral("0x0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..22, TokenKind::HexIntegerLiteral("0x0123456789abcdef"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(
            25..50,
            TokenKind::HexIntegerLiteral("0x01_23_45_67_89_ab_cd_ef")
        )
    );
    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_dec_integer() {
    // Decimal integer
    let mut lexer = Lexer::from("0 1234 1_234 0_123_456_789");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..1, TokenKind::DecIntegerLiteral("0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(2..6, TokenKind::DecIntegerLiteral("1234"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(7..12, TokenKind::DecIntegerLiteral("1_234"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(13..26, TokenKind::DecIntegerLiteral("0_123_456_789"))
    );
    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_float() {
    // Float with a final dot
    let mut lexer = Lexer::from("1. 0. 1_000. 0_123.");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..2, TokenKind::FloatLiteral("1."))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(3..5, TokenKind::FloatLiteral("0."))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(6..12, TokenKind::FloatLiteral("1_000."))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(13..19, TokenKind::FloatLiteral("0_123."))
    );
    assert!(lexer.next().is_none());

    // Float with a dot followed by exponent part
    let mut lexer = Lexer::from("1.e1 0.e+0 1_000.e-1_000 0_123.E0_123");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..4, TokenKind::FloatLiteral("1.e1"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(5..10, TokenKind::FloatLiteral("0.e+0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(11..24, TokenKind::FloatLiteral("1_000.e-1_000"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(25..37, TokenKind::FloatLiteral("0_123.E0_123"))
    );
    assert!(lexer.next().is_none());

    // Float with a leading dot
    let mut lexer = Lexer::from(".1 .0 .1_000 .0_123");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..2, TokenKind::FloatLiteral(".1"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(3..5, TokenKind::FloatLiteral(".0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(6..12, TokenKind::FloatLiteral(".1_000"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(13..19, TokenKind::FloatLiteral(".0_123"))
    );
    assert!(lexer.next().is_none());

    // Float with a leading dot and exponent part
    let mut lexer = Lexer::from(".1e1 .0e+0 .1_000e-1_000 .0_123E0_123");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..4, TokenKind::FloatLiteral(".1e1"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(5..10, TokenKind::FloatLiteral(".0e+0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(11..24, TokenKind::FloatLiteral(".1_000e-1_000"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(25..37, TokenKind::FloatLiteral(".0_123E0_123"))
    );
    assert!(lexer.next().is_none());

    // Float with a dot followed by a fraction part
    let mut lexer = Lexer::from("1.1 0.0 1_000.1_000 0_123.0_123");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..3, TokenKind::FloatLiteral("1.1"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..7, TokenKind::FloatLiteral("0.0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(8..19, TokenKind::FloatLiteral("1_000.1_000"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(20..31, TokenKind::FloatLiteral("0_123.0_123"))
    );
    assert!(lexer.next().is_none());

    // Float with a dot followed by a fraction part and exponent part
    let mut lexer = Lexer::from("1.1e1 0.0e+0 1_000.1_000e-1_000 0_123.0_123E0_123");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..5, TokenKind::FloatLiteral("1.1e1"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(6..12, TokenKind::FloatLiteral("0.0e+0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(13..31, TokenKind::FloatLiteral("1_000.1_000e-1_000"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(32..49, TokenKind::FloatLiteral("0_123.0_123E0_123"))
    );
    assert!(lexer.next().is_none());

    // Float without fraction part followed by exponent part
    let mut lexer = Lexer::from("1e1 0e+0 1_000e-1_000 0_123E0_123");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..3, TokenKind::FloatLiteral("1e1"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..8, TokenKind::FloatLiteral("0e+0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(9..21, TokenKind::FloatLiteral("1_000e-1_000"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(22..33, TokenKind::FloatLiteral("0_123E0_123"))
    );
    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_string() {
    // Single-quoted string
    let mut lexer = Lexer::from("'Hello, World!'");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..15, TokenKind::StringLiteral("Hello, World!"))
    );

    // Double-quoted string
    let mut lexer = Lexer::from(r#""Hello, World!""#);

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..15, TokenKind::StringLiteral("Hello, World!"))
    );
}

#[test]
fn test_lexer_regex() {
    // Regex
    let mut lexer = Lexer::from(r"//[a-zA-Z_][a-zA-Z0-9_]*//");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..26, TokenKind::RegexLiteral(r"[a-zA-Z_][a-zA-Z0-9_]*"))
    );
}

#[test]
fn test_lexer_symbol() {
    // Symbol
    let mut lexer = Lexer::from("@hello @World_0 @_world @_0world");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..6, TokenKind::SymbolLiteral("hello"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(7..15, TokenKind::SymbolLiteral("World_0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(16..23, TokenKind::SymbolLiteral("_world"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(24..32, TokenKind::SymbolLiteral("_0world"))
    );
    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_boolean() {
    // Boolean
    let mut lexer = Lexer::from("true false");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..4, TokenKind::BooleanLiteral(true))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(5..10, TokenKind::BooleanLiteral(false))
    );
    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_keyword() {
    // Keyword
    let mut lexer = Lexer::from(
        "type trait import export let in transaction if else for while continue break return match fun",
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..4, TokenKind::KeywordType)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(5..10, TokenKind::KeywordTrait)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(11..17, TokenKind::KeywordImport)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(18..24, TokenKind::KeywordExport)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(25..28, TokenKind::KeywordLet)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(29..31, TokenKind::KeywordIn)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(32..43, TokenKind::KeywordTransaction)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(44..46, TokenKind::KeywordIf)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(47..51, TokenKind::KeywordElse)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(52..55, TokenKind::KeywordFor)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(56..61, TokenKind::KeywordWhile)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(62..70, TokenKind::KeywordContinue)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(71..76, TokenKind::KeywordBreak)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(77..83, TokenKind::KeywordReturn)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(84..89, TokenKind::KeywordMatch)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(90..93, TokenKind::KeywordFun)
    );
    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_operator() {
    // Operator
    let mut lexer = Lexer::from(
        "+ - * / % ^ . :: -> -!> = += -= *= /= %= ^= && || ! == != < <= > >= & | ~ << >> &= |= <<= >>= .. ..= ... |> , ; : ( ) [ ] { }",
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..1, TokenKind::OpPlus)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(2..3, TokenKind::OpMinus)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..5, TokenKind::OpMul)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(6..7, TokenKind::OpDiv)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(8..9, TokenKind::OpMod)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(10..11, TokenKind::OpPow)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(12..13, TokenKind::OpDot)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(14..16, TokenKind::OpScope)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(17..19, TokenKind::OpArrow)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(20..23, TokenKind::OpRelateNeg)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(24..25, TokenKind::OpAssign)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(26..28, TokenKind::OpAssignAdd)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(29..31, TokenKind::OpAssignSub)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(32..34, TokenKind::OpAssignMul)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(35..37, TokenKind::OpAssignDiv)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(38..40, TokenKind::OpAssignMod)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(41..43, TokenKind::OpAssignPow)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(44..46, TokenKind::OpAnd)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(47..49, TokenKind::OpOr)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(50..51, TokenKind::OpNot)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(52..54, TokenKind::OpEq)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(55..57, TokenKind::OpNe)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(58..59, TokenKind::OpLt)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(60..62, TokenKind::OpLe)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(63..64, TokenKind::OpGt)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(65..67, TokenKind::OpGe)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(68..69, TokenKind::OpBitAnd)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(70..71, TokenKind::OpBitOr)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(72..73, TokenKind::OpBitNot)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(74..76, TokenKind::OpBitShl)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(77..79, TokenKind::OpBitShr)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(80..82, TokenKind::OpAssignBitAnd)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(83..85, TokenKind::OpAssignBitOr)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(86..89, TokenKind::OpAssignBitShl)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(90..93, TokenKind::OpAssignBitShr)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(94..96, TokenKind::OpRange)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(97..100, TokenKind::OpRangeInclusive)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(101..104, TokenKind::OpEllipsis)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(105..107, TokenKind::OpPipe)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(108..109, TokenKind::OpComma)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(110..111, TokenKind::OpSemicolon)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(112..113, TokenKind::OpColon)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(114..115, TokenKind::OpLParen)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(116..117, TokenKind::OpRParen)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(118..119, TokenKind::OpLBracket)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(120..121, TokenKind::OpRBracket)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(122..123, TokenKind::OpLBrace)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(124..125, TokenKind::OpRBrace)
    );
    assert!(lexer.next().is_none());
}
