use crate::parse;

use super::*;
use itertools::Itertools;
use mock::*;
use tracing::info;

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test]
fn test_combinator_perm_optional() -> anyhow::Result<()> {
    for i in ('a'..='d').permutations(4) {
        let parser = &mut Parser::new(i);
        let result = parse!(parser, Parser => (perm_opt parse_a parse_b parse_c parse_d));
        info!(
            "input = {:?} | (perm_opt parse_a parse_b parse_c parse_d) = {:?}",
            parser.input, result
        );
        assert_eq!(
            result,
            Some(Combinator::Seq4(
                Box::new(Combinator::Single(Ast::A)),
                Box::new(Combinator::Single(Ast::B)),
                Box::new(Combinator::Single(Ast::C)),
                Box::new(Combinator::Single(Ast::D))
            ))
        );
    }

    // Optional rules
    let parser = &mut Parser::new(['b']);
    let result = parse!(parser, Parser => (perm_opt (opt parse_c) parse_b (opt parse_a)));
    info!(
        "input = {:?} | (perm_opt (opt parse_c) parse_b (opt parse_a)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq3(
            Box::new(Combinator::Void),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Void)
        ))
    );

    let parser = &mut Parser::new(['b', 'c']);
    let result = parse!(parser, Parser => (perm_opt (opt parse_c) parse_b (opt parse_a)));
    info!(
        "input = {:?} | (perm_opt (opt parse_c) parse_b (opt parse_a)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq3(
            Box::new(Combinator::Single(Ast::C)),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Void)
        ))
    );

    let parser = &mut Parser::new(['a', 'b', 'c']);
    let result = parse!(parser, Parser => (perm_opt (opt parse_c) parse_b (opt parse_a)));
    info!(
        "input = {:?} | (perm_opt (opt parse_c) parse_b (opt parse_a)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq3(
            Box::new(Combinator::Single(Ast::C)),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Single(Ast::A))
        ))
    );

    // Fail Cases
    let parser = &mut Parser::new(['c', 'a']);
    let result = parse!(parser, Parser => (perm_opt (opt parse_c) parse_b (opt parse_a)));
    info!(
        "input = {:?} | (perm_opt (opt parse_c) parse_b (opt parse_a)) = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    let parser = &mut Parser::new(['a', 'b']);
    let result = parse!(parser, Parser => (perm_opt parse_a parse_c));
    info!(
        "input = {:?} | (perm_opt parse_a parse_c) = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    Ok(())
}

#[test_log::test]
fn test_combinator_perm() -> anyhow::Result<()> {
    let parser = &mut Parser::new(['a', 'b']);
    let result = parse!(parser, Parser => (perm parse_a parse_b));
    info!(
        "input = {:?} | (perm parse_a parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq2(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B))
        ))
    );

    let parser = &mut Parser::new(['b', 'a']);
    let result = parse!(parser, Parser => (perm parse_a parse_b));
    info!(
        "input = {:?} | (perm parse_a parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq2(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B))
        ))
    );

    for i in ('a'..='c').permutations(3) {
        let parser = &mut Parser::new(i);
        let result = parse!(parser, Parser => (perm parse_a parse_b parse_c));
        info!(
            "input = {:?} | (perm parse_a parse_b parse_c) = {:?}",
            parser.input, result
        );
        assert_eq!(
            result,
            Some(Combinator::Seq3(
                Box::new(Combinator::Single(Ast::A)),
                Box::new(Combinator::Single(Ast::B)),
                Box::new(Combinator::Single(Ast::C))
            ))
        );
    }

    for i in ('a'..='d').permutations(4) {
        let parser = &mut Parser::new(i);
        let result = parse!(parser, Parser => (perm parse_a parse_b parse_c parse_d));
        info!(
            "input = {:?} | (perm parse_a parse_b parse_c parse_d) = {:?}",
            parser.input, result
        );
        assert_eq!(
            result,
            Some(Combinator::Seq4(
                Box::new(Combinator::Single(Ast::A)),
                Box::new(Combinator::Single(Ast::B)),
                Box::new(Combinator::Single(Ast::C)),
                Box::new(Combinator::Single(Ast::D))
            ))
        );
    }

    // Fail Cases
    let parser = &mut Parser::new(['a', 'b']);
    let result = parse!(parser, Parser => (perm parse_a parse_c));
    info!(
        "input = {:?} | (perm parse_a parse_c) = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    Ok(())
}

