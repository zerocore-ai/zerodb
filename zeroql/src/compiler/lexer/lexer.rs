use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    lexer::{LexerError, LexerResult, Token, TokenKind},
    Span,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A lexer for the `zeroql` language.
///
/// It is based on the grammar defined in the `./lexer.grammar` file.
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    /// The input string.
    pub(crate) string: &'a str,

    /// The current position in the input string.
    pub(crate) cursor: usize,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Returns the current position in the input string.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Produces the next token in the input string.
    fn next_token(&mut self) -> LexerResult<Option<Token<'a>>> {
        // Check for the end of the input string.
        if self.cursor >= self.string.len() {
            return Ok(None);
        }

        // Skip whitespace and comments.
        let mut remainder = &self.string[self.cursor..];
        if let Some(m) = WHITESPACE_COMMENT_REGEX.find(remainder) {
            self.cursor += m.end();
            remainder = &self.string[self.cursor..];
        }

        // Check for the end of the input string.
        if self.cursor >= self.string.len() {
            return Ok(None);
        }

        // Check for a token.
        let token = if let Some(m) = IDENTIFIER_REGEX.find(remainder) {
            let span = self.advance_by_match(m);
            let token = m.as_str();
            match token {
                "true" => Token::new(span, TokenKind::BooleanLiteral(true)),
                "false" => Token::new(span, TokenKind::BooleanLiteral(false)),
                "type" => Token::new(span, TokenKind::KeywordType),
                "trait" => Token::new(span, TokenKind::KeywordTrait),
                "import" => Token::new(span, TokenKind::KeywordImport),
                "export" => Token::new(span, TokenKind::KeywordExport),
                "let" => Token::new(span, TokenKind::KeywordLet),
                "in" => Token::new(span, TokenKind::KeywordIn),
                "transaction" => Token::new(span, TokenKind::KeywordTransaction),
                "if" => Token::new(span, TokenKind::KeywordIf),
                "else" => Token::new(span, TokenKind::KeywordElse),
                "for" => Token::new(span, TokenKind::KeywordFor),
                "while" => Token::new(span, TokenKind::KeywordWhile),
                "continue" => Token::new(span, TokenKind::KeywordContinue),
                "break" => Token::new(span, TokenKind::KeywordBreak),
                "return" => Token::new(span, TokenKind::KeywordReturn),
                "match" => Token::new(span, TokenKind::KeywordMatch),
                "fun" => Token::new(span, TokenKind::KeywordFun),
                _ => Token::new(span, TokenKind::Identifier(token)),
            }
        } else if let Some(m) = BIN_INTEGER_LITERAL_REGEX.find(remainder) {
            Token::new(
                self.advance_by_match(m),
                TokenKind::BinIntegerLiteral(m.as_str()),
            )
        } else if let Some(m) = OCT_INTEGER_LITERAL_REGEX.find(remainder) {
            Token::new(
                self.advance_by_match(m),
                TokenKind::OctIntegerLiteral(m.as_str()),
            )
        } else if let Some(m) = HEX_INTEGER_LITERAL_REGEX.find(remainder) {
            Token::new(
                self.advance_by_match(m),
                TokenKind::HexIntegerLiteral(m.as_str()),
            )
        } else if let Some(m) = FLOAT_LITERAL_REGEX.find(remainder) {
            Token::new(
                self.advance_by_match(m),
                TokenKind::FloatLiteral(m.as_str()),
            )
        } else if let Some(m) = DEC_INTEGER_LITERAL_REGEX.find(remainder) {
            Token::new(
                self.advance_by_match(m),
                TokenKind::DecIntegerLiteral(m.as_str()),
            )
        } else if let Some(m) = STRING_LITERAL_REGEX.find(remainder) {
            // Remove first and last character (quote marks).
            let trimmed_str = &m.as_str()[1..m.as_str().len() - 1];

            Token::new(
                self.advance_by_match(m),
                TokenKind::StringLiteral(trimmed_str),
            )
        } else if let Some(m) = REGEX_LITERAL_REGEX.find(remainder) {
            // Remove first two and last two characters (slashes).
            let trimmed_str = &m.as_str()[2..m.as_str().len() - 2];

            Token::new(
                self.advance_by_match(m),
                TokenKind::RegexLiteral(trimmed_str),
            )
        } else {
            // TODO: Need to consider moving this to the top, since operators are more common than literals.
            match remainder.chars().next() {
                // === Three Character Tokens ===
                Some('-') if remainder.starts_with("-!>") => {
                    self.cursor += 3;
                    Token::new(self.cursor - 3..self.cursor, TokenKind::OpRelateNeg)
                }
                Some('<') if remainder.starts_with("<<=") => {
                    self.cursor += 3;
                    Token::new(self.cursor - 3..self.cursor, TokenKind::OpAssignBitShl)
                }
                Some('>') if remainder.starts_with(">>=") => {
                    self.cursor += 3;
                    Token::new(self.cursor - 3..self.cursor, TokenKind::OpAssignBitShr)
                }
                Some('.') if remainder.starts_with("..=") => {
                    self.cursor += 3;
                    Token::new(self.cursor - 3..self.cursor, TokenKind::OpRangeInclusive)
                }
                // === Two Character Tokens ===
                Some(':') if remainder.starts_with("::") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpScope)
                }
                Some('-') if remainder.starts_with("->") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpArrow)
                }
                Some('+') if remainder.starts_with("+=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpAssignAdd)
                }
                Some('-') if remainder.starts_with("-=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpAssignSub)
                }
                Some('*') if remainder.starts_with("*=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpAssignMul)
                }
                Some('/') if remainder.starts_with("/=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpAssignDiv)
                }
                Some('%') if remainder.starts_with("%=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpAssignMod)
                }
                Some('^') if remainder.starts_with("^=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpAssignPow)
                }
                Some('&') if remainder.starts_with("&&") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpAnd)
                }
                Some('|') if remainder.starts_with("||") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpOr)
                }
                Some('=') if remainder.starts_with("==") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpEq)
                }
                Some('!') if remainder.starts_with("!=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpNe)
                }
                Some('<') if remainder.starts_with("<=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpLe)
                }
                Some('>') if remainder.starts_with(">=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpGe)
                }
                Some('<') if remainder.starts_with("<<") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpBitShl)
                }
                Some('>') if remainder.starts_with(">>") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpBitShr)
                }
                Some('&') if remainder.starts_with("&=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpAssignBitAnd)
                }
                Some('|') if remainder.starts_with("|=") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpAssignBitOr)
                }
                Some('.') if remainder.starts_with("..") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpRange)
                }
                Some('|') if remainder.starts_with("|>") => {
                    self.cursor += 2;
                    Token::new(self.cursor - 2..self.cursor, TokenKind::OpPipe)
                }
                // === One Character Tokens ===
                Some('+') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpPlus)
                }
                Some('-') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpMinus)
                }
                Some('*') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpMul)
                }
                Some('/') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpDiv)
                }
                Some('%') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpMod)
                }
                Some('^') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpPow)
                }
                Some('.') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpDot)
                }
                Some('=') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpAssign)
                }
                Some('!') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpNot)
                }
                Some('<') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpLt)
                }
                Some('>') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpGt)
                }
                Some('&') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpBitAnd)
                }
                Some('|') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpBitOr)
                }
                Some('~') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpBitNot)
                }
                Some(',') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpComma)
                }
                Some(':') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpColon)
                }
                Some(';') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpSemicolon)
                }
                Some('(') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpLParen)
                }
                Some(')') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpRParen)
                }
                Some('[') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpLBracket)
                }
                Some(']') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpRBracket)
                }
                Some('{') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpLBrace)
                }
                Some('}') => {
                    self.cursor += 1;
                    Token::new(self.cursor - 1..self.cursor, TokenKind::OpRBrace)
                }
                _ => {
                    return Err(LexerError::UnexpectedCharacter {
                        span: self.cursor..self.cursor + 1,
                        character: remainder.chars().next().unwrap(),
                    });
                }
            }
        };

        Ok(Some(token))
    }

    #[inline]
    fn advance_by_match(&mut self, m: regex::Match) -> Span {
        let span = self.cursor..self.cursor + m.end();
        self.cursor += m.end();
        span
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(string: &'a str) -> Self {
        Self { string, cursor: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

lazy_static! {
    static ref WHITESPACE_COMMENT_REGEX: Regex = Regex::new(r"(^(\s+|^//.*\n))").unwrap();
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^([_a-zA-Z][_a-zA-Z0-9]*)").unwrap();
    static ref BIN_INTEGER_LITERAL_REGEX: Regex = Regex::new(r"^(0b[01]+(_?[01])*)").unwrap();
    static ref OCT_INTEGER_LITERAL_REGEX: Regex = Regex::new(r"^(0o[0-7]+(_?[0-7])*)").unwrap();
    static ref HEX_INTEGER_LITERAL_REGEX: Regex = Regex::new(r"^(0x[0-9a-fA-F]+(_?[0-9a-fA-F])*)").unwrap();
    static ref DEC_INTEGER_LITERAL_REGEX: Regex = Regex::new(r"^(\d(_?\d)*)").unwrap();
    static ref FLOAT_LITERAL_REGEX: Regex = Regex::new(r"^(\.\d(_?\d)*([eE][+-]?\d(_?\d)*)?|\d(_?\d)*\.(\d(_?\d)*)?([eE][+-]?\d(_?\d)*)?|\d(_?\d)*([eE][+-]?\d(_?\d)*))").unwrap();
    static ref STRING_LITERAL_REGEX: Regex = Regex::new(r#"^('([^'\\]|\\t|\\n|\\r|\\\\)*'|"([^"\\]|\\t|\\n|\\r|\\\\)*")"#).unwrap();
    static ref REGEX_LITERAL_REGEX: Regex = Regex::new(r#"^(//[^/\n]+//)"#).unwrap();
}
