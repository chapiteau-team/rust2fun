use proc_macro::{TokenStream, TokenTree};

use quote::{format_ident, quote};

#[proc_macro]
pub fn curry_arity(input: TokenStream) -> TokenStream {
    let arity = parse_arity(input);
    let fn_name = format_ident!("curry{}", arity);
    let fn_args = (0..arity)
        .map(|i| format_ident!("x{}", i))
        .collect::<Vec<_>>();
    let msg = format!("Curry a function of {arity} arguments.");

    let expanded = quote! {
        #[doc = #msg]
        #[macro_export]
        macro_rules! #fn_name {
            ($f:expr) => {
                #( move | #fn_args | )* $f( #( #fn_args ),* )
            };
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn constant_arity(input: TokenStream) -> TokenStream {
    let arity = parse_arity(input);
    let fn_name = format_ident!("constant{}", arity);
    let fn_args = (1..arity).map(|_| quote!(_));
    let msg_args = vec!["_"; arity as usize].join(", ");
    let msg = format!(
        "The constant function with {arity} arguments *constant{arity}(x) = ({msg_args}) -> x*."
    );

    let expanded = quote! {
        #[doc = #msg]
        #[macro_export]
        macro_rules! #fn_name {
            ($x:expr) => {
                |_ #( , #fn_args )* | $x
            };
        }
    };

    TokenStream::from(expanded)
}

fn parse_arity(input: TokenStream) -> u32 {
    match input.into_iter().next().expect("arity is required") {
        TokenTree::Literal(x) => x
            .to_string()
            .parse::<u32>()
            .expect("arity must be a number"),
        _ => panic!("arity must be a literal"),
    }
}
