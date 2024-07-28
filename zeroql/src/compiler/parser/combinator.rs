use itertools::Either;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a combinator expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Combinator<T> {
    /// A single `T` value.
    Single(T),

    /// A choice between two combinators.
    Choice(Either<Box<Combinator<T>>, Box<Combinator<T>>>),

    /// A vector of repeated `T` values.
    Many(Vec<Combinator<T>>),

    /// A sequence of two `T` values.
    Seq2(Box<Combinator<T>>, Box<Combinator<T>>),

    /// A sequence of three `T` values.
    Seq3(Box<Combinator<T>>, Box<Combinator<T>>, Box<Combinator<T>>),

    /// A sequence of four `T` values.
    Seq4(
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
    ),

    /// A sequence of five `T` values.
    Seq5(
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
    ),

    /// A sequence of six `T` values.
    Seq6(
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
    ),

    /// A sequence of seven `T` values.
    Seq7(
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
    ),

    /// A sequence of eight `T` values.
    Seq8(
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
    ),

    /// An absent value for optional combinators.
    Void,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[allow(clippy::type_complexity)]
impl<T> Combinator<T> {
    /// Unwraps the combinator as a single value.
    pub fn unwrap_single(self) -> T {
        match self {
            Combinator::Single(x) => x,
            _ => panic!("Combinator::unwrap_single: combinator is not a single value"),
        }
    }

    /// Unwraps the combinator as a choice between two values.
    pub fn unwrap_choice(self) -> Either<Box<Combinator<T>>, Box<Combinator<T>>> {
        match self {
            Combinator::Choice(x) => x,
            _ => panic!("Combinator::unwrap_choice: combinator is not a choice between two values"),
        }
    }

    /// Unwraps the combinator as a vector of repeated values.
    pub fn unwrap_many(self) -> Vec<Combinator<T>> {
        match self {
            Combinator::Many(x) => x,
            _ => panic!("Combinator::unwrap_many: combinator is not a vector of repeated values"),
        }
    }

    /// Unwraps the combinator of a sequence of two values.
    pub fn unwrap_seq2(self) -> (Box<Combinator<T>>, Box<Combinator<T>>) {
        match self {
            Combinator::Seq2(x, y) => (x, y),
            _ => panic!("Combinator::unwrap_seq2: combinator is not a sequence of two values"),
        }
    }

    /// Unwraps the combinator of a sequence of three values.
    pub fn unwrap_seq3(self) -> (Box<Combinator<T>>, Box<Combinator<T>>, Box<Combinator<T>>) {
        match self {
            Combinator::Seq3(x, y, z) => (x, y, z),
            _ => panic!("Combinator::unwrap_seq3: combinator is not a sequence of three values"),
        }
    }

    /// Unwraps the combinator of a sequence of four values.
    pub fn unwrap_seq4(
        self,
    ) -> (
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
    ) {
        match self {
            Combinator::Seq4(x, y, z, w) => (x, y, z, w),
            _ => panic!("Combinator::unwrap_seq4: combinator is not a sequence of four values"),
        }
    }

    /// Unwraps the combinator of a sequence of five values.
    pub fn unwrap_seq5(
        self,
    ) -> (
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
    ) {
        match self {
            Combinator::Seq5(x, y, z, w, v) => (x, y, z, w, v),
            _ => panic!("Combinator::unwrap_seq5: combinator is not a sequence of five values"),
        }
    }

    /// Unwraps the combinator of a sequence of six values.
    pub fn unwrap_seq6(
        self,
    ) -> (
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
    ) {
        match self {
            Combinator::Seq6(x, y, z, w, v, u) => (x, y, z, w, v, u),
            _ => panic!("Combinator::unwrap_seq6: combinator is not a sequence of six values"),
        }
    }

    /// Unwraps the combinator of a sequence of seven values.
    pub fn unwrap_seq7(
        self,
    ) -> (
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
    ) {
        match self {
            Combinator::Seq7(x, y, z, w, v, u, t) => (x, y, z, w, v, u, t),
            _ => panic!("Combinator::unwrap_seq7: combinator is not a sequence of seven values"),
        }
    }

    /// Unwraps the combinator of a sequence of eight values.
    pub fn unwrap_seq8(
        self,
    ) -> (
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
        Box<Combinator<T>>,
    ) {
        match self {
            Combinator::Seq8(x, y, z, w, v, u, t, s) => (x, y, z, w, v, u, t, s),
            _ => panic!("Combinator::unwrap_seq8: combinator is not a sequence of eight values"),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Macros
//--------------------------------------------------------------------------------------------------

/// A parser combinator macro.
///
/// ## Important
///
/// Single parsers are expected to backtrack when they fail.
#[macro_export(local_inner_macros)]
macro_rules! parse {
    // a => a
    ($parser:expr $(, $path:ident)? => $parse:ident) => {
        $( $path :: )? $parse($parser)?.map(|x| $crate::compiler::parser::Combinator::Single(x))
    };
    // (arg a b ...) => [ a(b, ...) ]
    ($parser:expr $(, $path:ident)? => (arg $parse:ident $( $parse_args:tt )+)) => {{
        $( $path :: )? $parse($parser $(, $parse_args )+)?.map(|x| $crate::compiler::parser::Combinator::Single(x))
    }};
    // (opt a) => a?
    ($parser:expr $(, $path:ident)? => (opt $parse:tt)) => {{
        match parse!($parser $(, $path)? => $parse) {
            Some(x) => Some(x),
            None => Some($crate::compiler::parser::Combinator::Void),
        }
    }};
    // (many_0 a) => a*
    ($parser:expr $(, $path:ident)? => (many_0 $parse:tt)) => {{
        let mut result = Vec::new();
        while let Some(__result) = parse!($parser $(, $path)? => $parse) {
            result.push(__result);
        }
        Some($crate::compiler::parser::Combinator::Many(result))
    }};
    // (many_1 a) => a+
    ($parser:expr $(, $path:ident)? => (many_1 $parse:tt)) => {{
        let mut result = Vec::new();
        while let Some(__result) = parse!($parser $(, $path)? => $parse) {
            result.push(__result);
        }
        if !result.is_empty() {
            Some($crate::compiler::parser::Combinator::Many(result))
        } else {
            None
        }
    }};
    // (alt a b) => a | b
    ($parser:expr $(, $path:ident)? => (alt $parse_a:tt $parse_b:tt)) => {{
        if let Some(__result_a) = parse!($parser $(, $path)? => $parse_a) {
            Some($crate::compiler::parser::Combinator::Choice(itertools::Either::Left(Box::new(__result_a))))
        } else if let Some(__result_b) = parse!($parser $(, $path)? => $parse_b) {
            Some($crate::compiler::parser::Combinator::Choice(itertools::Either::Right(Box::new(__result_b))))
        } else {
            None
        }
    }};
    // (seq a b c ...) => a b c ...
    ($parser:expr $(, $path:ident)? => (seq $( $parse:tt )+)) => {{
        use $crate::compiler::reversible::Reversible;
        let state = $parser.get_state();
        if let Some(x) = seq!($parser $(, $path)? => $( $parse )+) {
            Some(x)
        } else {
            $parser.set_state(state);
            None
        }
    }};
    // (perm a b c) => << a b c >>
    ($parser:expr $(, $path:ident)? => (perm $( $parse:tt )+)) => {{
        // use $crate::compiler::parser::capture::StateCapture;
        // let state = $parser.get_state();
        // if let Some(x) = perm!($parser $(, $path)? => $( $parse )+) {
        //     Some(x)
        // } else {
        //     $parser.set_state(state);
        //     None
        // }
    }};
}

/// Sequence combinator macro.
#[macro_export(local_inner_macros)]
macro_rules! seq {
    // Sequence(2) => a b
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt) => {{
        if let Some(__result_a) = parse!($parser $(, $path)? => $parse_a) {
            if let Some(__result_b) = parse!($parser $(, $path)? => $parse_b) {
                Some($crate::compiler::parser::Combinator::Seq2(Box::new(__result_a), Box::new(__result_b)))
            } else {
                None
            }
        } else {
            None
        }
    }};
    // Sequence(3) => a b c
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt) => {{
        if let Some(__result_a) = parse!($parser $(, $path)? => $parse_a) {
            if let Some($crate::compiler::parser::Combinator::Seq2(__result_b, __result_c)) = seq!($parser $(, $path)? => $parse_b $parse_c) {
                Some($crate::compiler::parser::Combinator::Seq3(Box::new(__result_a), __result_b, __result_c))
            } else {
                None
            }
        } else {
            None
        }
    }};
    // Sequence(4) => a b c d
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt) => {{
        if let Some(__result_a) = parse!($parser $(, $path)? => $parse_a) {
            if let Some($crate::compiler::parser::Combinator::Seq3(__result_b, __result_c, __result_d)) = seq!($parser $(, $path)? => $parse_b $parse_c $parse_d) {
                Some($crate::compiler::parser::Combinator::Seq4(Box::new(__result_a), __result_b, __result_c, __result_d))
            } else {
                None
            }
        } else {
            None
        }
    }};
    // Sequence(5) => a b c d e
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt) => {{
        if let Some(__result_a) = parse!($parser $(, $path)? => $parse_a) {
            if let Some($crate::compiler::parser::Combinator::Seq4(__result_b, __result_c, __result_d, __result_e)) = seq!($parser $(, $path)? => $parse_b $parse_c $parse_d $parse_e) {
                Some($crate::compiler::parser::Combinator::Seq5(Box::new(__result_a), __result_b, __result_c, __result_d, __result_e))
            } else {
                None
            }
        } else {
            None
        }
    }};
    // Sequence(6) => a b c d e f
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt) => {{
        if let Some(__result_a) = parse!($parser $(, $path)? => $parse_a) {
            if let Some($crate::compiler::parser::Combinator::Seq5(__result_b, __result_c, __result_d, __result_e, __result_f)) = seq!($parser $(, $path)? => $parse_b $parse_c $parse_d $parse_e $parse_f) {
                Some($crate::compiler::parser::Combinator::Seq6(Box::new(__result_a), __result_b, __result_c, __result_d, __result_e, __result_f))
            } else {
                None
            }
        } else {
            None
        }
    }};
    // Sequence(7) => a b c d e f g
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt) => {{
        if let Some(__result_a) = parse!($parser $(, $path)? => $parse_a) {
            if let Some($crate::compiler::parser::Combinator::Seq6(__result_b, __result_c, __result_d, __result_e, __result_f, __result_g)) = seq!($parser $(, $path)? => $parse_b $parse_c $parse_d $parse_e $parse_f $parse_g) {
                Some($crate::compiler::parser::Combinator::Seq7(Box::new(__result_a), __result_b, __result_c, __result_d, __result_e, __result_f, __result_g))
            } else {
                None
            }
        } else {
            None
        }
    }};
    // Sequence(8) => a b c d e f g h
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt $parse_h:tt) => {{
        if let Some(__result_a) = parse!($parser $(, $path)? => $parse_a) {
            if let Some($crate::compiler::parser::Combinator::Seq7(__result_b, __result_c, __result_d, __result_e, __result_f, __result_g, __result_h)) = seq!($parser $(, $path)? => $parse_b $parse_c $parse_d $parse_e $parse_f $parse_g $parse_h) {
                Some($crate::compiler::parser::Combinator::Seq8(Box::new(__result_a), __result_b, __result_c, __result_d, __result_e, __result_f, __result_g, __result_h))
            } else {
                None
            }
        } else {
            None
        }
    }};
}

