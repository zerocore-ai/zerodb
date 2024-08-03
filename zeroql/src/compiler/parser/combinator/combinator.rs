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
    ($parser:expr $(, $path:ident)? => $parse:ident) => {{
        tracing::debug!("entered {}", std::stringify!($parse));
        $( $path :: )? $parse($parser)?.map(|x| $crate::compiler::parser::Combinator::Single(x))
    }};
    // (arg a b ...) => [ a(b, ...) ]
    ($parser:expr $(, $path:ident)? => (arg $parse:ident $( $parse_args:tt )+)) => {{
        tracing::debug!("entered (arg {} {})", std::stringify!($parse), concat_with::concat!(with " ", $( std::stringify!($parse_args) ),+));
        $( $path :: )? $parse($parser $(, $parse_args )+)?.map(|x| $crate::compiler::parser::Combinator::Single(x))
    }};
    // (opt a) => a?
    ($parser:expr $(, $path:ident)? => (opt $parse:tt)) => {{
        tracing::debug!("entered (opt {:?})", std::stringify!($parse));
        match parse!($parser $(, $path)? => $parse) {
            Some(x) => Some(x),
            None => Some($crate::compiler::parser::Combinator::Void),
        }
    }};
    // (many_0 a) => a*
    ($parser:expr $(, $path:ident)? => (many_0 $parse:tt)) => {{
        tracing::debug!("entered (many_0 {:?})", std::stringify!($parse));
        let mut result = Vec::new();
        while let Some(__result) = parse!($parser $(, $path)? => $parse) {
            result.push(__result);
        }
        Some($crate::compiler::parser::Combinator::Many(result))
    }};
    // (many_1 a) => a+
    ($parser:expr $(, $path:ident)? => (many_1 $parse:tt)) => {{
        tracing::debug!("entered (many_1 {:?})", std::stringify!($parse));
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
        tracing::debug!("entered (alt {:?})", concat_with::concat!(with " ", $(std::stringify!($parse)),+));
        alt!($parser $(, $path)? => $( $parse )+)
    }};
    // (seq a b ...) => a b ...
    ($parser:expr $(, $path:ident)? => (seq $( $parse:tt )+)) => {{
        use $crate::compiler::reversible::Reversible;
        let state = $parser.get_state();
        tracing::debug!("entered (seq {:?})", concat_with::concat!(with " ", $(std::stringify!($parse)),+));
        if let Some(x) = seq!($parser $(, $path)? => $( $parse )+) {
            tracing::debug!("success seq");
            Some(x)
        } else {
            tracing::debug!("failed seq");
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

/// Alternative combinator macro.
#[macro_export(local_inner_macros)]
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
        if let Some(__result) = alt!($parser $(, $path)? => $parse_a $parse_b) {
            Some(__result)
        } else if let Some(__result_c) = parse!($parser $(, $path)? => $parse_c)  {
            Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::C(Box::new(__result_c))))
        } else {
            None
        }
    }};
    // Alternative(4) => a | b | c | d
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt) => {{
        if let Some(__result) = alt!($parser $(, $path)? => $parse_a $parse_b $parse_c) {
            Some(__result)
        } else if let Some(__result_d) = parse!($parser $(, $path)? => $parse_d)  {
            Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::D(Box::new(__result_d))))
        } else {
            None
        }
    }};
    // Alternative(5) => a | b | c | d | e
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt) => {{
        if let Some(__result) = alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d) {
            Some(__result)
        } else if let Some(__result_e) = parse!($parser $(, $path)? => $parse_e)  {
            Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::E(Box::new(__result_e))))
        } else {
            None
        }
    }};
    // Alternative(6) => a | b | c | d | e | f
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt) => {{
        if let Some(__result) = alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d $parse_e) {
            Some(__result)
        } else if let Some(__result_f) = parse!($parser $(, $path)? => $parse_f)  {
            Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::F(Box::new(__result_f))))
        } else {
            None
        }
    }};
    // Alternative(7) => a | b | c | d | e | f | g
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt) => {{
        if let Some(__result) = alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d $parse_e $parse_f) {
            Some(__result)
        } else if let Some(__result_g) = parse!($parser $(, $path)? => $parse_g)  {
            Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::G(Box::new(__result_g))))
        } else {
            None
        }
    }};
    // Alternative(8) => a | b | c | d | e | f | g | h
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt $parse_h:tt) => {{
        if let Some(__result) = alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d $parse_e $parse_f $parse_g) {
            Some(__result)
        } else if let Some(__result_h) = parse!($parser $(, $path)? => $parse_h)  {
            Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::H(Box::new(__result_h))))
        } else {
            None
        }
    }};
    // Alternative(9) => a | b | c | d | e | f | g | h | i
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt $parse_h:tt $parse_i:tt) => {{
        if let Some(__result) = alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d $parse_e $parse_f $parse_g $parse_h) {
            Some(__result)
        } else if let Some(__result_i) = parse!($parser $(, $path)? => $parse_i)  {
            Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::I(Box::new(__result_i))))
        } else {
            None
        }
    }};
    // Alternative(10) => a | b | c | d | e | f | g | h | i | j
    ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt $parse_c:tt $parse_d:tt $parse_e:tt $parse_f:tt $parse_g:tt $parse_h:tt $parse_i:tt $parse_j:tt) => {{
        if let Some(__result) = alt!($parser $(, $path)? => $parse_a $parse_b $parse_c $parse_d $parse_e $parse_f $parse_g $parse_h $parse_i) {
            Some(__result)
        } else if let Some(__result_j) = parse!($parser $(, $path)? => $parse_j)  {
            Some($crate::compiler::parser::Combinator::Choice($crate::compiler::parser::combinator::Choice::J(Box::new(__result_j))))
        } else {
            None
        }
    }};
}

// /// Permutation combinator macro.
// macro_rules! perm {
//     ($parser:expr $(, $path:ident)? => $parse_a:tt $parse_b:tt) => {{
//         todo!()
//     }};
// }
