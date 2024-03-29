use proc_macro::{TokenStream, TokenTree};

use proc_macro2::Ident;
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
    let fn_args = (0..arity).map(|_| quote!(_));
    let msg_args = vec!["_"; arity as usize].join(", ");
    let msg = format!(
        "The constant function with {arity} arguments *constant{arity}(x) = ({msg_args}) -> x*."
    );

    let expanded = quote! {
        #[doc = #msg]
        #[macro_export]
        macro_rules! #fn_name {
            ($x:expr) => {
                | #( #fn_args ),* | $x
            };
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn tuple_arity(input: TokenStream) -> TokenStream {
    let arity = parse_arity(input);
    let fn_name = format_ident!("tuple{}", arity);
    let types = ('A'..='Z').take(arity as usize);
    let type_args = types.clone().map(|t| format_ident!("{}", t));
    let args = types.map(|x| format_ident!("{}", x.to_lowercase().next().unwrap()));
    let fn_args = args
        .clone()
        .zip(type_args.clone())
        .map(|(a, t)| quote!(#a: #t));
    let return_type = type_args.clone();
    let msg = format!("Create a tuple of {arity} elements.");

    let expanded = quote! {
        #[doc = #msg]
        #[inline]
        pub const fn #fn_name< #( #type_args ),* >( #( #fn_args ),* ) -> ( #( #return_type ),* ) {
            ( #( #args ),* )
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn noop_arity(input: TokenStream) -> TokenStream {
    let arity = parse_arity(input);
    let fn_name = format_ident!("noop{}", arity);
    let type_args = ('A'..='Z')
        .take(arity as usize)
        .map(|t| format_ident!("{}", t));
    let fn_args = type_args.clone().map(|t| quote!(_: #t));
    let msg = format!("The no operation function of {arity} arguments.");

    let expanded = quote! {
        #[doc = #msg]
        #[inline(always)]
        pub fn #fn_name< #( #type_args ),* >( #( #fn_args ),*) {}
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn ap_n(input: TokenStream) -> TokenStream {
    let arity = parse_arity(input);
    let fn_name = format_ident!("ap{}", arity);
    let msg = format!("Is a version of [Apply::ap] for a function of {arity} arguments.");

    let types = ('A'..='Y').take(arity as usize);
    let generic_type_args = types.clone().map(|t| format_ident!("{}", t));
    let fn_args = types.clone().map(to_fn_arg);
    let constraints = {
        let (f, mut c) = generic_type_args
            .clone()
            .skip(1)
            .fold((quote!(A), Vec::new()), generate_semigroupal_constraint);

        c.push(quote!(Self::Target< #f >: Functor<Z, Target<Z> = Self::Target<Z>> + Clone));
        c
    };
    let fn_types = types.clone().map(|t| format_ident!("{}", t));
    let f_args = types
        .clone()
        .map(|t| format_ident!("{}", t.to_lowercase().next().unwrap()));

    let products = types.clone().skip(1).map(|t| {
        let a = format_ident!("f{}", t.to_lowercase().next().unwrap());
        quote!(product(#a))
    });
    let map_pattern = types
        .skip(1)
        .map(|t| format_ident!("{}", t.to_lowercase().next().unwrap()))
        .fold(quote!(a), |acc, a| quote!((#acc, #a)));

    let expanded = quote! {
        #[doc = #msg]
        #[inline]
        fn #fn_name<  #( #generic_type_args ),* >( self, #( #fn_args ),*) -> Self::Target<Z>
        where
            Self::Param: FnMut( #( #fn_types ),* ) -> Z,
            Self: Sized,
            #( #constraints ),*
        {
            let product = fa. #( #products ).*;
            self.and_then(|mut func| product.clone().map(| #map_pattern | func( #( #f_args ),* )))
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn map_n(input: TokenStream) -> TokenStream {
    let arity = parse_arity(input);
    let fn_name = format_ident!("map{}", arity);
    let msg = format!("Is a version of [Apply::map2] for a function of {arity} arguments.");

    let types = ('B'..'Z').take((arity - 1) as usize);
    let generic_type_args = types.clone().skip(1).map(|t| format_ident!("{}", t));
    let fn_args = types.clone().map(to_fn_arg);
    let constraints = {
        let (f, mut c) = generic_type_args.clone().fold(
            (quote!((Self::Param, B)), Vec::new()),
            generate_semigroupal_constraint,
        );

        c.push(quote!(Self::Target< #f >: Functor<Z, Target<Z> = Self::Target<Z>>));
        c
    };
    let fn_types = types.clone().map(|t| format_ident!("{}", t));
    let f_args = types
        .clone()
        .map(|t| format_ident!("{}", t.to_lowercase().next().unwrap()));

    let products = types.clone().map(|t| {
        let a = format_ident!("f{}", t.to_lowercase().next().unwrap());
        quote!(product(#a))
    });
    let map_pattern = types
        .map(|t| format_ident!("{}", t.to_lowercase().next().unwrap()))
        .fold(quote!(a), |acc, a| quote!((#acc, #a)));

    let expanded = quote! {
        #[doc = #msg]
        #[inline]
        fn #fn_name<  #( #generic_type_args ),* , Z, FN >( self, #( #fn_args ),* , mut func: FN) -> Self::Target<Z>
        where
            FN: FnMut( Self::Param, #( #fn_types ),* ) -> Z,
            Self: Sized,
            #( #constraints ),*
        {
            self. #( #products ).* .map(| #map_pattern | func(a, #( #f_args ),* ))
        }
    };

    TokenStream::from(expanded)
}

fn to_fn_arg(t: char) -> proc_macro2::TokenStream {
    let a = format_ident!("f{}", t.to_lowercase().next().unwrap());
    let t = format_ident!("{}", t);
    quote!(#a: Self::Target<#t>)
}

fn generate_semigroupal_constraint(
    acc: (proc_macro2::TokenStream, Vec<proc_macro2::TokenStream>),
    t: Ident,
) -> (proc_macro2::TokenStream, Vec<proc_macro2::TokenStream>) {
    let (curr, mut acc) = acc;
    let next = quote!((#curr, #t));
    let gen = quote! {
        Self::Target< #curr >: Semigroupal< #t ,  Target< #t > = Self::Target< #t >>
            + Higher<Target< #next > = Self::Target< #next >>
    };
    acc.push(gen);
    (next, acc)
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
