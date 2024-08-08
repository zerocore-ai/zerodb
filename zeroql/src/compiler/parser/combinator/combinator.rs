use crate::compiler::reversible::Reversible;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a combinator expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Combinator<T> {
    /// A single `T` value.
    Single(T),

    /// A choice between two combinators.
    Choice(Choice<T>),

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

/// A choice between multiple combinators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice<T> {
    /// First alternative.
    A(Box<Combinator<T>>),

    /// Second alternative.
    B(Box<Combinator<T>>),

    /// Third alternative.
    C(Box<Combinator<T>>),

    /// Fourth alternative.
    D(Box<Combinator<T>>),

    /// Fifth alternative.
    E(Box<Combinator<T>>),

    /// Sixth alternative.
    F(Box<Combinator<T>>),

    /// Seventh alternative.
    G(Box<Combinator<T>>),

    /// Eighth alternative.
    H(Box<Combinator<T>>),

    /// Ninth alternative.
    I(Box<Combinator<T>>),

    /// Tenth alternative.
    J(Box<Combinator<T>>),
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
    pub fn unwrap_choice(self) -> Choice<T> {
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
// Functions
//--------------------------------------------------------------------------------------------------

/// Calls provided parser functions in different permutation sequences.
pub fn permute<T: Reversible, U, E>(
    parser: &mut T,
    mut parser_funcs: Vec<(
        usize,
        Box<dyn Fn(&mut T) -> Result<Option<Combinator<U>>, E>>,
    )>,
) -> std::collections::HashMap<usize, Combinator<U>> {
    let mut result_map = std::collections::HashMap::<usize, Combinator<U>>::new();
    let mut prev_len = std::usize::MAX;

    // Keep looping until the parser functions vector is exhausted.
    while parser_funcs.len() < prev_len {
        prev_len = parser_funcs.len();

        // Remove parser functions that succeed and insert the result into the map.
        parser_funcs.retain(|(index, f)| {
            if let Ok(Some(result)) = f(parser) {
                result_map.insert(*index, result);
                return false;
            }

            true
        });
    }

    result_map
}

/// Permutes parser functions with proper optional combinator support.
pub fn permute_opt<T: Reversible, U: Clone, E>(
    index: usize,
    parser: &mut T,
    // Parser functions and their rule positions.
    parser_funcs: Vec<(
        usize,
        Box<dyn Fn(&mut T) -> Result<Option<Combinator<U>>, E>>,
    )>,
) -> Result<
    (
        std::collections::BTreeMap<usize, Combinator<U>>,
        std::collections::HashSet<usize>,
    ),
    E,
> {
    let mut map = std::collections::BTreeMap::new();

    // For each rule position, find the first parser function that succeeds and insert it into the map.
    for _ in 0..index {
        for (index, f) in parser_funcs.iter() {
            if let Some(combinator) = f(parser)? {
                map.insert(*index, combinator);
                break;
            }
        }
    }

    // Collect map indices before further processing.
    let map_indices = map
        .keys()
        .cloned()
        .collect::<std::collections::HashSet<_>>();

    // Fill unoccupied indices with void combinators.
    for i in 0..index {
        if map.get(&i).is_none() {
            map.insert(i, Combinator::Void);
        }
    }

    Ok((map, map_indices))
}

//--------------------------------------------------------------------------------------------------
// Macros
//--------------------------------------------------------------------------------------------------

/// A parser combinator macro for convenience.
///
/// ## Important
///
/// Single parsers are expected to backtrack when they fail.
#[macro_export(local_inner_macros)]
macro_rules! parse {
    // a => a
    ($parser:expr $(, $path:ident)? => $parse:ident) => {{
        // $( $path :: )? $parse($parser)?.map(|x| $crate::compiler::parser::Combinator::Single(x))
        match $( $path :: )? $parse($parser) {
            Ok(Some(x)) => Some($crate::compiler::parser::Combinator::Single(x)),
            Ok(None) => None,
            Err(e) => return Err(e),
        }
    }};
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
    // (alt a b ...) => a | b | ...
    ($parser:expr $(, $path:ident)? => (alt $( $parse:tt )+)) => {{
        $crate::compiler::parser::combinator::inner::alt!($parser $(, $path)? => $( $parse )+)
    }};
    // (seq a b ...) => a b ...
    ($parser:expr $(, $path:ident)? => (seq $( $parse:tt )+)) => {{
        let state = <_ as $crate::compiler::reversible::Reversible>::get_state($parser);
        if let Some(x) = $crate::compiler::parser::combinator::inner::seq!($parser $(, $path)? => $( $parse )+) {
            Some(x)
        } else {
            <_ as $crate::compiler::reversible::Reversible>::set_state($parser, state);
            None
        }
    }};
    // (perm a b c) => << a b c >> // This won't work well with optional combinators
    ($parser:expr $(, $path:ident)? => (perm $( $parse:tt )+)) => {{
        let mut index = 0;
        let mut parser_funcs = std::vec::Vec::<(
            usize,
            Box<dyn Fn(&mut _) -> std::result::Result<std::option::Option<Combinator<_>>, _>>,
        )>::new();

        $(
            let parser_func = |parser: &mut _| -> std::result::Result<std::option::Option<Combinator<_>>, _> {
                Ok(parse!(parser, Parser => $parse))
            };

            parser_funcs.push((index, Box::new(parser_func)));

            index += 1;
        )+

        let mut result = $crate::compiler::parser::combinator::permute($parser, parser_funcs);
        tracing::trace!("result = {:?}", result);

        if result.len() == index {
            Some($crate::compiler::parser::combinator::inner::perm_result!(result, $( $parse )+))
        } else {
            None
        }
    }};
    // (perm_opt (opt a) b (opt c)) => << a? b c? >>
    // This macro supports optional combinators at the top-level for convenience.
    // However, it comes at the cost of performance.
    ($parser:expr $(, $path:ident)? => (perm_opt $( $parse:tt )+)) => {{
        let mut index = 0;
        let mut non_optionals = std::collections::HashSet::<usize>::new();
        let mut parser_funcs = std::vec::Vec::<(
            usize,
            Box<dyn Fn(&mut _) -> std::result::Result<std::option::Option<Combinator<_>>, _>>,
        )>::new();

        $crate::compiler::parser::combinator::inner::perm_opt_args!(index, parser_funcs, non_optionals, $parser $(, $path)? => $( $parse )+);
        let (mut map, map_indices) = $crate::compiler::parser::combinator::permute_opt(index, $parser, parser_funcs)?;

        if map_indices.is_superset(&non_optionals) {
            Some($crate::compiler::parser::combinator::inner::perm_result!(map, $( $parse )+))
        } else {
            None
        }
    }}
}

pub(crate) mod inner {
    /// Sequence combinator macro.
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
                if let Some($crate::compiler::parser::Combinator::Seq2(__result_b, __result_c)) = $crate::compiler::parser::combinator::inner::seq!($parser $(, $path)? => $parse_b $parse_c) {
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
                if let Some($crate::compiler::parser::Combinator::Seq3(__result_b, __result_c, __result_d)) = $crate::compiler::parser::combinator::inner::seq!($parser $(, $path)? => $parse_b $parse_c $parse_d) {
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
                if let Some($crate::compiler::parser::Combinator::Seq4(__result_b, __result_c, __result_d, __result_e)) = $crate::compiler::parser::combinator::inner::seq!($parser $(, $path)? => $parse_b $parse_c $parse_d $parse_e) {
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
                if let Some($crate::compiler::parser::Combinator::Seq5(__result_b, __result_c, __result_d, __result_e, __result_f)) = $crate::compiler::parser::combinator::inner::seq!($parser $(, $path)? => $parse_b $parse_c $parse_d $parse_e $parse_f) {
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
                if let Some($crate::compiler::parser::Combinator::Seq6(__result_b, __result_c, __result_d, __result_e, __result_f, __result_g)) = $crate::compiler::parser::combinator::inner::seq!($parser $(, $path)? => $parse_b $parse_c $parse_d $parse_e $parse_f $parse_g) {
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
                if let Some($crate::compiler::parser::Combinator::Seq7(__result_b, __result_c, __result_d, __result_e, __result_f, __result_g, __result_h)) = $crate::compiler::parser::combinator::inner::seq!($parser $(, $path)? => $parse_b $parse_c $parse_d $parse_e $parse_f $parse_g $parse_h) {
                    Some($crate::compiler::parser::Combinator::Seq8(Box::new(__result_a), __result_b, __result_c, __result_d, __result_e, __result_f, __result_g, __result_h))
                } else {
                    None
                }
            } else {
                None
            }
        }};
    }

    /// Alternative combinator macro.
    macro_rules! alt {
        // Alternative(2) => a | b
        ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt) => {{
            if let Some(__result_a) = parse!($parser $(, $path)? => $parse_a) {
                Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::A(Box::new(__result_a))))
            } else if let Some(__result_b) = parse!($parser $(, $path)? => $parse_b) {
                Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::B(Box::new(__result_b))))
            } else {
                None
            }
        }};
        // Alternative(3) => a | b | c
        ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt) => {{
            if let Some(__result) = $crate::compiler::parser::combinator::inner::alt!($parser $(, $path)? => $parse_a $parse_b) {
                Some(__result)
            } else if let Some(__result_c) = parse!($parser $(, $path)? => $parse_c)  {
                Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::C(Box::new(__result_c))))
            } else {
                None
            }
        }};
        // Alternative(4) => a | b | c | d
        ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt) => {{
            if let Some(__result) = $crate::compiler::parser::combinator::inner::alt!($parser $(, $path)? => $parse_a $parse_b $parse_c) {
                Some(__result)
            } else if let Some(__result_d) = parse!($parser $(, $path)? => $parse_d)  {
                Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::D(Box::new(__result_d))))
            } else {
                None
            }
        }};
        // Alternative(5) => a | b | c | d | e
        ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt) => {{
            if let Some(__result) = $crate::compiler::parser::combinator::inner::alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d) {
                Some(__result)
            } else if let Some(__result_e) = parse!($parser $(, $path)? => $parse_e)  {
                Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::E(Box::new(__result_e))))
            } else {
                None
            }
        }};
        // Alternative(6) => a | b | c | d | e | f
        ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt) => {{
            if let Some(__result) = $crate::compiler::parser::combinator::inner::alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d $parse_e) {
                Some(__result)
            } else if let Some(__result_f) = parse!($parser $(, $path)? => $parse_f)  {
                Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::F(Box::new(__result_f))))
            } else {
                None
            }
        }};
        // Alternative(7) => a | b | c | d | e | f | g
        ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt) => {{
            if let Some(__result) = $crate::compiler::parser::combinator::inner::alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d $parse_e $parse_f) {
                Some(__result)
            } else if let Some(__result_g) = parse!($parser $(, $path)? => $parse_g)  {
                Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::G(Box::new(__result_g))))
            } else {
                None
            }
        }};
        // Alternative(8) => a | b | c | d | e | f | g | h
        ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt $parse_h:tt) => {{
            if let Some(__result) = $crate::compiler::parser::combinator::inner::alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d $parse_e $parse_f $parse_g) {
                Some(__result)
            } else if let Some(__result_h) = parse!($parser $(, $path)? => $parse_h)  {
                Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::H(Box::new(__result_h))))
            } else {
                None
            }
        }};
        // Alternative(9) => a | b | c | d | e | f | g | h | i
        ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt $parse_h:tt $parse_i:tt) => {{
            if let Some(__result) = $crate::compiler::parser::combinator::inner::alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d $parse_e $parse_f $parse_g $parse_h) {
                Some(__result)
            } else if let Some(__result_i) = parse!($parser $(, $path)? => $parse_i)  {
                Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::I(Box::new(__result_i))))
            } else {
                None
            }
        }};
        // Alternative(10) => a | b | c | d | e | f | g | h | i | j
        ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt $parse_h:tt $parse_i:tt $parse_j:tt) => {{
            if let Some(__result) = $crate::compiler::parser::combinator::inner::alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d $parse_e $parse_f $parse_g $parse_h $parse_i) {
                Some(__result)
            } else if let Some(__result_j) = parse!($parser $(, $path)? => $parse_j)  {
                Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::J(Box::new(__result_j))))
            } else {
                None
            }
        }};
    }

    /// Permutation combinator macro result.
    macro_rules! perm_result {
        ($result:expr, $parse_a:tt $parse_b:tt) => {{
            Combinator::Seq2(
                Box::new($result.remove(&0).unwrap()),
                Box::new($result.remove(&1).unwrap()),
            )
        }};
        ($result:expr, $parse_a:tt $parse_b:tt $parse_c:tt) => {{
            Combinator::Seq3(
                Box::new($result.remove(&0).unwrap()),
                Box::new($result.remove(&1).unwrap()),
                Box::new($result.remove(&2).unwrap()),
            )
        }};
        ($result:expr, $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt) => {{
            Combinator::Seq4(
                Box::new($result.remove(&0).unwrap()),
                Box::new($result.remove(&1).unwrap()),
                Box::new($result.remove(&2).unwrap()),
                Box::new($result.remove(&3).unwrap()),
            )
        }};
        ($result:expr, $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt) => {{
            Combinator::Seq5(
                Box::new($result.remove(&0).unwrap()),
                Box::new($result.remove(&1).unwrap()),
                Box::new($result.remove(&2).unwrap()),
                Box::new($result.remove(&3).unwrap()),
                Box::new($result.remove(&4).unwrap()),
            )
        }};
        ($result:expr, $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt) => {{
            Combinator::Seq6(
                Box::new($result.remove(&0).unwrap()),
                Box::new($result.remove(&1).unwrap()),
                Box::new($result.remove(&2).unwrap()),
                Box::new($result.remove(&3).unwrap()),
                Box::new($result.remove(&4).unwrap()),
                Box::new($result.remove(&5).unwrap()),
            )
        }};
        ($result:expr, $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt) => {{
            Combinator::Seq7(
                Box::new($result.remove(&0).unwrap()),
                Box::new($result.remove(&1).unwrap()),
                Box::new($result.remove(&2).unwrap()),
                Box::new($result.remove(&3).unwrap()),
                Box::new($result.remove(&4).unwrap()),
                Box::new($result.remove(&5).unwrap()),
                Box::new($result.remove(&6).unwrap()),
            )
        }};
        ($result:expr, $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt $parse_h:tt) => {{
            Combinator::Seq8(
                Box::new($result.remove(&0).unwrap()),
                Box::new($result.remove(&1).unwrap()),
                Box::new($result.remove(&2).unwrap()),
                Box::new($result.remove(&3).unwrap()),
                Box::new($result.remove(&4).unwrap()),
                Box::new($result.remove(&5).unwrap()),
                Box::new($result.remove(&6).unwrap()),
                Box::new($result.remove(&7).unwrap()),
            )
        }};
    }

    /// Permutation combinator macro.
    macro_rules! perm_opt_args {
        ($index:expr, $parser_funcs:expr, $non_optionals:expr, $parser:expr, $path:ident => $parse_0:tt $( $parse_rest:tt )+) => {{
            $crate::compiler::parser::combinator::inner::perm_opt_args!($index, $parser_funcs, $non_optionals, $parser, $path => $parse_0);
            $(
                $crate::compiler::parser::combinator::inner::perm_opt_args!($index, $parser_funcs, $non_optionals, $parser, $path => $parse_rest);
            )+
        }};
        ($index:expr, $parser_funcs:expr, $non_optionals:expr, $parser:expr => $parse_0:tt $( $parse_rest:tt )+) => {{
            $crate::compiler::parser::combinator::inner::perm_opt_args!($index, $parser_funcs, $non_optionals, $parser => $parse_0);
            $(
                $crate::compiler::parser::combinator::inner::perm_opt_args!($index, $parser_funcs, $non_optionals, $parser => $parse_rest);
            )+
        }};
        ($index:expr, $parser_funcs:expr, $non_optionals:expr, $parser:expr $(, $path:ident)? => (opt $parse:tt)) => {{
            {
                let parser_func =
                    |parser: &mut _| -> std::result::Result<std::option::Option<Combinator<_>>, _> {
                        Ok(parse!(parser $(, $path)? => $parse))
                    };
                $parser_funcs.push(($index, Box::new(parser_func)));
                $index += 1;
            }
        }};
        ($index:expr, $parser_funcs:expr, $non_optionals:expr, $parser:expr $(, $path:ident)? => $parse:tt) => {{
            {
                let parser_func =
                    |parser: &mut _| -> std::result::Result<std::option::Option<Combinator<_>>, _> {
                        Ok(parse!(parser $(, $path)? => $parse))
                    };
                $parser_funcs.push(($index, Box::new(parser_func)));
                $non_optionals.insert($index);
                $index += 1;
            }
        }}
    }

    pub(crate) use alt;
    pub(crate) use perm_opt_args;
    pub(crate) use perm_result;
    pub(crate) use seq;
}
