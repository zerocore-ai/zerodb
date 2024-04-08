use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    ExprField, Ident, ImplItemFn, ItemFn, ItemImpl, Token,
};

use crate::utils;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

pub(super) struct BacktrackOptions {
    state: ExprField,
}

//--------------------------------------------------------------------------------------------------
// Modules
//--------------------------------------------------------------------------------------------------

mod keyword {
    use syn::custom_keyword;

    custom_keyword!(state);
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Parse for BacktrackOptions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(keyword::state) {
            input.parse::<keyword::state>()?;
            input.parse::<Token![=]>()?;
            Ok(Self {
                state: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions: Entry Point
//--------------------------------------------------------------------------------------------------

pub(super) fn impl_generate(options: &BacktrackOptions, impl_tree: &ItemImpl) -> TokenStream {
    let mut methods = Vec::new();
    for method in impl_tree.items.iter() {
        if let syn::ImplItem::Fn(method) = method {
            if utils::has_attr(&method.attrs, "backtrack") {
                methods.push(generate_transformed_impl_method(method, options));
            } else {
                methods.push(quote! { #method });
            }
        }
    }

    generate_transformed_impl(impl_tree, methods)
}

pub(super) fn fn_generate(options: BacktrackOptions, fn_tree: &ItemFn) -> TokenStream {
    let fn_updated_name = &format_ident!("__backtrack_original_{}", fn_tree.sig.ident);
    let renamed_fn = generate_renamed_fn(fn_updated_name, fn_tree);
    let transformed_fn = generate_transformed_fn(fn_updated_name, fn_tree, options);

    quote! {
        #renamed_fn
        #transformed_fn
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn generate_renamed_fn(fn_updated_name: &Ident, fn_tree: &ItemFn) -> TokenStream {
    let fn_vis = &fn_tree.vis;
    let fn_inputs = &fn_tree.sig.inputs;
    let fn_output = &fn_tree.sig.output;
    let fn_block = &fn_tree.block;

    quote! {
        #[doc(hidden)]
        #fn_vis fn #fn_updated_name ( #fn_inputs ) #fn_output #fn_block
    }
}

fn generate_transformed_fn(
    fn_updated_name: &Ident,
    fn_tree: &ItemFn,
    options: BacktrackOptions,
) -> TokenStream {
    let fn_name = &fn_tree.sig.ident;
    let fn_attrs = &fn_tree.attrs;
    let fn_vis = &fn_tree.vis;
    let fn_inputs = &fn_tree.sig.inputs;
    let fn_output = &fn_tree.sig.output;
    let state = options.state;
    let fn_call = utils::fn_call(fn_inputs, fn_updated_name);

    quote! {
        #(#fn_attrs)*
        #fn_vis fn #fn_name ( #fn_inputs ) #fn_output {
            let mut __backtrack_state = #state.clone();
            #fn_call.or_else(|| {
                #state = __backtrack_state;
                None
            })
        }
    }
}

fn generate_transformed_impl(impl_tree: &ItemImpl, methods: Vec<TokenStream>) -> TokenStream {
    let impl_attrs = &impl_tree.attrs;
    let impl_generics = &impl_tree.generics;
    let impl_self_ty = &impl_tree.self_ty;

    quote! {
        #(#impl_attrs)*
        impl #impl_generics #impl_self_ty {
            #(#methods)*
        }
    }
}

fn generate_transformed_impl_method(
    method: &ImplItemFn,
    options: &BacktrackOptions,
) -> TokenStream {
    let method_vis = &method.vis;
    let method_sig = &method.sig;
    let method_block = &method.block;
    let method_attrs = &method
        .attrs
        .iter()
        .filter(|attr| !utils::has_attr(&[attr.to_owned().clone()], "backtrack"))
        .collect::<Vec<_>>();

    let state = &options.state;

    quote! {
        #[backtrack(state = #state)]
        #(#method_attrs)*
        #method_vis #method_sig #method_block
    }
}
