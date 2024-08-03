use std::fmt::Display;

use lazy_static::lazy_static;
use regex::{Match, Regex, RegexBuilder};

use crate::{
    compiler::reversible::Reversible,
    lexer::{LexerError, LexerResult, Token, TokenKind},
    Span,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A lexer for the `zeroql` language.
///
/// It is based on the grammar defined in the [`./lexer.grammar`](./lexer.grammar) file.
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    /// The input string.
    pub(crate) string: &'a str,

    /// The state of the lexer.
    pub(crate) state: LexerState,
}

/// The state of the lexer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct LexerState {
    /// The current position in the input string.
    pub(crate) cursor: usize,

    /// Bracket stack.
    pub(crate) bracket_stack: Vec<Bracket>,

    /// Whether the last token was a comma or assignment operator.
    pub(crate) continuation_precedent: bool,

    /// To process a module block, we need tokens `kw_define`, `kw_module`, `identifier`, and
    /// `kw_with`, in that order. This field is used to keep track of the number of tokens we have
    /// seen so far. When value reaches 4, then we can start lexing the module block.
    pub(crate) module_block_precedent: u8,
}

/// A bracket.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Bracket {
    /// An opening parenthesis.
    OpenParen,

    /// A closing parenthesis.
    CloseParen,

    /// An opening square bracket.
    OpenSquareBracket,

    /// A closing square bracket.
    CloseSquareBracket,

    /// An opening brace.
    OpenBrace,

    /// A closing brace.
    CloseBrace,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Produces the next token in the input string.
    pub fn next_token(&mut self) -> LexerResult<Option<Token<'a>>> {
        // Check for the end of the input string.
        if self.state.cursor >= self.string.len() {
            return Ok(None);
        }

        // Handle the module block precedent.
        if self.state.module_block_precedent == 4 {
            return Ok(Some(self.lex_module_block()?));
        }

        // Skip continuations, whitespaces and comments.
        self.state.cursor += self.skip_superfluous();
        let remainder = &self.string[self.state.cursor..];

        // Check for the end of the input string.
        if self.state.cursor >= self.string.len() {
            return Ok(None);
        }

        // Check for other tokens.
        let token = if let Some(m) = TERMINATOR_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::Terminator)
        } else if let Some(m) = BYTE_STRING_LITERAL_REGEX.find(remainder) {
            // Remove first two characters (`b` and quote marks).
            let trimmed_str = &m.as_str()[2..m.len() - 1];

            Token::new(
                self.advance_by_match(m),
                TokenKind::ByteStringLiteral(trimmed_str),
            )
        } else if let Some(m) = PLAIN_IDENTIFIER_REGEX.find(remainder) {
            let identifier = m.as_str();

            // Check for the module block precedent.
            match self.state.module_block_precedent {
                // kw_define
                0 => {
                    if identifier == "define" || identifier == "DEFINE" {
                        self.state.module_block_precedent = 1;
                    }
                }
                // kw_module
                1 => {
                    if identifier == "module" || identifier == "MODULE" {
                        self.state.module_block_precedent = 2;
                    }
                }
                // identifier
                2 => {
                    self.state.module_block_precedent = 3;
                }
                // kw_with
                3 => {
                    if identifier == "with" || identifier == "WITH" {
                        self.state.module_block_precedent = 4;
                    }
                }
                _ => unreachable!(),
            }

            Token::new(
                self.advance_by_match(m),
                TokenKind::PlainIdentifier(identifier),
            )
        } else if let Some(m) = ESCAPED_IDENTIFIER_REGEX.find(remainder) {
            // Check for the module block precedent.
            if self.state.module_block_precedent == 2 {
                self.state.module_block_precedent = 3;
            }

            // Remove first and last character (ticks).
            let trimmed_str = &m.as_str()[1..m.len() - 1];

            Token::new(
                self.advance_by_match(m),
                TokenKind::EscapedIdentifier(trimmed_str),
            )
        } else if let Some(m) = VARIABLE_REGEX.find(remainder) {
            // Remove first character (`$`).
            let trimmed_str = &m.as_str()[1..];

            Token::new(self.advance_by_match(m), TokenKind::Variable(trimmed_str))
        } else if let Some(m) = STRING_LITERAL_REGEX.find(remainder) {
            // Remove first and last character (quote marks).
            let trimmed_str = &m.as_str()[1..m.len() - 1];

            Token::new(
                self.advance_by_match(m),
                TokenKind::StringLiteral(trimmed_str),
            )
        } else if let Some(c) = REGEX_LITERAL_REGEX.captures(remainder) {
            // Regex needs to be matched first before operators like "/".

            let captured_regex = c.get(1).unwrap().as_str();
            let captured_flags = c.get(2).map(|m| m.as_str().into()).unwrap_or_default();

            Token::new(
                self.advance_by_match(c.get(0).unwrap()),
                TokenKind::RegexLiteral(captured_regex, captured_flags),
            )
        } else if let Some(m) = OP_OPEN_PAREN_REGEX.find(remainder) {
            self.state.bracket_stack.push(Bracket::OpenParen);

            Token::new(self.advance_by_match(m), TokenKind::OpOpenParen)
        } else if let Some(m) = OP_CLOSE_PAREN_REGEX.find(remainder) {
            match self.state.bracket_stack.pop() {
                Some(Bracket::OpenParen) => (),
                other => {
                    return Err(LexerError::MismatchedBracket {
                        span: self.state.cursor..self.state.cursor + 1,
                        expected: other.map(|b| b.opposite()),
                        found: Bracket::CloseParen,
                    })
                }
            }

            Token::new(self.advance_by_match(m), TokenKind::OpCloseParen)
        } else if let Some(m) = OP_OPEN_SQUARE_BRACKET_REGEX.find(remainder) {
            self.state.bracket_stack.push(Bracket::OpenSquareBracket);

            Token::new(self.advance_by_match(m), TokenKind::OpOpenSquareBracket)
        } else if let Some(m) = OP_CLOSE_SQUARE_BRACKET_REGEX.find(remainder) {
            match self.state.bracket_stack.pop() {
                Some(Bracket::OpenSquareBracket) => (),
                other => {
                    return Err(LexerError::MismatchedBracket {
                        span: self.state.cursor..self.state.cursor + 1,
                        expected: other.map(|b| b.opposite()),
                        found: Bracket::CloseSquareBracket,
                    })
                }
            }

            Token::new(self.advance_by_match(m), TokenKind::OpCloseSquareBracket)
        } else if let Some(m) = OP_OPEN_BRACE_REGEX.find(remainder) {
            self.state.bracket_stack.push(Bracket::OpenBrace);

            Token::new(self.advance_by_match(m), TokenKind::OpOpenBrace)
        } else if let Some(m) = OP_CLOSE_BRACE_REGEX.find(remainder) {
            match self.state.bracket_stack.pop() {
                Some(Bracket::OpenBrace) => (),
                other => {
                    return Err(LexerError::MismatchedBracket {
                        span: self.state.cursor..self.state.cursor + 1,
                        expected: other.map(|b| b.opposite()),
                        found: Bracket::CloseBrace,
                    })
                }
            }

            Token::new(self.advance_by_match(m), TokenKind::OpCloseBrace)
        } else if let Some(m) = OP_COMMA_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpComma)
        } else if let Some(m) = OP_SCOPE_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpScope)
        } else if let Some(m) = OP_COLON_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpColon)
        } else if let Some(m) = OP_ASSIGN_PLUS_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignPlus)
        } else if let Some(m) = OP_ASSIGN_MINUS_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignMinus)
        } else if let Some(m) = OP_ASSIGN_MUL_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignMul)
        } else if let Some(m) = OP_ASSIGN_DIV_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignDiv)
        } else if let Some(m) = OP_ASSIGN_MOD_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignMod)
        } else if let Some(m) = OP_ASSIGN_POW_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignPow)
        } else if let Some(m) = OP_ASSIGN_SHL_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignShl)
        } else if let Some(m) = OP_ASSIGN_SHR_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignShr)
        } else if let Some(m) = OP_ASSIGN_BIT_AND_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignBitAnd)
        } else if let Some(m) = OP_ASSIGN_BIT_OR_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignBitOr)
        } else if let Some(m) = OP_ASSIGN_BIT_XOR_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignBitXor)
        } else if let Some(m) = OP_ASSIGN_BIT_NOT_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignBitNot)
        } else if let Some(m) = OP_ASSIGN_NULL_COALESCE_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpAssignNullCoalesce)
        } else if let Some(m) = OP_MULTI_ARROW_RIGHT_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpMultiArrowRight)
        } else if let Some(m) = OP_MULTI_ARROW_LEFT_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpMultiArrowLeft)
        } else if let Some(m) = OP_ARROW_RIGHT_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpArrowRight)
        } else if let Some(m) = OP_ARROW_LEFT_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpArrowLeft)
        } else if let Some(m) = OP_POW_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpPow)
        } else if let Some(m) = OP_PLUS_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpPlus)
        } else if let Some(m) = OP_MINUS_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpMinus)
        } else if let Some(m) = OP_MUL_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpMulLexer)
        } else if let Some(m) = OP_DIV_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpDiv)
        } else if let Some(m) = OP_MOD_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpMod)
        } else if let Some(m) = OP_MATCH_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpMatchLexer)
        } else if let Some(m) = OP_NOT_MATCH_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpNotMatchLexer)
        } else if let Some(m) = OP_SIMILARITY_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpSimilarity)
        } else if let Some(m) = OP_AND_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpAndLexer)
        } else if let Some(m) = OP_OR_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpOrLexer)
        } else if let Some(m) = OP_EQ_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpEq)
        } else if let Some(m) = OP_IS_LEXER_REGEX.find(remainder) {
            self.state.continuation_precedent = true;
            Token::new(self.advance_by_match(m), TokenKind::OpIsLexer)
        } else if let Some(m) = OP_IS_NOT_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpIsNotLexer)
        } else if let Some(m) = OP_NOT_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpNotLexer)
        } else if let Some(m) = OP_LTE_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpLte)
        } else if let Some(m) = OP_GTE_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpGte)
        } else if let Some(m) = OP_SHL_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpShl)
        } else if let Some(m) = OP_SHR_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpShr)
        } else if let Some(m) = OP_LT_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpLt)
        } else if let Some(m) = OP_GT_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpGt)
        } else if let Some(m) = OP_CONTAINS_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpContainsLexer)
        } else if let Some(m) = OP_NOT_CONTAINS_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpNotContainsLexer)
        } else if let Some(m) = OP_CONTAINS_NONE_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpContainsNoneLexer)
        } else if let Some(m) = OP_CONTAINS_ALL_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpContainsAllLexer)
        } else if let Some(m) = OP_CONTAINS_ANY_LEXER_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpContainsAnyLexer)
        } else if let Some(m) = OP_SAFE_NAV_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpSafeNav)
        } else if let Some(m) = OP_NULL_COALESCE_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpNullCoalesce)
        } else if let Some(m) = OP_BIT_AND_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpBitAnd)
        } else if let Some(m) = OP_BIT_OR_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpBitOr)
        } else if let Some(m) = OP_BIT_XOR_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpBitXor)
        } else if let Some(m) = OP_RANGE_INCL_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpRangeIncl)
        } else if let Some(m) = OP_RANGE_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpRange)
        } else if let Some(m) = OP_STAR_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpStar)
        } else if let Some(m) = HEX_INTEGER_LITERAL_REGEX.find(remainder) {
            // Remove marker characters (`0x`).
            let trimmed_str = &m.as_str()[2..];

            Token::new(
                self.advance_by_match(m),
                TokenKind::HexIntegerLiteral(trimmed_str),
            )
        } else if let Some(m) = OCT_INTEGER_LITERAL_REGEX.find(remainder) {
            // Remove marker characters (`0o`).
            let trimmed_str = &m.as_str()[2..];

            Token::new(
                self.advance_by_match(m),
                TokenKind::OctIntegerLiteral(trimmed_str),
            )
        } else if let Some(m) = BIN_INTEGER_LITERAL_REGEX.find(remainder) {
            // Remove marker characters (`0b`).
            let trimmed_str = &m.as_str()[2..];

            Token::new(
                self.advance_by_match(m),
                TokenKind::BinIntegerLiteral(trimmed_str),
            )
        } else if let Some(m) = Self::float_literal_matches(remainder) {
            Token::new(
                self.advance_by_match(m),
                TokenKind::FloatLiteral(m.as_str()),
            )
        } else if let Some(m) = DEC_INTEGER_LITERAL_REGEX.find(remainder) {
            Token::new(
                self.advance_by_match(m),
                TokenKind::DecIntegerLiteral(m.as_str()),
            )
        } else if let Some(m) = OP_DOT_REGEX.find(remainder) {
            Token::new(self.advance_by_match(m), TokenKind::OpDot)
        } else {
            return Err(LexerError::UnexpectedCharacter {
                span: self.state.cursor..self.state.cursor + 1,
                character: self.string.chars().nth(self.state.cursor).unwrap(),
            });
        };

        Ok(Some(token))
    }

    fn float_literal_matches(remainder: &str) -> Option<Match> {
        if FLOAT_SHOULD_NOT_MATCH_REGEX.find(remainder).is_some() {
            return None;
        }

        if let Some(m) = FLOAT_LITERAL_REGEX.find(remainder) {
            return Some(m);
        }

        None
    }

    fn lex_module_block(&mut self) -> LexerResult<Token<'a>> {
        self.state.module_block_precedent = 0;
        let remainder = &self.string[self.state.cursor..];

        if let Some(m) = MODULE_BLOCK_REGEX.find(remainder) {
            // Vomit last three characters (`end` or `END`).
            let trimmed_block = &m.as_str()[..m.len() - 3];
            let span = self.state.cursor..self.state.cursor + (m.len() - 3);
            self.state.cursor += m.end() - 3;

            return Ok(Token::new(span, TokenKind::ModuleBlock(trimmed_block)));
        }

        Err(LexerError::UnableToLexModuleBlock {
            span: self.state.cursor..self.state.cursor + 1,
        })
    }

    /// Skip continuations, whitespaces and comments
    fn skip_superfluous(&mut self) -> usize {
        let remainder = &self.string[self.state.cursor..];

        if let Some(m) = COMMENT_REGEX.find(remainder) {
            return m.len();
        } else if let Some(m) = CONT_BACK_SLASH_REGEX.find(remainder) {
            return m.len();
        } else if let Some(m) = CONT_WHITESPACE_REGEX.find(remainder) {
            // Check for "," or "=" precedent.
            if self.state.continuation_precedent {
                self.state.continuation_precedent = false;
                return m.len();
            }

            // Check for unmatched bracket precedent.
            if !self.state.bracket_stack.is_empty() {
                return m.len();
            }

            // Otherwise, check if horizontal space.
            if let Some(m) = CONT_HORIZONTAL_SPACE_REGEX.find(remainder) {
                return m.len();
            }
        }

        0
    }

    #[inline]
    fn advance_by_match(&mut self, m: regex::Match) -> Span {
        let span = self.state.cursor..self.state.cursor + m.len();
        self.state.cursor += m.len();
        span
    }
}

