//! This crate provides procedural macros for the `vamo` HTTP client, which is a higher-level
//! abstraction over `deboa`. It includes a derive macro for automatically implementing the `Resource`
//! trait, making it easy to work with RESTful resources.
//!
//! ## Features
//!
//! - **Resource Derive Macro**: Automatically implement RESTful operations for your types
//! - **Attribute-based Configuration**: Configure resource endpoints using attributes
//! - **Type-safe Serialization**: Seamless integration with serde for request/response bodies
//! - **Async Support**: Built for async/await workflows
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deboa = { path = "../deboa" }
//! deboa-extras = { path = "../deboa-extras" }
//! deboa-tokio = { path = "../deboa-tokio" }
//! serde = { version = "1.0", features = ["derive"] }
//! vamo-macros = { path = "../vamo-macros" }
//! vamo = { path = "../vamo" }
//! ```
//!
//! ## Examples
//!
//! ### Basic Resource
//!
//! ```compile_fail
//! use serde::{Deserialize, Serialize};
//! use vamo::Vamo;
//! use vamo_macros::Resource;
//! use deboa::Result;
//! use deboa_extras::http::serde::json::JsonBody;
//! use deboa_tokio::Client;
//!
//! #[derive(Debug, Serialize, Deserialize, Resource)]
//! #[name("posts")]
//! #[body_type(JsonBody)]
//! struct Post {
//!     #[rid]
//!     id: Option<u64>,
//!     title: String,
//!     body: String,
//!     user_id: u64,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut vamo = Vamo::<Client>::new("https://jsonplaceholder.typicode.com")?;
//!
//!     // Create a new post
//!     let new_post = Post {
//!         id: None,
//!         title: "Hello World".into(),
//!         body: "This is a test post".into(),
//!         user_id: 1,
//!     };
//!
//!     let created: Post = vamo.create(&new_post).await?;
//!     println!("Created post with ID: {}", created.id.unwrap());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Available Attributes
//!
//! ### Struct Attributes
//!
//! - `#[name("path")]`: Specify the resource name, rest endpoint (e.g., `posts`, `users`)
//! - `#[body_type(Type)]`: Specify the request/response body type (e.g., `JsonBody`, `XmlBody`)
//!
//! ### Field Attributes
//!
//! - `#[rid]`: Mark a field as the resource identifier (must be `Option<T>` where T is a primitive type)
//!
//! ## Note
//!
//! The `Resource` derive macro automatically implements the following methods:
//! - `new(base_path, vamo)`: Create a new resource client
//! - `id()`: Get the resource identifier
//! - `name()`: Get the resource name
//! - `body_type()`: Get the resource body type

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

pub fn resource(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(data) => data.fields,
        _ => panic!("only structs are supported"),
    };

    let mut rid_field: Option<Ident> = None;
    for field in fields {
        if field
            .attrs
            .iter()
            .any(|attr| {
                attr.path()
                    .is_ident("rid")
            })
        {
            rid_field = field.ident;
            break;
        }
    }

    // Extract literals from attributes
    let mut resource_name: Option<String> = None;

    let mut body_type: Option<Ident> = None;
    for attr in ast.attrs {
        if attr
            .path()
            .is_ident("name")
        {
            resource_name = extract_path(&attr);
        } else if attr
            .path()
            .is_ident("body_type")
        {
            body_type = extract_ident(&attr);
        }
    }

    if resource_name.is_none() {
        panic!("resource name is required");
    }

    if body_type.is_none() {
        panic!("body type is required");
    }

    quote! {
        impl vamo::resource::Resource for #name {
            fn id(&self) -> String {
                self.#rid_field.to_string()
            }

            fn name(&self) -> &str {
                #resource_name
            }

            fn body_type(&self) -> impl RequestBody {
                #body_type
            }
        }
    }
    .into()
}
