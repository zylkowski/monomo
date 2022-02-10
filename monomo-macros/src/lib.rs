use std::fmt::Debug;

use proc_macro::TokenStream;
use quote::*;
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, GenericArgument, Ident, Path, PathArguments,
    Type, TypePath,
};

// struct RphizeInput {
//     attrs: Vec<Attribute>,
//     trait_ident: Ident,
//     generics: Generics,
// }

// impl Parse for RphizeInput {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let attrs = input.call(Attribute::parse_outer)?;
//         let lookahead = input.lookahead1();
//         if lookahead.peek(Ident) {
//             let trait_ident = input.parse::<Ident>()?;
//             let generics = input.parse::<Generics>()?;

//             Ok(RphizeInput {
//                 attrs,
//                 trait_ident,
//                 generics,
//             })
//         } else {
//             Err(lookahead.error())
//         }
//     }
// }

fn flatten_path(path: Path) -> Ident {
    let ty = path.segments.last().expect("No identifier!");
    // let ty_ident = ty.ident;

    if let PathArguments::AngleBracketed(gen_args) = ty.arguments.clone() {
        if gen_args
            .args
            .iter()
            .all(|gen_arg| matches!(gen_arg, GenericArgument::Type(_)))
        {}
    }
    // path.

    todo!();
}

#[proc_macro]
pub fn rphize(tokens: TokenStream) -> TokenStream {
    let parse_input = parse_macro_input!(tokens as TypePath);

    // parse_input.path.get_ident()
    // let attrs = parse_input.attrs;
    let ty = parse_input.path;
    // println!("{:#?}", ty.segments.last().unwrap().);

    quote! {}.into()
    // let ty = ty.ident;
    // if let PathArguments::AngleBracketed(generic_args) = ty.arguments {
    //     let monomorphized = generic_args
    //         .args
    //         .iter()
    //         .fold(format_ident!("_{}", ty), |acc, gen| match gen {
    //             syn::GenericArgument::Type(Type::Path(type_path)) => {
    //                 format_ident!(
    //                     "{}_{}",
    //                     acc,
    //                     type_path
    //                         .path
    //                         .segments
    //                         .last()
    //                         .expect("Invalid generic argument!")
    //                         .ident
    //                 )
    //             }
    //             _ => panic!("Invalid generic param"),
    //         });

    //     quote! {
    //         #[allow(non_camel_case_types)]
    //         // #(#attrs)*
    //         trait #monomorphized : #parse_input {}
    //     }
    //     .into()
    // } else {
    //     panic!("AAAAAAA");
    // }
    // let generics = gen_monomorph.arguments.
    // let (_, ty_generics, where_clause) = parse_input.generics.split_for_impl();

    // println!("Generics: {:?}", parse_input.generics);

    // let monomorphized =
    //     parse_input
    //         .generics
    //         .params
    //         .iter()
    //         .fold(format_ident!("_{}", ty), |acc, gen| match gen {
    //             syn::GenericParam::Type(t) => format_ident!("{}_{}", acc, t.ident),
    //             _ => panic!("Invalid generic param"),
    //         });
}

// #[proc_macro]
// pub fn rphize(tokens: TokenStream) -> TokenStream {
//     let parse_input = parse_macro_input!(tokens as RphizeInput);

//     let attrs = parse_input.attrs;
//     let ty = parse_input.trait_ident;
//     let (_, ty_generics, where_clause) = parse_input.generics.split_for_impl();

//     println!("Generics: {:?}", parse_input.generics);

//     let monomorphized =
//         parse_input
//             .generics
//             .params
//             .iter()
//             .fold(format_ident!("_{}", ty), |acc, gen| match gen {
//                 syn::GenericParam::Type(t) => format_ident!("{}_{}", acc, t.ident),
//                 _ => panic!("Invalid generic param"),
//             });

//     quote! {
//         #[allow(non_camel_case_types)]
//         #(#attrs)*
//         trait #monomorphized : #ty #ty_generics #where_clause {}
//     }
//     .into()
// }

// #[proc_macro]
// pub fn rph(tokens: TokenStream) -> TokenStream {
//     let parse_input = parse_macro_input!(tokens as RphizeInput);

//     let ty = parse_input.trait_ident;

//     let monomorphized =
//         parse_input
//             .generics
//             .params
//             .iter()
//             .fold(format_ident!("_{}", ty), |acc, gen| match gen {
//                 syn::GenericParam::Type(t) => format_ident!("{}_{}", acc, t.ident),
//                 _ => panic!("Invalid generic param"),
//             });

//     quote! {
//         dyn #monomorphized
//     }
//     .into()
// }
