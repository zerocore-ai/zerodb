use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    ExprClosure, ExprField, Ident, ImplItemFn, ItemFn, ItemImpl, Pat, PatType, Result, ReturnType,
    Token, Type, TypeReference,
};

use crate::utils;

//--------------------------------------------------------------------------------------------------
// Modules
//--------------------------------------------------------------------------------------------------

mod keyword {
    use syn::custom_keyword;

    custom_keyword!(state);
    custom_keyword!(condition);
}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

pub(super) enum BacktrackOption {
    State(ExprField),
    Condition(ExprClosure),
}

pub(super) struct BacktrackOptions {
    state: ExprField,
    condition: ExprClosure,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Parse for BacktrackOption {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(keyword::state) {
            input.parse::<keyword::state>()?;
            input.parse::<Token![=]>()?;
            Ok(Self::State(input.parse()?))
        } else if lookahead.peek(keyword::condition) {
            input.parse::<keyword::condition>()?;
            input.parse::<Token![=]>()?;
            Ok(Self::Condition(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for BacktrackOptions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let options = input.parse_terminated(BacktrackOption::parse, Token![,])?;

        let option_count = options
            .iter()
            .filter(|option| {
                matches!(
                    option,
                    BacktrackOption::State(_) | BacktrackOption::Condition(_)
                )
            })
            .count();

        if option_count != 2 {
            return Err(syn::Error::new(
                input.span(),
                "Expected `state` and `condition` options for the `backtrack` macro",
            ));
        }

        // Construct backtrack options.
        let options = options
            .into_iter()
            .map(|option| match option {
                BacktrackOption::State(state) => (Some(state), None),
                BacktrackOption::Condition(condition) => (None, Some(condition)),
            })
            .fold((None, None), |(c1, k1), (c2, k2)| (c1.or(c2), k1.or(k2)));

        Ok(Self {
            state: options.0.unwrap(),
            condition: options.1.unwrap(),
        })
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
    options: BacktrackOptions,
) -> TokenStream {
    let fn_name = &fn_tree.sig.ident;
    let fn_attrs = &fn_tree.attrs;
    let fn_vis = &fn_tree.vis;
    let fn_inputs = &fn_tree.sig.inputs;
    let fn_output = &fn_tree.sig.output;
    let state = options.state;
    let condition = match add_type_to_closure(options.condition, fn_output) {
        Ok(condition) => condition,
        Err(err) => return err.to_compile_error(),
    };
    let fn_call = utils::fn_call(fn_inputs, fn_updated_name);

    quote! {
        #(#fn_attrs)*
        #fn_vis fn #fn_name ( #fn_inputs ) #fn_output {
            let __backtrack_state = #state.clone();
            let result = #fn_call;
            if (#condition)(&result) {
                #state = __backtrack_state;
            }
            return result;
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
    let condition = &options.condition;

    quote! {
        #[backtrack(state = #state, condition = #condition)]
        #(#method_attrs)*
        #method_vis #method_sig #method_block
    }
}

//--------------------------------------------------------------------------------------------------
// Functions: Helpers
//--------------------------------------------------------------------------------------------------

/// If there is no type annotation on the `condition` closure argument, add it.
fn add_type_to_closure(mut closure: ExprClosure, ty: &ReturnType) -> Result<ExprClosure> {
    if closure.inputs.is_empty() || closure.inputs.len() > 1 {
        return Err(syn::Error::new(
            closure.inputs.span(),
            "Expected a single input argument for the `condition` closure",
        ));
    }

    // Extract the type from the return type.
    let ty = match ty {
        ReturnType::Default => Box::new(syn::Type::Tuple(syn::TypeTuple {
            paren_token: Default::default(),
            elems: Default::default(),
        })),
        ReturnType::Type(_, ty) => ty.to_owned(),
    };

    // Create a reference type.
    let ty = Box::new(Type::Reference(TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: ty,
    }));

    // Construct the argument pattern with the type.
    let pat = closure.inputs[0].clone();
    let pat_type = match pat {
        Pat::Ident(pat_ident) => Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Ident(pat_ident)),
            colon_token: Default::default(),
            ty,
        }),
        Pat::Struct(pat_struct) => Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Struct(pat_struct)),
            colon_token: Default::default(),
            ty,
        }),
        Pat::Tuple(pat_tuple) => Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Tuple(pat_tuple)),
            colon_token: Default::default(),
            ty,
        }),
        Pat::TupleStruct(pat_tuple_struct) => Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::TupleStruct(pat_tuple_struct)),
            colon_token: Default::default(),
            ty,
        }),
        Pat::Wild(pat_wild) => Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Wild(pat_wild)),
            colon_token: Default::default(),
            ty,
        }),
        pat_type @ Pat::Type(_) => pat_type,
        _ => {
            return Err(syn::Error::new(
                pat.span(),
                "Unsupported argument pattern type",
            ))
        }
    };

    // Construct the closure with the type.
    closure.inputs[0] = pat_type;

    Ok(closure)
}
