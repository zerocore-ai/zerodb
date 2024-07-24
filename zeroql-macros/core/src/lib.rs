mod backtrack;
mod memoize;
mod utils;

use backtrack::BacktrackOptions;
use memoize::MemoizeOptions;
use proc_macro::TokenStream;

//--------------------------------------------------------------------------------------------------
// Attribute Procedural Macros
//--------------------------------------------------------------------------------------------------

/// Backtracks the function, allowing the function to return to a previous state.
///
/// `backtrack` works by taking field representing the state it has to revert when the function
/// returns a `None`, therefore, requiring that the function returns a clone and that state objet
/// implememnts `Clone`.
///
/// This attribute macro is implemented with parsers in mind but it can find use in other contexts as well.
///
/// # Example
///
/// ```no_run
/// use zeroql_macros::backtrack;
///
/// struct Counter {
///     state: usize,
/// }
///
/// #[backtrack(state = self.state, condition = |r| r.is_none())]
/// impl Counter {
///     #[backtrack]
///     fn inc_even(&mut self, n: usize) -> Option<usize> {
///         c.state += n; // Modify the state ahead of time
///         if n % 2 == 0 {
///             return Some(c.state);
///         }
///         None
///     }
/// }
///
/// let mut counter = Counter { state: 0 };
/// assert_eq!(counter.inc_even(2), Some(2));
/// assert_eq!(counter.inc_even(3), None);
/// ```
///
/// You can also use `backtrack` on functions:
///
/// ```no_run
/// #[backtrack(state = c.state, condition = |r| r.is_none())]
/// fn inc_even(&mut self, n: usize) -> Option<usize> {
///     c.state += n;
///     if n % 2 == 0 {
///         return Some(c.state);
///     }
///     None
/// }
/// ```
#[proc_macro_attribute]
pub fn backtrack(attr: TokenStream, item: TokenStream) -> TokenStream {
    let options: BacktrackOptions = match syn::parse(attr) {
        Ok(options) => options,
        Err(err) => return err.to_compile_error().into(),
    };

    if let Ok(impl_tree) = syn::parse::<syn::ItemImpl>(item.clone()) {
        return backtrack::impl_generate(&options, &impl_tree).into();
    }

    let fn_tree = syn::parse_macro_input!(item as syn::ItemFn);
    backtrack::fn_generate(options, &fn_tree).into()
}

/// Memoizes a function, caching the result of the function call based on the input arguments.
///
/// `memoize` works by combining the function's name and arguments into a single key that is used to
/// cache the result of the function call so it requires that the arguments implement `Eq`, `Hash`, and
/// `Clone`. When used on a method, the `self` argument is not made part of the key.
///
/// If a cache is passed, the cache must implement [`Cache`][zero_4`3macros::cache::Cache] and must take
/// [`Box<dyn Anykey>`][zero_macros::anykey::AnyKey]
///
/// This attribute macro is implemented with parsers in mind but it can find use in other contexts as well.
///
/// # Example
///
/// ```no_run
/// use zeroql_macros::{memoize, anykey::AnyKey};
/// use std::collections::HashMap;
///
/// struct RandomCompute {
///    cache: HashMap<Box<dyn AnyKey>, usize>,
/// }
///
/// #[memoize(cache = self.cache)]
/// impl RandomCompute {
///     #[memoize]
///     fn plus_rand(&mut self, x: usize) -> usize {
///         x + rand::random::<usize>()
///     }
/// }
///
/// let mut computer = RandomCompute { cache: HashMap::new() };
/// let a = computer.plus_rand(1);
/// let b = computer.plus_rand(1);
///
/// assert_eq!(a, b);
/// ```
///
/// You can also use `memoize` on functions:
///
/// ```no_run
/// #[memoize(cache = r.cache)]
/// fn plus_rand(r: &mut RandomCompute, x: usize) -> usize {
///     x + rand::random::<usize>()
/// }
#[proc_macro_attribute]
pub fn memoize(attr: TokenStream, item: TokenStream) -> TokenStream {
    let options: MemoizeOptions = match syn::parse(attr) {
        Ok(options) => options,
        Err(err) => return err.to_compile_error().into(),
    };

    if let Ok(impl_tree) = syn::parse::<syn::ItemImpl>(item.clone()) {
        return memoize::impl_generate(&options, &impl_tree).into();
    }

    let fn_tree = syn::parse_macro_input!(item as syn::ItemFn);
    memoize::fn_generate(options, &fn_tree).into()
}