// macro_rules! perm {
//     ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt) => {{
//         todo!()
//     }};
// }

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use mock::*;
    use tracing::info;

    #[test_log::test]
    fn test_combinator_perm() -> anyhow::Result<()> {
        // let mut parser = &mut Parser::new(['a', 'b']);
        // let result = parse!(&mut parser, Parser => (perm parse_a parse_b));
        // info!(
        //     "input = {:?} | (perm parse_a parse_b) = {:?}",
        //     parser.input, result
        // );
        // assert_eq!(
        //     result,
        //     Some(Combinator::Seq2(
        //         Box::new(Combinator::Single(Ast::A)),
        //         Box::new(Combinator::Single(Ast::B))
        //     ))
        // );

        // let mut parser = &mut Parser::new(['b', 'a']);
        // let result = perm!(&mut parser, Parser => parse_a parse_b);
        // info!(
        //     "input = {:?} | (perm parse_a parse_b) = {:?}",
        //     parser.input, result
        // );
        // assert_eq!(
        //     result,
        //     Some(Combinator::Seq2(
        //         Box::new(Combinator::Single(Ast::B)),
        //         Box::new(Combinator::Single(Ast::A))
        //     ))
        // );

        // for i in ('a'..='c').permutations(3) {
        //     let mut parser = &mut Parser::new(i);
        //     let result = parse!(&mut parser, Parser => (perm parse_a parse_b parse_c));
        //     info!(
        //         "input = {:?} | (perm parse_a parse_b parse_c) = {:?}",
        //         parser.input, result
        //     );
        //     assert!(matches!(result, Some(Combinator::Seq3(_, _, _))));
        // }

        // for i in ('a'..='d').permutations(4) {
        //     let mut parser = &mut Parser::new(i);
        //     let result = parse!(&mut parser, Parser => (perm parse_a parse_b parse_c parse_d));
        //     info!(
        //         "input = {:?} | (perm parse_a parse_b parse_c parse_d) = {:?}",
        //         parser.input, result
        //     );
        //     assert!(matches!(result, Some(Combinator::Seq4(_, _, _, _))));
        // }

        // for i in ('a'..='e').permutations(5) {
        //     let mut parser = &mut Parser::new(i);
        //     let result = parse!(&mut parser, Parser => (perm parse_a parse_b parse_c parse_d parse_e));
        //     assert!(matches!(result, Some(Combinator::Seq5(_, _, _, _, _))));
        // }

        // for i in ('a'..='f').permutations(6) {
        //     let mut parser = &mut Parser::new(i);
        //     let result = parse!(&mut parser, Parser => (perm parse_a parse_b parse_c parse_d parse_e parse_f));
        //     assert!(matches!(result, Some(Combinator::Seq6(_, _, _, _, _, _))));
        // }

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
            Some(Combinator::Choice(itertools::Either::Left(Box::new(
                Combinator::Single(Ast::A)
            ))))
        );

        // Fail Cases
        let parser = &mut Parser::new(['b']);
        let result = parse!(parser, Parser => (alt parse_a parse_b));
        info!(
            "input = {:?} | (alt parse_a parse_b) = {:?}",
            parser.input, result
        );
        assert_eq!(
            result,
            Some(Combinator::Choice(itertools::Either::Right(Box::new(
                Combinator::Single(Ast::B)
            ))))
        );

        let parser = &mut Parser::new(['b']);
        let result = parse!(parser, Parser => (alt parse_b parse_c));
        info!(
            "input = {:?} | (alt parse_b parse_c) = {:?}",
            parser.input, result
        );
        assert_eq!(
            result,
            Some(Combinator::Choice(itertools::Either::Left(Box::new(
                Combinator::Single(Ast::B)
            ))))
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
        let result =
            parse!(parser, Parser => (seq parse_a parse_b parse_c parse_d parse_a parse_b));
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
        let parser = &mut Parser::new(['a']);
        let result = parse!(parser, Parser => (alt parse_a (seq parse_a parse_b)));
        info!(
            "input = {:?} | (alt parse_a (seq parse_a parse_b)) = {:?}",
            parser.input, result
        );
        assert_eq!(
            result,
            Some(Combinator::Choice(Either::Left(Box::new(
                Combinator::Single(Ast::A)
            ))))
        );

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
                Box::new(Combinator::Choice(Either::Left(Box::new(
                    Combinator::Single(Ast::A)
                ))))
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
            Some(Combinator::Choice(Either::Right(Box::new(
                Combinator::Seq2(
                    Box::new(Combinator::Single(Ast::B)),
                    Box::new(Combinator::Single(Ast::C))
                )
            ))))
        );

        let parser = &mut Parser::new(['a', 'd']);
        let result = parse!(parser, Parser => (alt (seq parse_a parse_b) parse_a));
        info!(
            "input = {:?} | (alt (seq parse_a parse_b) parse_a) = {:?}",
            parser.input, result
        );
        assert_eq!(
            result,
            Some(Combinator::Choice(Either::Right(Box::new(
                Combinator::Single(Ast::A)
            ))))
        );

        let parser = &mut Parser::new(['a', 'a', 'c']);
        let result = parse!(parser, Parser => (alt (seq parse_a parse_b parse_c) (alt (seq parse_a parse_b) parse_a)));
        info!("input = {:?} | (alt (seq parse_a parse_b parse_c) (alt (seq parse_a parse_b) parse_a)) = {:?}", parser.input, result);
        assert_eq!(
            result,
            Some(Combinator::Choice(Either::Right(Box::new(
                Combinator::Choice(Either::Right(Box::new(Combinator::Single(Ast::A))))
            ))))
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
        let result =
            parse!(parser, Parser => (seq (opt (seq parse_a parse_b)) (seq parse_a parse_d)));
        info!(
            "input = {:?} | (alt (opt (seq parse_a parse_b)) (seq parse_a parse_b)) = {:?}",
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
            Some(Combinator::Choice(Either::Right(Box::new(
                Combinator::Choice(Either::Right(Box::new(Combinator::Choice(Either::Right(
                    Box::new(Combinator::Single(Ast::D))
                )))))
            ))))
        );

        Ok(())
    }
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
        // E,
        // F,
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

        // pub(super) fn parse_e(&mut self) -> anyhow::Result<Option<Ast>> {
        //     match self.input.get(self.cursor) {
        //         Some('e') => {
        //             self.cursor += 1;
        //             Ok(Some(Ast::E))
        //         }
        //         _ => Ok(None),
        //     }
        // }

        // pub(super) fn parse_f(&mut self) -> anyhow::Result<Option<Ast>> {
        //     match self.input.get(self.cursor) {
        //         Some('f') => {
        //             self.cursor += 1;
        //             Ok(Some(Ast::F))
        //         }
        //         _ => Ok(None),
        //     }
        // }
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