#[test_log::test]
fn test_combinator_single() -> anyhow::Result<()> {
    let mut parser = Parser::new(['a']);
    let result = parse!(&mut parser, Parser => parse_a);
    info!("input = {:?} | parse_a = {:?}", parser.input, result);
    assert_eq!(result, Some(Combinator::Single(Ast::A)));

    // Fail Cases
    let mut parser = Parser::new(['b']);
    let result = parse!(&mut parser, Parser => parse_a);
    info!("input = {:?} | parse_a = {:?}", parser.input, result);
    assert_eq!(result, None);

    Ok(())
}

#[test_log::test]
fn test_combinator_arg() -> anyhow::Result<()> {
    let mut parser = Parser::new(['a']);
    let result = parse!(&mut parser, Parser => (arg parse_char 'a'));
    info!(
        "input = {:?} | (arg parse_char 'a') = {:?}",
        parser.input, result
    );
    assert_eq!(result, Some(Combinator::Single(Ast::Some('a'))));

    // Fail Cases
    let mut parser = Parser::new(['b']);
    let result = parse!(&mut parser, Parser => (arg parse_char 'a'));
    info!(
        "input = {:?} | (arg parse_char 'a') = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    Ok(())
}

#[test_log::test]
fn test_combinator_opt() -> anyhow::Result<()> {
    let mut parser = Parser::new(['a']);
    let result = parse!(&mut parser, Parser => (opt parse_a));
    info!("input = {:?} | (opt parse_a) = {:?}", parser.input, result);
    assert_eq!(result, Some(Combinator::Single(Ast::A)));

    // Fail Cases
    let mut parser = Parser::new(['b']);
    let result = parse!(&mut parser, Parser => (opt parse_a));
    info!("input = {:?} | (opt parse_a) = {:?}", parser.input, result);
    assert_eq!(result, Some(Combinator::Void));

    Ok(())
}

#[test_log::test]
fn test_combinator_many() -> anyhow::Result<()> {
    let parser = &mut Parser::new(['b', 'b', 'b']);
    let result = parse!(parser, Parser => (many_0 parse_b));
    info!(
        "input = {:?} | (many_0 parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Many(vec![
            Combinator::Single(Ast::B),
            Combinator::Single(Ast::B),
            Combinator::Single(Ast::B),
        ]))
    );

    let parser = &mut Parser::new(['b', 'b']);
    let result = parse!(parser, Parser => (many_1 parse_b));
    info!(
        "input = {:?} | (many_1 parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Many(vec![
            Combinator::Single(Ast::B),
            Combinator::Single(Ast::B),
        ]))
    );

    // Fail Cases
    let parser = &mut Parser::new([]);
    let result = parse!(parser, Parser => (many_0 parse_b));
    info!(
        "input = {:?} | (many_0 parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(result, Some(Combinator::Many(vec![])));

    let parser = &mut Parser::new([]);
    let result = parse!(parser, Parser => (many_1 parse_b));
    info!(
        "input = {:?} | (many_1 parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    Ok(())
}

#[test_log::test]
fn test_combinator_alt() -> anyhow::Result<()> {
    let parser = &mut Parser::new(['a']);
    let result = parse!(parser, Parser => (alt parse_a parse_b));
    info!(
        "input = {:?} | (alt parse_a parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::A(Box::new(Combinator::Single(
            Ast::A
        )))))
    );

    let parser = &mut Parser::new(['b']);
    let result = parse!(parser, Parser => (alt parse_a parse_b parse_c));
    info!(
        "input = {:?} | (alt parse_a parse_b parse_c) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::B(Box::new(Combinator::Single(
            Ast::B
        )))))
    );

    let parser = &mut Parser::new(['d']);
    let result = parse!(parser, Parser => (alt parse_a parse_b parse_c parse_d));
    info!(
        "input = {:?} | (alt parse_a parse_b parse_c parse_d) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::D(Box::new(Combinator::Single(
            Ast::D
        )))))
    );

    let parser = &mut Parser::new(['a']);
    let result = parse!(parser, Parser => (alt parse_a parse_b parse_c parse_d parse_e));
    info!(
        "input = {:?} | (alt parse_a parse_b parse_c parse_d parse_e) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::A(Box::new(Combinator::Single(
            Ast::A
        )))))
    );

    let parser = &mut Parser::new(['f']);
    let result = parse!(parser, Parser => (alt parse_a parse_b parse_c parse_d parse_e parse_f));
    info!(
        "input = {:?} | (alt parse_a parse_b parse_c parse_d parse_e parse_f) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::F(Box::new(Combinator::Single(
            Ast::F
        )))))
    );

    let parser = &mut Parser::new(['c']);
    let result = parse!(parser, Parser => (alt parse_b parse_c parse_d));
    info!(
        "input = {:?} | (alt parse_b parse_c parse_d) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::B(Box::new(Combinator::Single(
            Ast::C
        )))))
    );

    let parser = &mut Parser::new(['a']);
    let result = parse!(parser, Parser => (alt parse_b parse_c));
    info!(
        "input = {:?} | (alt parse_b parse_c) = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    Ok(())
}

