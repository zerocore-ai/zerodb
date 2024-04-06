use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, FnArg, Ident, Pat, Result,
};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const CRATE_NAME: &str = "zeroql-macros";

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn get_fn_arg_names(
    fn_inputs: &Punctuated<FnArg, Comma>,
) -> impl Iterator<Item = Result<Ident>> + '_ {
    // TODO: Improve error message
    fn_inputs.iter().map(|arg| match arg {
        FnArg::Typed(arg) => match &*arg.pat {
            Pat::Ident(ident) => Ok(ident.ident.clone()),
            _ => Err(syn::Error::new(
                arg.pat.span(),
                "Expected an identifier for the argument",
            )),
        },
        FnArg::Receiver(arg) => match arg.reference {
            Some(_) => Ok(format_ident!("self")),
            None => Err(syn::Error::new(
                arg.span(),
                "Expected a reference for the argument",
            )),
        },
    })
}

pub(crate) fn fn_call(fn_inputs: &Punctuated<FnArg, Comma>, fn_name: &Ident) -> TokenStream {
    let fn_arg_names = get_fn_arg_names(fn_inputs)
        .collect::<Result<Vec<_>>>()
        .unwrap();

    if fn_arg_names.iter().any(|arg| arg == &format_ident!("self")) {
        quote! { Self::#fn_name ( #(#fn_arg_names),* ) }
    } else {
        quote! { #fn_name ( #(#fn_arg_names),* ) }
    }
}

pub(crate) fn crate_path(fn_name: &Ident) -> Result<Ident> {
    let Ok(crate_name) = crate_name(CRATE_NAME) else {
        return Err(syn::Error::new(
            fn_name.span(),
            format!("Could not find `{CRATE_NAME}` crate in your `Cargo.toml`"),
        ));
    };

    Ok(match crate_name {
        FoundCrate::Itself => format_ident!("{}", CRATE_NAME.replace('-', "_")),
        FoundCrate::Name(name) => format_ident!("{name}"),
    })
}

pub fn has_attr(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(name))
}