impl Bracket {
    fn opposite(&self) -> Self {
        match self {
            Self::OpenParen => Self::CloseParen,
            Self::OpenSquareBracket => Self::CloseSquareBracket,
            Self::OpenBrace => Self::CloseBrace,
            Self::CloseParen => Self::OpenParen,
            Self::CloseSquareBracket => Self::OpenSquareBracket,
            Self::CloseBrace => Self::OpenBrace,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(string: &'a str) -> Self {
        Self {
            string,
            state: LexerState::default(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().transpose()
    }
}

impl Display for Bracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OpenParen => "(",
                Self::CloseParen => ")",
                Self::OpenSquareBracket => "[",
                Self::CloseSquareBracket => "]",
                Self::OpenBrace => "{",
                Self::CloseBrace => "}",
            }
        )
    }
}

impl<'a> Reversible for Lexer<'a> {
    type State = LexerState;

    fn get_state(&self) -> LexerState {
        self.state.clone()
    }

    fn set_state(&mut self, state: LexerState) {
        self.state = state;
    }
}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

lazy_static! {
    static ref COMMENT_REGEX: Regex = Regex::new(r"^\s*--[^\r\n]*").unwrap();
    static ref CONT_BACK_SLASH_REGEX: Regex = Regex::new(r"^\s*\\\r?\n\s*").unwrap();
    static ref CONT_HORIZONTAL_SPACE_REGEX: Regex = Regex::new(r"^[ \t]+").unwrap();
    static ref CONT_WHITESPACE_REGEX: Regex = Regex::new(r"^\s+").unwrap();

    static ref TERMINATOR_REGEX: Regex = Regex::new(r"^(;\s*|(\r?\n)\s*)").unwrap();
    static ref VARIABLE_REGEX: Regex = Regex::new(r"^\$[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    static ref PLAIN_IDENTIFIER_REGEX: Regex = Regex::new(r"^([_a-zA-Z][_a-zA-Z0-9]*)").unwrap();
    static ref ESCAPED_IDENTIFIER_REGEX: Regex = Regex::new(r"^(`[_a-zA-Z][_a-zA-Z0-9]*`)").unwrap();
    static ref STRING_LITERAL_REGEX: Regex = Regex::new(r#"^('([^'\\]|\\t|\\n|\\r|\\\\)*'|"([^"\\]|\\t|\\n|\\r|\\\\)*")"#).unwrap();
    static ref BYTE_STRING_LITERAL_REGEX: Regex = Regex::new(r#"^b('([^'\\]|\\t|\\n|\\r|\\\\)*'|"([^"\\]|\\t|\\n|\\r|\\\\)*")"#).unwrap();

    static ref OP_OPEN_PAREN_REGEX: Regex = Regex::new(r"^\(").unwrap();
    static ref OP_CLOSE_PAREN_REGEX: Regex = Regex::new(r"^\)").unwrap();
    static ref OP_OPEN_SQUARE_BRACKET_REGEX: Regex = Regex::new(r"^\[").unwrap();
    static ref OP_CLOSE_SQUARE_BRACKET_REGEX: Regex = Regex::new(r"^\]").unwrap();
    static ref OP_OPEN_BRACE_REGEX: Regex = Regex::new(r"^\{").unwrap();
    static ref OP_CLOSE_BRACE_REGEX: Regex = Regex::new(r"^\}").unwrap();

    static ref OP_COMMA_REGEX: Regex = Regex::new(r"^,").unwrap();
    static ref OP_SCOPE_REGEX: Regex = Regex::new(r"^::").unwrap();
    static ref OP_COLON_REGEX: Regex = Regex::new(r"^:").unwrap();

    static ref OP_ASSIGN_PLUS_REGEX: Regex = Regex::new(r"^\+=").unwrap();
    static ref OP_ASSIGN_MINUS_REGEX: Regex = Regex::new(r"^-=").unwrap();
    static ref OP_ASSIGN_MUL_REGEX: Regex = Regex::new(r"^(\*=|×=)").unwrap();
    static ref OP_ASSIGN_DIV_REGEX: Regex = Regex::new(r"^(\/=|÷=)").unwrap();
    static ref OP_ASSIGN_MOD_REGEX: Regex = Regex::new(r"^%=").unwrap();
    static ref OP_ASSIGN_POW_REGEX: Regex = Regex::new(r"^\*\*=").unwrap();
    static ref OP_ASSIGN_SHL_REGEX: Regex = Regex::new(r"^<<=").unwrap();
    static ref OP_ASSIGN_SHR_REGEX: Regex = Regex::new(r"^>>=").unwrap();
    static ref OP_ASSIGN_BIT_AND_REGEX: Regex = Regex::new(r"^&=").unwrap();
    static ref OP_ASSIGN_BIT_OR_REGEX: Regex = Regex::new(r"^\|=").unwrap();
    static ref OP_ASSIGN_BIT_XOR_REGEX: Regex = Regex::new(r"^\^=").unwrap();
    static ref OP_ASSIGN_BIT_NOT_REGEX: Regex = Regex::new(r"^~=").unwrap();
    static ref OP_ASSIGN_NULL_COALESCE_REGEX: Regex = Regex::new(r"^\?\?=").unwrap();

    static ref OP_MULTI_ARROW_RIGHT_REGEX: Regex = Regex::new(r"^->>").unwrap();
    static ref OP_MULTI_ARROW_LEFT_REGEX: Regex = Regex::new(r"^<<-").unwrap();
    static ref OP_ARROW_RIGHT_REGEX: Regex = Regex::new(r"^->").unwrap();
    static ref OP_ARROW_LEFT_REGEX: Regex = Regex::new(r"^<-").unwrap();

    static ref OP_POW_REGEX: Regex = Regex::new(r"^\*\*").unwrap();
    static ref OP_PLUS_REGEX: Regex = Regex::new(r"^\+").unwrap();
    static ref OP_MINUS_REGEX: Regex = Regex::new(r"^-").unwrap();
    static ref OP_MUL_LEXER_REGEX: Regex = Regex::new(r"^×").unwrap();
    static ref OP_DIV_REGEX: Regex = Regex::new(r"^(\/|÷)").unwrap();
    static ref OP_MOD_REGEX: Regex = Regex::new(r"^%").unwrap();

    static ref OP_MATCH_LEXER_REGEX: Regex = Regex::new(r"^~").unwrap();
    static ref OP_NOT_MATCH_LEXER_REGEX: Regex = Regex::new(r"^!~").unwrap();
    static ref OP_SIMILARITY_REGEX: Regex = Regex::new(r"^<>").unwrap();

    static ref OP_AND_LEXER_REGEX: Regex = Regex::new(r"^&&").unwrap();
    static ref OP_OR_LEXER_REGEX: Regex = Regex::new(r"^\|\|").unwrap();
    static ref OP_EQ_REGEX: Regex = Regex::new(r"^==").unwrap();
    static ref OP_IS_LEXER_REGEX: Regex = Regex::new(r"^=").unwrap();
    static ref OP_IS_NOT_LEXER_REGEX: Regex = Regex::new(r"^!=").unwrap();
    static ref OP_NOT_LEXER_REGEX: Regex = Regex::new(r"^!").unwrap();

    static ref OP_LTE_REGEX: Regex = Regex::new(r"^<=").unwrap();
    static ref OP_GTE_REGEX: Regex = Regex::new(r"^>=").unwrap();
    static ref OP_LT_REGEX: Regex = Regex::new(r"^<").unwrap();
    static ref OP_GT_REGEX: Regex = Regex::new(r"^>").unwrap();

    static ref OP_CONTAINS_LEXER_REGEX: Regex = Regex::new(r"^∋").unwrap();
    static ref OP_NOT_CONTAINS_LEXER_REGEX: Regex = Regex::new(r"^∌").unwrap();
    static ref OP_CONTAINS_NONE_LEXER_REGEX: Regex = Regex::new(r"^⊅").unwrap();
    static ref OP_CONTAINS_ALL_LEXER_REGEX: Regex = Regex::new(r"^⊇").unwrap();
    static ref OP_CONTAINS_ANY_LEXER_REGEX: Regex = Regex::new(r"^⊃").unwrap();

    static ref OP_SAFE_NAV_REGEX: Regex = Regex::new(r"^\?\?\.").unwrap();
    static ref OP_NULL_COALESCE_REGEX: Regex = Regex::new(r"^\?\?").unwrap();

    static ref OP_SHL_REGEX: Regex = Regex::new(r"^<<").unwrap();
    static ref OP_SHR_REGEX: Regex = Regex::new(r"^>>").unwrap();
    static ref OP_BIT_AND_REGEX: Regex = Regex::new(r"^&").unwrap();
    static ref OP_BIT_OR_REGEX: Regex = Regex::new(r"^\|").unwrap();
    static ref OP_BIT_XOR_REGEX: Regex = Regex::new(r"^\^").unwrap();

    static ref OP_RANGE_INCL_REGEX: Regex = Regex::new(r"^\.\.=").unwrap();
    static ref OP_RANGE_REGEX: Regex = Regex::new(r"^\.\.").unwrap();

    static ref OP_STAR_REGEX: Regex = Regex::new(r"^\*").unwrap();
    static ref OP_DOT_REGEX: Regex = Regex::new(r"^\.").unwrap();

    static ref DEC_INTEGER_LITERAL_REGEX: Regex = Regex::new(r"^\d(_?\d)*").unwrap();
    static ref FLOAT_LITERAL_REGEX: Regex = Regex::new(r"^(\.\d(_?\d)*([eE][+-]?\d(_?\d)*)?|\d(_?\d)*\.(\d(_?\d)*([eE][+-]?\d(_?\d)*)?)?|\d(_?\d)*([eE][+-]?\d(_?\d)*))").unwrap();
    static ref FLOAT_SHOULD_NOT_MATCH_REGEX: Regex = Regex::new(r"^\d(_?\d)*\.[a-zA-Z_.]").unwrap(); // We don't want to match `2..` or `2.max` as a float literal. That should be a range and dot operation respectively
    static ref HEX_INTEGER_LITERAL_REGEX: Regex = Regex::new(r"^0x[0-9a-fA-F]+(_?[0-9a-fA-F])*").unwrap();
    static ref BIN_INTEGER_LITERAL_REGEX: Regex = Regex::new(r"^0b[01]+(_?[01])*").unwrap();
    static ref OCT_INTEGER_LITERAL_REGEX: Regex = Regex::new(r"^0o[0-7]+(_?[0-7])*").unwrap();
    static ref REGEX_LITERAL_REGEX: Regex = Regex::new(r#"^//([^\r\n]+?)//([gimsux]*)"#).unwrap();

    static ref MODULE_BLOCK_REGEX: Regex = RegexBuilder::new(r"^.+?\b(end|END)\b").dot_matches_new_line(true).build().unwrap();
}
