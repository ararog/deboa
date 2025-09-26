use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use syn::{parse_macro_input, ImplItemFn, Pat, PatType, Visibility};
use crate::token::utils::extract_params_from_path;
use crate::parser::{operations::delete::{DeleteFieldEnum, DeleteStruct}};

pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ImplItemFn);
    let attr = TS2::from(attr);

    let parse_attr = attr.clone().into();
    let root = parse_macro_input!(parse_attr as DeleteStruct);
    let _path_field = root.fields.iter().fold(HashMap::new(), |mut acc, field| {
        if let DeleteFieldEnum::path(path) = field {
            let (api_params, _) = extract_params_from_path(&path.value);
            acc.insert(path.value.value(), api_params);
        }
        acc
    }); 

    let ImplItemFn { attrs: _, vis, defaultness: _, sig, block: _ } = item;
    
    if sig.asyncness.is_none() {
        panic!("expected to be an async function");
    }

    if ! matches!(vis, Visibility::Public(_)) {
        panic!("expected to be a public function");
    }

    sig.inputs.iter().for_each(|input| {
        if let syn::FnArg::Typed(typed) = input {
            let PatType { attrs: _, pat, colon_token: _, ty: _ } = typed;
            if let Pat::Ident(_) = &**pat {
                
            }
        }
    });

    TokenStream::new()
}
