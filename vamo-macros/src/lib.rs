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

#[proc_macro_derive(Resource, attributes(rid, post, put, patch, delete, body_type))]
/// Derive macro for the Resource trait.
///
/// # Attributes
///
/// * `rid` - The id of the resource.
/// * `post` - The post path of the resource.
/// * `put` - The put path of the resource.
/// * `patch` - The patch path of the resource.
/// * `delete` - The delete path of the resource.
/// * `body_type` - The body type of the resource (impl RequestBody from deboa-extras).
///
/// # Example
///
/// ```compile_fail
/// use deboa_extras::http::serde::json::JsonBody;
///
/// #[derive(Resource)]
/// #[post("/posts")]
/// #[put("/posts/{}")]
/// #[patch("/posts/{}")]
/// #[delete("/posts/{}")]
/// #[body_type(JsonBody)]
/// struct MyResource {
///     #[rid("id")]
///     id: String,
/// }
/// ```
pub fn resource(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(data) => data.fields,
        _ => panic!("only structs are supported"),
    };

    let mut rid_field: Option<Ident> = None;
    for field in fields {
        if field.attrs.iter().any(|attr| attr.path().is_ident("rid")) {
            rid_field = field.ident;
            break;
        }
    }

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
        panic!("missing path for post");
    }

    if put_path.is_none() {
        panic!("missing path for put");
    }

    if patch_path.is_none() {
        panic!("missing path for patch");
    }

    if delete_path.is_none() {
        panic!("missing path for delete");
    }

    if body_type.is_none() {
        panic!("body type is required");
    }

    quote! {
        impl vamo::resource::Resource for #name {
            fn id(&self) -> String {
                self.#rid_field.to_string()
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