#[test_log::test]
fn test_combinator_seq() -> anyhow::Result<()> {
    let parser = &mut Parser::new(['a', 'b']);
    let result = parse!(parser, Parser => (seq parse_a parse_b));
    info!(
        "input = {:?} | (seq parse_a parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq2(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B))
        ))
    );

    let parser = &mut Parser::new(['a', 'b', 'c']);
    let result = parse!(parser, Parser => (seq parse_a parse_b parse_c));
    info!(
        "input = {:?} | (seq parse_a parse_b parse_c) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq3(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Single(Ast::C))
        ))
    );

    let parser = &mut Parser::new(['a', 'b', 'c', 'd']);
    let result = parse!(parser, Parser => (seq parse_a parse_b parse_c parse_d));
    info!(
        "input = {:?} | (seq parse_a parse_b parse_c parse_d) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq4(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Single(Ast::C)),
            Box::new(Combinator::Single(Ast::D))
        ))
    );

    let parser = &mut Parser::new(['a', 'b', 'c', 'd', 'a']);
    let result = parse!(parser, Parser => (seq parse_a parse_b parse_c parse_d parse_a));
    info!(
        "input = {:?} | (seq parse_a parse_b parse_c parse_d parse_a) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq5(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Single(Ast::C)),
            Box::new(Combinator::Single(Ast::D)),
            Box::new(Combinator::Single(Ast::A))
        ))
    );

    let parser = &mut Parser::new(['a', 'b', 'c', 'd', 'a', 'b']);
    let result = parse!(parser, Parser => (seq parse_a parse_b parse_c parse_d parse_a parse_b));
    info!(
        "input = {:?} | (seq parse_a parse_b parse_c parse_a parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq6(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Single(Ast::C)),
            Box::new(Combinator::Single(Ast::D)),
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B))
        ))
    );

    let parser = &mut Parser::new(['a', 'b', 'c', 'd', 'a', 'b', 'c']);
    let result =
        parse!(parser, Parser => (seq parse_a parse_b parse_c parse_d parse_a parse_b parse_c));
    info!(
        "input = {:?} | (seq parse_a parse_b parse_c parse_d parse_a parse_b parse_c) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq7(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Single(Ast::C)),
            Box::new(Combinator::Single(Ast::D)),
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Single(Ast::C))
        ))
    );

    let parser = &mut Parser::new(['a', 'b', 'c', 'd', 'a', 'b', 'c', 'd']);
    let result = parse!(parser, Parser => (seq parse_a parse_b parse_c parse_d parse_a parse_b parse_c parse_d));
    info!(
            "input = {:?} | (seq parse_a parse_b parse_c parse_d parse_a parse_b parse_c parse_d) = {:?}",
            parser.input, result
        );
    assert_eq!(
        result,
        Some(Combinator::Seq8(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Single(Ast::C)),
            Box::new(Combinator::Single(Ast::D)),
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Single(Ast::C)),
            Box::new(Combinator::Single(Ast::D))
        ))
    );

    // Fail Cases
    let parser = &mut Parser::new(['c', 'b']);
    let result = parse!(parser, Parser => (seq parse_a parse_b));
    info!(
        "input = {:?} | (seq parse_a parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    let parser = &mut Parser::new(['a', 'c']);
    let result = parse!(parser, Parser => (seq parse_a parse_b));
    info!(
        "input = {:?} | (seq parse_a parse_b) = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    let parser = &mut Parser::new(['d', 'b', 'c']);
    let result = parse!(parser, Parser => (seq parse_a parse_b parse_c));
    info!(
        "input = {:?} | (seq parse_a parse_b parse_c) = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    let parser = &mut Parser::new(['a', 'b', 'c', 'a']);
    let result = parse!(parser, Parser => (seq parse_a parse_b parse_c parse_d));
    info!(
        "input = {:?} | (seq parse_a parse_b parse_c parse_d) = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    let parser = &mut Parser::new(['a', 'b', 'c', 'e', 'a', 'b', 'c', 'd']);
    let result = parse!(parser, Parser => (seq parse_a parse_b parse_c parse_d parse_a parse_b parse_c parse_d));
    info!(
            "input = {:?} | (seq parse_a parse_b parse_c parse_d parse_a parse_b parse_c parse_d) = {:?}",
            parser.input, result

        );
    assert_eq!(result, None);

    Ok(())
}

#[test_log::test]
fn test_combinator_mix() -> anyhow::Result<()> {
    // Simple alternative with left choice
    let parser = &mut Parser::new(['a']);
    let result = parse!(parser, Parser => (alt parse_a (seq parse_a parse_b)));
    info!(
        "input = {:?} | (alt parse_a (seq parse_a parse_b)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::A(Box::new(Combinator::Single(
            Ast::A
        )))))
    );

    // Sequence with nested sequence
    let parser = &mut Parser::new(['a', 'b', 'c', 'd']);
    let result = parse!(parser, Parser => (seq (opt parse_a) (seq (arg parse_char 'b') (arg parse_char 'c') parse_d)));
    info!(
            "input = {:?} | (seq (opt parse_a) (seq (arg parse_char 'b') (arg parse_char 'c') parse_d))) = {:?}",
            parser.input, result
        );
    assert_eq!(
        result,
        Some(Combinator::Seq2(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Seq3(
                Box::new(Combinator::Single(Ast::Some('b'))),
                Box::new(Combinator::Single(Ast::Some('c'))),
                Box::new(Combinator::Single(Ast::D)),
            ))
        ))
    );

    // Sequence with alternative
    let parser = &mut Parser::new(['c', 'a']);
    let result = parse!(parser, Parser => (seq parse_c (alt parse_a parse_b)));
    info!(
        "input = {:?} | (seq parse_c (alt parse_a parse_b)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq2(
            Box::new(Combinator::Single(Ast::C)),
            Box::new(Combinator::Choice(Choice::A(Box::new(Combinator::Single(
                Ast::A
            )))))
        ))
    );

    // Fail Cases
    let parser = &mut Parser::new(['b', 'c']);
    let result = parse!(parser, Parser => (alt parse_a (seq parse_b parse_c)));
    info!(
        "input = {:?} | (alt parse_a (seq parse_b parse_c)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::B(Box::new(Combinator::Seq2(
            Box::new(Combinator::Single(Ast::B)),
            Box::new(Combinator::Single(Ast::C))
        )))))
    );

    let parser = &mut Parser::new(['a', 'd']);
    let result = parse!(parser, Parser => (alt (seq parse_a parse_b) parse_a));
    info!(
        "input = {:?} | (alt (seq parse_a parse_b) parse_a) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::B(Box::new(Combinator::Single(
            Ast::A
        )))))
    );

    let parser = &mut Parser::new(['a', 'a', 'c']);
    let result = parse!(parser, Parser => (alt (seq parse_a parse_b parse_c) (alt (seq parse_a parse_b) parse_a)));
    info!("input = {:?} | (alt (seq parse_a parse_b parse_c) (alt (seq parse_a parse_b) parse_a)) = {:?}", parser.input, result);
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::B(Box::new(Combinator::Choice(
            Choice::B(Box::new(Combinator::Single(Ast::A)))
        )))))
    );

    let parser = &mut Parser::new(['b', 'b']);
    let result = parse!(parser, Parser => (alt (seq parse_a parse_b) parse_a));
    info!(
        "input = {:?} | (alt (seq parse_a parse_b) parse_a) = {:?}",
        parser.input, result
    );
    assert_eq!(result, None);

    let parser = &mut Parser::new(['b', 'c']);
    let result = parse!(parser, Parser => (seq (opt parse_a) (seq parse_b parse_c)));
    info!(
        "input = {:?} | (seq (opt parse_a) (seq parse_b parse_c)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq2(
            Box::new(Combinator::Void),
            Box::new(Combinator::Seq2(
                Box::new(Combinator::Single(Ast::B)),
                Box::new(Combinator::Single(Ast::C))
            ))
        ))
    );

    let parser = &mut Parser::new(['a', 'd']);
    let result = parse!(parser, Parser => (seq (opt (seq parse_a parse_b)) (seq parse_a parse_d)));
    info!(
        "input = {:?} | (seq (opt (seq parse_a parse_b)) (seq parse_a parse_d)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq2(
            Box::new(Combinator::Void),
            Box::new(Combinator::Seq2(
                Box::new(Combinator::Single(Ast::A)),
                Box::new(Combinator::Single(Ast::D))
            ))
        ))
    );

    // Permutation in sequence
    let parser = &mut Parser::new(['b', 'a', 'd', 'c']);
    let result = parse!(parser, Parser => (seq (perm parse_a parse_b) (perm parse_c parse_d)));
    info!(
        "input = {:?} | (seq (perm parse_a parse_b) (perm parse_c parse_d)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq2(
            Box::new(Combinator::Seq2(
                Box::new(Combinator::Single(Ast::A)),
                Box::new(Combinator::Single(Ast::B))
            )),
            Box::new(Combinator::Seq2(
                Box::new(Combinator::Single(Ast::C)),
                Box::new(Combinator::Single(Ast::D))
            ))
        ))
    );

    // Sequence in permutation
    let parser = &mut Parser::new(['c', 'd', 'a', 'b']);
    let result = parse!(parser, Parser => (perm (seq parse_a parse_b) (seq parse_c parse_d)));
    info!(
        "input = {:?} | (perm (seq parse_a parse_b) (seq parse_c parse_d)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Seq2(
            Box::new(Combinator::Seq2(
                Box::new(Combinator::Single(Ast::A)),
                Box::new(Combinator::Single(Ast::B))
            )),
            Box::new(Combinator::Seq2(
                Box::new(Combinator::Single(Ast::C)),
                Box::new(Combinator::Single(Ast::D))
            ))
        ))
    );

    // Zero or more sequences
    let parser = &mut Parser::new(['a', 'b', 'a', 'd']);
    let result = parse!(parser, Parser => (many_0 (seq parse_a parse_b)));
    info!(
        "input = {:?} | (many_0 (seq parse_a parse_b)) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Many(vec![Combinator::Seq2(
            Box::new(Combinator::Single(Ast::A)),
            Box::new(Combinator::Single(Ast::B))
        )]))
    );

    // Nested alternative
    let parser = &mut Parser::new(['d']);
    let result = parse!(parser, Parser => (alt
        parse_a
        (alt
            parse_b
            (alt
                parse_c
                parse_d
            )
        )
    ));
    info!(
        "input = {:?} | (alt parse_a (alt parse_b (alt parse_c parse_d))) = {:?}",
        parser.input, result
    );
    assert_eq!(
        result,
        Some(Combinator::Choice(Choice::B(Box::new(Combinator::Choice(
            Choice::B(Box::new(Combinator::Choice(Choice::B(Box::new(
                Combinator::Single(Ast::D)
            )))))
        )))))
    );

    Ok(())
}

