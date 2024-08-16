use crate::lexer::{Lexer, RegexFlags, Token, TokenKind};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_lexer_whitespace() {
    // Whitespace
    let mut lexer = Lexer::from(" \t \t");

    assert!(lexer.next().is_none());

    // Comment
    let mut lexer = Lexer::from(" -- This is a comment\nNext");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(21..22, TokenKind::Terminator)
    );

    // Backslash continuation
    let mut lexer = Lexer::from("SlashContinuation \\\n\t\n \tNext");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..17, TokenKind::PlainIdentifier("SlashContinuation"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(24..28, TokenKind::PlainIdentifier("Next"))
    );

    assert!(lexer.next().is_none());

    // Comma continuation
    let mut lexer = Lexer::from("CommaContinuation,\n\t\n \tNext");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..17, TokenKind::PlainIdentifier("CommaContinuation"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(17..18, TokenKind::OpComma)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(23..27, TokenKind::PlainIdentifier("Next"))
    );

    assert!(!lexer.state.continuation_precedent);

    assert!(lexer.next().is_none());

    // Assign continuation
    let mut lexer = Lexer::from("AssignContinuation= \r\n\t\n \tNext");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..18, TokenKind::PlainIdentifier("AssignContinuation"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(18..19, TokenKind::OpIsLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(26..30, TokenKind::PlainIdentifier("Next"))
    );

    assert!(!lexer.state.continuation_precedent);

    assert!(lexer.next().is_none());

    // Bracket continuation
    let mut lexer = Lexer::from("BracketContinuation[()\n\t\n \tNext]\r\nNext");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..19, TokenKind::PlainIdentifier("BracketContinuation"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(19..20, TokenKind::OpOpenSquareBracket)
    );

    assert_eq!(lexer.state.bracket_stack.len(), 1);

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(20..21, TokenKind::OpOpenParen)
    );

    assert_eq!(lexer.state.bracket_stack.len(), 2);

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(21..22, TokenKind::OpCloseParen)
    );

    assert_eq!(lexer.state.bracket_stack.len(), 1);

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(27..31, TokenKind::PlainIdentifier("Next"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(31..32, TokenKind::OpCloseSquareBracket)
    );

    assert_eq!(lexer.state.bracket_stack.len(), 0);

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(32..34, TokenKind::Terminator)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(34..38, TokenKind::PlainIdentifier("Next"))
    );

    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_terminator() {
    // Semicolon
    let mut lexer = Lexer::from(";\r\n \tNext\r\n");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..5, TokenKind::Terminator)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(5..9, TokenKind::PlainIdentifier("Next"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(9..11, TokenKind::Terminator)
    );

    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_identifier() {
    // Plain Identifier
    let mut lexer = Lexer::from("hello World_0 _world _0world");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..5, TokenKind::PlainIdentifier("hello"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(6..13, TokenKind::PlainIdentifier("World_0"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(14..20, TokenKind::PlainIdentifier("_world"))
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(21..28, TokenKind::PlainIdentifier("_0world"))
    );
    assert!(lexer.next().is_none());

    // Escaped Identifier
    let mut lexer = Lexer::from(r#"`hello` `World_0` `_world` `_0world`"#);

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..7, TokenKind::EscapedIdentifier("hello"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(8..17, TokenKind::EscapedIdentifier("World_0"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(18..26, TokenKind::EscapedIdentifier("_world"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(27..36, TokenKind::EscapedIdentifier("_0world"))
    );

    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_variable() {
    // Variable
    let mut lexer = Lexer::from("$hello $World_0 $_world $_0world");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..6, TokenKind::Variable("hello"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(7..15, TokenKind::Variable("World_0"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(16..23, TokenKind::Variable("_world"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(24..32, TokenKind::Variable("_0world"))
    );

    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_bin_integer() {
    // Binary integer
    let mut lexer = Lexer::from("0b0 0b1 0b10 0b11  0b1_000  0b0_111");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..3, TokenKind::BinIntegerLiteral("0"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..7, TokenKind::BinIntegerLiteral("1"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(8..12, TokenKind::BinIntegerLiteral("10"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(13..17, TokenKind::BinIntegerLiteral("11"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(19..26, TokenKind::BinIntegerLiteral("1_000"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(28..35, TokenKind::BinIntegerLiteral("0_111"))
    );

    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_oct_integer() {
    // Octal integer
    let mut lexer = Lexer::from("0o0 0o01234567 \t0o01_23_45_67");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..3, TokenKind::OctIntegerLiteral("0"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..14, TokenKind::OctIntegerLiteral("01234567"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(16..29, TokenKind::OctIntegerLiteral("01_23_45_67"))
    );

    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_hex_integer() {
    // Hexadecimal integer
    let mut lexer = Lexer::from("0x0\t0x0123456789abcdef\n  0x01_23_45_67_89_ab_cd_ef");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..3, TokenKind::HexIntegerLiteral("0"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..22, TokenKind::HexIntegerLiteral("0123456789abcdef"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(22..25, TokenKind::Terminator)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(
            25..50,
            TokenKind::HexIntegerLiteral("01_23_45_67_89_ab_cd_ef")
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
    let mut lexer = Lexer::from("1. 0. 1_000. 0_123.,");

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

    // Digits with multiple final dot or dot followed by a letter SHOULD NOT be matched as a float literal
    let mut lexer = Lexer::from("2.. 2.max");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..1, TokenKind::DecIntegerLiteral("2"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(1..3, TokenKind::OpRange)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..5, TokenKind::DecIntegerLiteral("2"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(5..6, TokenKind::OpDot)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(6..9, TokenKind::PlainIdentifier("max"))
    );

    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_string() {
    // Single-quoted string
    let mut lexer = Lexer::from("'Hello, World!' ''");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..15, TokenKind::StringLiteral("Hello, World!"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(16..18, TokenKind::StringLiteral(""))
    );

    assert!(lexer.next().is_none());

    // Double-quoted string
    let mut lexer = Lexer::from(r#""Hello, World!" """#);

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..15, TokenKind::StringLiteral("Hello, World!"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(16..18, TokenKind::StringLiteral(""))
    );

    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_byte_string() {
    // Single-quoted byte string
    let mut lexer = Lexer::from("b'Hello, World!'");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..16, TokenKind::ByteStringLiteral("Hello, World!"))
    );

    assert!(lexer.next().is_none());

    // Double-quoted byte string
    let mut lexer = Lexer::from(r#"b"Hello, World!""#);

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..16, TokenKind::ByteStringLiteral("Hello, World!"))
    );

    assert!(lexer.next().is_none());
}

#[test]
fn test_lexer_operator() {
    // Brackets
    let mut lexer = Lexer::from(r"( ) [ ] { }");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..1, TokenKind::OpOpenParen)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(2..3, TokenKind::OpCloseParen)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..5, TokenKind::OpOpenSquareBracket)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(6..7, TokenKind::OpCloseSquareBracket)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(8..9, TokenKind::OpOpenBrace)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(10..11, TokenKind::OpCloseBrace)
    );

    // Separators
    let mut lexer = Lexer::from(r", :: :");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..1, TokenKind::OpComma)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(2..4, TokenKind::OpScope)
    );
    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(5..6, TokenKind::OpColon)
    );

    // Assignment
    let mut lexer = Lexer::from(r"+= -= *= ×= /= ÷= %= **= <<= >>= &= |= ^= ~=");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..2, TokenKind::OpAssignPlus)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(3..5, TokenKind::OpAssignMinus)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(6..8, TokenKind::OpAssignMul)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(9..12, TokenKind::OpAssignMul)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(13..15, TokenKind::OpAssignDiv)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(16..19, TokenKind::OpAssignDiv)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(20..22, TokenKind::OpAssignMod)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(23..26, TokenKind::OpAssignPow)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(27..30, TokenKind::OpAssignShl)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(31..34, TokenKind::OpAssignShr)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(35..37, TokenKind::OpAssignBitAnd)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(38..40, TokenKind::OpAssignBitOr)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(41..43, TokenKind::OpAssignBitXor)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(44..46, TokenKind::OpAssignBitNot)
    );

    // Arrows
    let mut lexer = Lexer::from(r"->> <<- -> <-");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..3, TokenKind::OpMultiArrowRight)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..7, TokenKind::OpMultiArrowLeft)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(8..10, TokenKind::OpArrowRight)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(11..13, TokenKind::OpArrowLeft)
    );

    // Arithmetic
    let mut lexer = Lexer::from(r"** + - × / ÷ %");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..2, TokenKind::OpPow)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(3..4, TokenKind::OpPlus)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(5..6, TokenKind::OpMinus)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(7..9, TokenKind::OpMulLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(10..11, TokenKind::OpDiv)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(12..14, TokenKind::OpDiv)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(15..16, TokenKind::OpMod)
    );

    // Conditionals
    let mut lexer = Lexer::from(r"~ !~ <> && || == = != ! <= >= < > ∋ ∌ ⊅ ⊇ ⊃ ?. ?:");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..1, TokenKind::OpMatchLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(2..4, TokenKind::OpNotMatchLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(5..7, TokenKind::OpSimilarity)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(8..10, TokenKind::OpAndLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(11..13, TokenKind::OpOrLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(14..16, TokenKind::OpEq)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(17..18, TokenKind::OpIsLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(19..21, TokenKind::OpIsNotLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(22..23, TokenKind::OpNotLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(24..26, TokenKind::OpLte)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(27..29, TokenKind::OpGte)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(30..31, TokenKind::OpLt)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(32..33, TokenKind::OpGt)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(34..37, TokenKind::OpContainsLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(38..41, TokenKind::OpNotContainsLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(42..45, TokenKind::OpContainsNoneLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(46..49, TokenKind::OpContainsAllLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(50..53, TokenKind::OpContainsAnyLexer)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(54..56, TokenKind::OpSafeNav)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(57..59, TokenKind::OpNullCoalesce)
    );

    assert!(lexer.next().is_none());

    // Bitwise
    let mut lexer = Lexer::from(r"<< >> & | ^");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..2, TokenKind::OpShl)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(3..5, TokenKind::OpShr)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(6..7, TokenKind::OpBitAnd)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(8..9, TokenKind::OpBitOr)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(10..11, TokenKind::OpBitXor)
    );

    // Range
    let mut lexer = Lexer::from(r"..= ..");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..3, TokenKind::OpRangeIncl)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(4..6, TokenKind::OpRange)
    );

    // Dot & Star
    let mut lexer = Lexer::from(r"* .");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..1, TokenKind::OpStar)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(2..3, TokenKind::OpDot)
    );

    // Optional
    let mut lexer = Lexer::from(r"?");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..1, TokenKind::OpOptional)
    );
}

#[test]
fn test_lexer_regex() {
    // Regex
    let mut lexer =
        Lexer::from(r"//[a-zA-Z_][a-zA-Z0-9_]*/// //.+//gimsux //[a-zA-Z_][a-zA-Z0-9_]*//xmig");

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(
            0..26,
            TokenKind::RegexLiteral(r"[a-zA-Z_][a-zA-Z0-9_]*", RegexFlags::empty())
        )
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(26..27, TokenKind::OpDiv)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(
            28..40,
            TokenKind::RegexLiteral(
                r".+",
                RegexFlags::G_GLOBAL
                    | RegexFlags::I_IGNORE_CASE
                    | RegexFlags::M_MULTILINE
                    | RegexFlags::S_SINGLELINE
                    | RegexFlags::U_UNICODE
                    | RegexFlags::X_EXTENDED
            )
        )
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(
            41..71,
            TokenKind::RegexLiteral(
                r"[a-zA-Z_][a-zA-Z0-9_]*",
                RegexFlags::G_GLOBAL
                    | RegexFlags::I_IGNORE_CASE
                    | RegexFlags::M_MULTILINE
                    | RegexFlags::X_EXTENDED
            )
        )
    );
}

#[test]
fn test_lexer_module_block() {
    let mut lexer = Lexer::from(
        r#"
    DEFINE MODULE \
    utilities WITH
        export function identity(value: any): any {
            return value;
        }
    END"#,
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(0..5, TokenKind::Terminator)
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(5..11, TokenKind::PlainIdentifier("DEFINE"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(12..18, TokenKind::PlainIdentifier("MODULE"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(25..34, TokenKind::PlainIdentifier("utilities"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(35..39, TokenKind::PlainIdentifier("WITH"))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(39..132, TokenKind::ModuleBlock("\n        export function identity(value: any): any {\n            return value;\n        }\n    "))
    );

    assert_eq!(
        lexer.next().unwrap().unwrap(),
        Token::new(132..135, TokenKind::PlainIdentifier("END"))
    );

    assert!(lexer.next().is_none());
}
