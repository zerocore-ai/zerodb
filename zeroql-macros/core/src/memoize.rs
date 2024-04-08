use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    Expr, ExprField, FnArg, Ident, ImplItemFn, ItemFn, ItemImpl, Result, Token,
};

use crate::utils;

//--------------------------------------------------------------------------------------------------
// Modules
//--------------------------------------------------------------------------------------------------

mod keyword {
    use syn::custom_keyword;

    custom_keyword!(cache);
    custom_keyword!(salt);
}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

pub(super) enum MemoizeOption {
    Cache(ExprField),
    Salt(ExprField),
}

pub(super) struct MemoizeOptions {
    cache: ExprField,
    salt: Option<ExprField>,
}

impl Parse for MemoizeOption {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(keyword::cache) {
            input.parse::<keyword::cache>()?;
            input.parse::<Token![=]>()?;
            Ok(Self::Cache(input.parse()?))
        } else if lookahead.peek(keyword::salt) {
            input.parse::<keyword::salt>()?;
            input.parse::<Token![=]>()?;
            Ok(Self::Salt(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Parse for MemoizeOptions {
    fn parse(input: ParseStream) -> Result<Self> {
        let options = input.parse_terminated(MemoizeOption::parse, Token![,])?;

        options
            .iter()
            .find(|option| matches!(option, MemoizeOption::Cache(_)))
            .ok_or_else(|| {
                syn::Error::new(
                    input.span(),
                    "Expected at least one `cache` option for the `memoize` macro",
                )
            })?;

        // Construct memoize options.
        let options = options
            .into_iter()
            .map(|option| match option {
                MemoizeOption::Cache(cache) => (Some(cache), None),
                MemoizeOption::Salt(salt) => (None, Some(salt)),
            })
            .fold((None, None), |(c1, k1), (c2, k2)| (c1.or(c2), k1.or(k2)));

        Ok(Self {
            cache: options.0.unwrap(),
            salt: options.1,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Functions: Entry Point
//--------------------------------------------------------------------------------------------------

pub(super) fn impl_generate(options: &MemoizeOptions, impl_tree: &ItemImpl) -> TokenStream {
    let mut methods = Vec::new();
    for method in impl_tree.items.iter() {
        if let syn::ImplItem::Fn(method) = method {
            if utils::has_attr(&method.attrs, "memoize") {
                methods.push(generate_transformed_impl_method(method, options));
            } else {
                methods.push(quote! { #method });
            }
        }
    }

    generate_transformed_impl(impl_tree, methods)
}

pub(super) fn fn_generate(options: MemoizeOptions, fn_tree: &ItemFn) -> TokenStream {
    let fn_updated_name = &format_ident!("__memoize_original_{}", fn_tree.sig.ident);
    let fn_renamed = generate_renamed_fn(fn_updated_name, fn_tree);
    let fn_transformed = generate_transformed_fn(fn_updated_name, fn_tree, options);

    quote! {
        #fn_renamed
        #fn_transformed
    }
}

//--------------------------------------------------------------------------------------------------
// Functions: Generations
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
    options: MemoizeOptions,
) -> TokenStream {
    let fn_name = &fn_tree.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_attrs = &fn_tree.attrs;
    let fn_vis = &fn_tree.vis;
    let fn_inputs = &fn_tree.sig.inputs;
    let fn_output = &fn_tree.sig.output;
    let fn_arg_names_no_cache = exclude_base_cache_from_args(&fn_tree.sig.inputs, &options.cache)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    let fn_call = utils::fn_call(fn_inputs, fn_updated_name);

    let MemoizeOptions {
        ref cache,
        ref salt,
    } = options;

    let hashable_key = if salt.is_some() {
        quote! { &(#fn_name_str, (#(#fn_arg_names_no_cache),*), #salt) }
    } else {
        quote! { &(#fn_name_str, (#(#fn_arg_names_no_cache),*)) }
    };

    let crate_path = utils::crate_path(fn_name).unwrap();

    quote! {
        #(#fn_attrs)*
        #fn_vis fn #fn_name ( #fn_inputs ) #fn_output {
            use #crate_path::anykey::{self, AnyKey};
            use #crate_path::cache::Cache;

            let hashable_key = #hashable_key;
            let anykey = anykey::into_key(hashable_key.clone());

            if #cache.get(&anykey).is_none() {
                let value = #fn_call;
                let anykey = anykey::into_key(hashable_key.clone());
                #cache.insert(anykey, value);
            }

            #cache.get(&anykey).unwrap().clone()
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

fn generate_transformed_impl_method(method: &ImplItemFn, options: &MemoizeOptions) -> TokenStream {
    let method_vis = &method.vis;
    let method_sig = &method.sig;
    let method_block = &method.block;
    let method_attrs = &method
        .attrs
        .iter()
        .filter(|attr| !utils::has_attr(&[attr.to_owned().clone()], "memoize"))
        .collect::<Vec<_>>();

    let cache = &options.cache;
    let salt = if let Some(salt) = &options.salt {
        quote! { salt = #salt }
    } else {
        quote! {}
    };

    quote! {
        #[memoize(cache = #cache, #salt)]
        #(#method_attrs)*
        #method_vis #method_sig #method_block
    }
}

//--------------------------------------------------------------------------------------------------
// Functions: Helpers
//--------------------------------------------------------------------------------------------------

/// Returns an iterator over the argument names of a function, exculding the cache base name if present.
///
/// Essentially if the cache is set to `self.cache`, then `self` will be excluded from the iterator.
fn exclude_base_cache_from_args<'a>(
    fn_inputs: &'a Punctuated<FnArg, Comma>,
    cache: &'a ExprField,
) -> impl Iterator<Item = Result<Ident>> + 'a {
    utils::get_fn_arg_names(fn_inputs).filter(move |arg| match arg {
        Ok(arg) => match &*cache.base {
            Expr::Path(path) => match path.path.get_ident() {
                Some(ident) => arg != ident,
                None => true,
            },
            _ => true,
        },
        Err(_) => true,
    })
}
