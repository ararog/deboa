use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_str, punctuated::Punctuated, Pat, PatType, Token, TraitItemFn, Visibility};
use crate::parser::{operations::delete::{DeleteFieldEnum}};
use crate::parser::utils::extract_params_from_path;

pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr with Punctuated<DeleteFieldEnum, Token![,]>::parse_terminated);
    let item = parse_macro_input!(item as TraitItemFn);
    
    let path_fields = attrs.iter().fold(HashMap::new(), |mut acc, field| {
        if let DeleteFieldEnum::path(path) = field {
            let params = extract_params_from_path(&path.value.value());
            for param in params {
                acc.insert(param.0, param.1);
            }
        }
        acc
    }); 

    if item.sig.asyncness.is_none() {
        panic!("expected to be an async function");
    }

    item.sig.inputs.iter().for_each(|input| {
        if let syn::FnArg::Typed(typed) = input {
            let PatType { attrs: _, pat, colon_token: _, ty } = typed;
            if let Pat::Ident(ident) = &**pat {
                if ! path_fields.contains_key(&ident.ident.to_string()) {
                    panic!("expected to have a parameter named {}", ident.ident);
                }

                let param_type = path_fields.get(&ident.ident.to_string()).unwrap();
                if *ty.as_ref() != parse_str::<syn::Type>(param_type).unwrap() {
                    panic!("expected type {param_type}");
                }
            }
        }
    });

    item.to_token_stream().into()
}
