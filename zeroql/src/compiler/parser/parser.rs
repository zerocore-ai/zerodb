use std::num::NonZeroUsize;

use lru::LruCache;
use zeroql_macros::{anykey::AnyKey, backtrack, memoize};

use crate::{
    ast::{Ast, AstKind},
    compiler::reversible::Reversible,
    lexer::{Lexer, LexerState, Token},
    parse,
    parser::ParserResult,
};

use super::Choice;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A [packrat parser][packrat] for the `zeroql` language.
///
/// This parser employs a recursive descent approach with memoization for subexpression results,
/// enabling it to parse any context-free grammar in linear time. It also utilizes state backtracking
/// to manage ambiguous grammars effectively.
///
/// The grammar rules are defined in the [`./parser.grammar`](./parser.grammar) file.
///
/// ## Important
///
/// Due to its recursive descent nature, this parser is not tail-recursive and may cause stack overflows
/// with large inputs. This limitation is known and there are no immediate plans to address it. To mitigate
/// this risk, it is recommended to run the parser in a separate thread to isolate potential faults and/or
/// use the [stacker][stacker] crate to dynamically increase the stack size when it is needed.
/// when it is needed.
///
/// [packrat]: https://en.wikipedia.org/wiki/Packrat_parser
/// [stacker]: https://docs.rs/stacker/latest/stacker/
pub struct Parser<'a> {
    /// This caches results of parsing subexpressions.
    pub(crate) cache: LruCache<Box<dyn AnyKey>, CacheValue<'a>>,

    /// The lexer that produces tokens from the input stream.
    pub(crate) lexer: Lexer<'a>,
}

