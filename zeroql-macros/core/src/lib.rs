mod backtrack;
mod memoize;
mod utils;

//--------------------------------------------------------------------------------------------------
// Procedural Macros
//--------------------------------------------------------------------------------------------------

use backtrack::BacktrackOptions;
use memoize::MemoizeOptions;
use proc_macro::TokenStream;

/// Backtracks the function, allowing the function to return to a previous state.
#[proc_macro_attribute]
pub fn backtrack(attr: TokenStream, item: TokenStream) -> TokenStream {
    let options: BacktrackOptions = syn::parse(attr).unwrap();
    let fn_tree = syn::parse_macro_input!(item as syn::ItemFn);
    backtrack::generate(options, &fn_tree).into()
}

/// Memoizes the function, caching the result of the function call based on the input arguments.
#[proc_macro_attribute]
pub fn memoize(attr: TokenStream, item: TokenStream) -> TokenStream {
    let options: MemoizeOptions = syn::parse(attr).unwrap();
    let fn_tree = syn::parse_macro_input!(item as syn::ItemFn);
    memoize::generate(options, &fn_tree).into()
}