#[cfg(test)]
mod mock {
    use crate::compiler::reversible::Reversible;

    //--------------------------------------------------------------------------------------------------
    // Types
    //--------------------------------------------------------------------------------------------------

    pub(super) struct Parser {
        pub(super) input: Vec<char>,
        pub(super) cursor: usize,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum Ast {
        Some(char),
        A,
        B,
        C,
        D,
        E,
        F,
    }

    //--------------------------------------------------------------------------------------------------
    // Methods
    //--------------------------------------------------------------------------------------------------

    impl Parser {
        pub(super) fn new(input: impl Into<Vec<char>>) -> Self {
            Self {
                input: input.into(),
                cursor: 0,
            }
        }

        pub(super) fn parse_char(&mut self, c: char) -> anyhow::Result<Option<Ast>> {
            if let Some(ch) = self.input.get(self.cursor) {
                if ch == &c {
                    self.cursor += 1;
                    return Ok(Some(Ast::Some(c)));
                }
            }

            Ok(None)
        }

        pub(super) fn parse_a(&mut self) -> anyhow::Result<Option<Ast>> {
            match self.input.get(self.cursor) {
                Some('a') => {
                    self.cursor += 1;
                    Ok(Some(Ast::A))
                }
                _ => Ok(None),
            }
        }