/// The value stored in the cache.
type CacheValue<'a> = (ParserResult<Option<Ast<'a>>>, LexerState);

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a> Parser<'a> {
    /// Creates a new parser for the given input.
    pub fn new(input: &'a str, cache_size: usize) -> Self {
        let lexer = Lexer::from(input);
        let cache = LruCache::new(NonZeroUsize::new(cache_size).unwrap());
        Self { cache, lexer }
    }

    /// Eats a token from the lexer.
    pub fn eat_token(&mut self) -> ParserResult<Option<Token<'a>>> {
        Ok(self.lexer.next_token()?)
    }

    /// Parses a program.
    ///
    /// ```txt
    /// program =
    ///     | terminator* (stmt | exp) (terminator+ (stmt | exp))* terminator*
    /// ```
    #[memoize(cache = self.cache, state = self.lexer.state)]
    #[backtrack(state = self.lexer.state, condition = |r| matches!(r, Ok(None)))]
    pub fn parse_program(&mut self) -> ParserResult<Option<Ast<'a>>> {
        let result = parse!(self, Self => (seq
            (many_0 parse_terminator)
            (alt parse_stmt parse_exp)
            (many_0 (seq
                (many_1 parse_terminator)
                (alt parse_stmt parse_exp)
            ))
            (many_0 parse_terminator)
        ));

        let ast = result.map(|x| {
            let (_, stmt_or_exp, rest, _) = x.unwrap_seq4();

            let ast = match stmt_or_exp.unwrap_choice() {
                Choice::A(x) => x.unwrap_single(),
                Choice::B(x) => x.unwrap_single(),
                _ => unreachable!(),
            };

            let span_start = ast.span.start;
            let mut span_end = ast.span.end;

            let mut asts = vec![ast];
            for x in rest.unwrap_many() {
                let (_, stmt_or_exp) = x.unwrap_seq2();
                let ast = match stmt_or_exp.unwrap_choice() {
                    Choice::A(x) => x.unwrap_single(),
                    Choice::B(x) => x.unwrap_single(),
                    _ => unreachable!(),
                };
                span_end = ast.span.end;
                asts.push(ast);
            }

            Ast::new(span_start..span_end, AstKind::Program(asts))
        });

        Ok(ast)
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<'a> Reversible for Parser<'a> {
    type State = LexerState;

    fn get_state(&self) -> Self::State {
        self.lexer.get_state()
    }

    fn set_state(&mut self, state: Self::State) {
        self.lexer.set_state(state);
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use tracing::info;

    use crate::ast::{AstKind::*, Field, TypeSig::*};

    use super::*;

    #[test_log::test]
    fn test_parser_program() -> anyhow::Result<()> {
        let parser = &mut Parser::new(
            r#"\
            LET $age =
            10

            IF $age > 18 THEN
               print("You are an adult")
            ELSE
               print("You are a minor")
            END

            DEFINE TABLE person FIELDS\
                name TYPE string,
                age TYPE u8

            2 + (0x100 * 3)
            "#,
            20,
        );

        let result = parser.parse_program()?;

        info!(
            r#"input = {:?} | parse_program = {:#?}"#,
            parser.lexer.string, result,
        );

        assert_eq!(
            result,
            Some(Ast {
                span: 14..315,
                kind: Program(vec![
                    Ast {
                        span: 14..39,
                        kind: Let {
                            name: Box::new(Ast {
                                span: 18..22,
                                kind: Variable("age"),
                                tag: Default::default(),
                            }),
                            r#type: None,
                            value: Box::new(Ast {
                                span: 37..39,
                                kind: IntegerLiteral(10),
                                tag: Default::default(),
                            }),
                        },
                        tag: Default::default(),
                    },
                    Ast {
                        span: 53..184,
                        kind: If {
                            condition: Box::new(Ast {
                                span: 56..65,
                                kind: GreaterThanOp(
                                    Box::new(Ast {
                                        span: 56..60,
                                        kind: Variable("age"),
                                        tag: Default::default(),
                                    }),
                                    Box::new(Ast {
                                        span: 63..65,
                                        kind: IntegerLiteral(18),
                                        tag: Default::default(),
                                    }),
                                ),
                                tag: Default::default(),
                            }),
                            then: Box::new(Ast {
                                span: 86..111,
                                kind: Program(vec![Ast {
                                    span: 86..111,
                                    kind: FunctionCall {
                                        subject: Box::new(Ast {
                                            span: 86..91,
                                            kind: Identifier("print"),
                                            tag: Default::default(),
                                        }),
                                        args: vec![Ast {
                                            span: 92..110,
                                            kind: FunctionArg {
                                                name: None,
                                                value: Box::new(Ast {
                                                    span: 92..110,
                                                    kind: StringLiteral("You are an adult"),
                                                    tag: Default::default(),
                                                }),
                                            },
                                            tag: Default::default(),
                                        }],
                                    },
                                    tag: Default::default(),
                                }]),
                                tag: Default::default(),
                            }),
                            else_ifs: vec![],
                            r#else: Some(Box::new(Ast {
                                span: 144..168,
                                kind: Program(vec![Ast {
                                    span: 144..168,
                                    kind: FunctionCall {
                                        subject: Box::new(Ast {
                                            span: 144..149,
                                            kind: Identifier("print"),
                                            tag: Default::default(),
                                        }),
                                        args: vec![Ast {
                                            span: 150..167,
                                            kind: FunctionArg {
                                                name: None,
                                                value: Box::new(Ast {
                                                    span: 150..167,
                                                    kind: StringLiteral("You are a minor"),
                                                    tag: Default::default(),
                                                }),
                                            },
                                            tag: Default::default(),
                                        }],
                                    },
                                    tag: Default::default(),
                                }]),
                                tag: Default::default(),
                            })),
                        },
                        tag: Default::default(),
                    },
                    Ast {
                        span: 198..287,
                        kind: DefineTable {
                            name: Box::new(Ast {
                                span: 211..217,
                                kind: Identifier("person"),
                                tag: Default::default(),
                            }),
                            if_not_exists: false,
                            database: None,
                            fields: vec![
                                Field {
                                    name: Box::new(Ast {
                                        span: 242..246,
                                        kind: Identifier("name"),
                                        tag: Default::default(),
                                    }),
                                    r#type: Basic(Box::new(Ast {
                                        span: 252..258,
                                        kind: Identifier("string"),
                                        tag: Default::default(),
                                    })),
                                    default: None,
                                    assertions: vec![],
                                    readonly: false,
                                    unique: false,
                                },
                                Field {
                                    name: Box::new(Ast {
                                        span: 276..279,
                                        kind: Identifier("age"),
                                        tag: Default::default(),
                                    }),
                                    r#type: Basic(Box::new(Ast {
                                        span: 285..287,
                                        kind: Identifier("u8"),
                                        tag: Default::default(),
                                    })),
                                    default: None,
                                    assertions: vec![],
                                    readonly: false,
                                    unique: false,
                                },
                            ],
                        },
                        tag: Default::default(),
                    },
                    Ast {
                        span: 301..315,
                        kind: AdditionOp(
                            Box::new(Ast {
                                span: 301..302,
                                kind: IntegerLiteral(2),
                                tag: Default::default(),
                            }),
                            Box::new(Ast {
                                span: 306..315,
                                kind: MultiplicationOp(
                                    Box::new(Ast {
                                        span: 306..311,
                                        kind: IntegerLiteral(0x100),
                                        tag: Default::default(),
                                    }),
                                    Box::new(Ast {
                                        span: 314..315,
                                        kind: IntegerLiteral(3),
                                        tag: Default::default(),
                                    }),
                                ),
                                tag: Default::default(),
                            }),
                        ),
                        tag: Default::default(),
                    },
                ]),
                tag: Default::default(),
            })
        );

        Ok(())
    }
}
