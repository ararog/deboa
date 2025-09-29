extern crate proc_macro;
use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Ident, LitStr};

fn extract_path(attr: &Attribute) -> Option<String> {
    let lit = attr.parse_args::<LitStr>();
    if let Err(e) = lit {
        panic!("failed to parse path: {}", e);
    }
    Some(lit.unwrap().value())
}

fn extract_ident(attr: &Attribute) -> Option<Ident> {
    let ident = attr.parse_args::<Ident>();
    if let Err(e) = ident {
        panic!("failed to parse path: {}", e);
    }
    Some(ident.unwrap())
}

#[proc_macro_derive(Resource, attributes(post, put, patch, delete, body_type))]
pub fn resource(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    // Extract literals from attributes
    let mut post_path: Option<String> = None;
    let mut put_path: Option<String> = None;
    let mut patch_path: Option<String> = None;
    let mut delete_path: Option<String> = None;
    let mut body_type: Option<Ident> = None;
    for attr in ast.attrs {
        if attr.path().is_ident("post") {
            post_path = extract_path(&attr);
        } else if attr.path().is_ident("put") {
            put_path = extract_path(&attr);
        } else if attr.path().is_ident("patch") {
            patch_path = extract_path(&attr);
        } else if attr.path().is_ident("delete") {
            delete_path = extract_path(&attr);
        } else if attr.path().is_ident("body_type") {
            body_type = extract_ident(&attr);
        }
    }

    if post_path.is_none() {
        panic!("post path is required");
    }

    if put_path.is_none() {
        panic!("put path is required");
    }

    if patch_path.is_none() {
        panic!("patch path is required");
    }

    if delete_path.is_none() {
        panic!("delete path is required");
    }

    if body_type.is_none() {
        panic!("body type is required");
    }

    quote! {
        impl vamo::resource::Resource for #name {
            fn id(&self) -> String {
                "1".to_string()
            }

            fn post_path(&self) -> &str {
                #post_path
            }

            fn put_path(&self) -> &str {
                #put_path
            }

            fn patch_path(&self) -> &str {
                #patch_path
            }

            fn delete_path(&self) -> &str {
                #delete_path
            }

            fn body_type(&self) -> impl RequestBody {
                #body_type
            }
        }
    }
    .into()
}