        pub(super) fn parse_b(&mut self) -> anyhow::Result<Option<Ast>> {
            match self.input.get(self.cursor) {
                Some('b') => {
                    self.cursor += 1;
                    Ok(Some(Ast::B))
                }
                _ => Ok(None),
            }
        }

        pub(super) fn parse_c(&mut self) -> anyhow::Result<Option<Ast>> {
            match self.input.get(self.cursor) {
                Some('c') => {
                    self.cursor += 1;
                    Ok(Some(Ast::C))
                }
                _ => Ok(None),
            }
        }

        pub(super) fn parse_d(&mut self) -> anyhow::Result<Option<Ast>> {
            match self.input.get(self.cursor) {
                Some('d') => {
                    self.cursor += 1;
                    Ok(Some(Ast::D))
                }
                _ => Ok(None),
            }
        }

        pub(super) fn parse_e(&mut self) -> anyhow::Result<Option<Ast>> {
            match self.input.get(self.cursor) {
                Some('e') => {
                    self.cursor += 1;
                    Ok(Some(Ast::E))
                }
                _ => Ok(None),
            }
        }

        pub(super) fn parse_f(&mut self) -> anyhow::Result<Option<Ast>> {
            match self.input.get(self.cursor) {
                Some('f') => {
                    self.cursor += 1;
                    Ok(Some(Ast::F))
                }
                _ => Ok(None),
            }
        }
    }

    //--------------------------------------------------------------------------------------------------
    // Trait Implementations
    //--------------------------------------------------------------------------------------------------

    impl Reversible for Parser {
        type State = usize;

        fn get_state(&self) -> Self::State {
            self.cursor
        }

        fn set_state(&mut self, state: Self::State) {
            self.cursor = state;
        }
    }
}
