extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Lit, Meta};

#[proc_macro_derive(Resource, attributes(post, put, patch, delete))]
pub fn resource(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    // Extract literals from attributes
    let literal_value: Option<String> = None;
    for attr in ast.attrs {
        if attr.path().is_ident("post") {
            //
        }
    }

    quote! {
        impl Resource for #name {
            fn id(&self) -> String {
                "".to_string()
            }

            fn post_path() -> &str {
                "/"
            }

            fn put_path() -> &str {
                "/"
            }

            fn patch_path() -> &str {
                "/"
            }

            fn delete_path() -> &str {
                "/"
            }

            fn body_type(&self) -> impl RequestBody {

            }
        }
    }
    .into()
}
