//! # Vamo Macros
//!
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
//! vamo-macros = { path = "../vamo-macros" }
//! vamo = { path = "../vamo" }
//! deboa-extras = { path = "../deboa-extras" }
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! ## Examples
//!
//! ### Basic Resource
//!
//! ```compile_fail
//! use serde::{Deserialize, Serialize};
//! use vamo_macros::Resource;
//! use deboa_extras::http::serde::json::JsonBody;
//!
//! #[derive(Debug, Serialize, Deserialize, Resource)]
//! #[get("/posts/:id")]
//! #[post("/posts")]
//! #[put("/posts/:id")]
//! #[patch("/posts/:id")]
//! #[delete("/posts/:id")]
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
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut vamo = vamo::Vamo::new("https://jsonplaceholder.typicode.com")?;
//!     let posts = Post::new("/posts", &mut vamo);
//!
//!     // Create a new post
//!     let new_post = Post {
//!         id: None,
//!         title: "Hello World".into(),
//!         body: "This is a test post".into(),
//!         user_id: 1,
//!     };
//!     let created: Post = posts.create(&new_post).await?;
//!     println!("Created post with ID: {}", created.id.unwrap());
//!
//!     // Get all posts
//!     let all_posts: Vec<Post> = posts.list().await?;
//!     println!("Total posts: {}", all_posts.len());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Available Attributes
//!
//! ### Struct Attributes
//!
//! - `#[post("path")]`: Specify the POST endpoint for creating resources
//! - `#[put("path")]`: Specify the PUT endpoint for updating resources
//! - `#[patch("path")]`: Specify the PATCH endpoint for partial updates
//! - `#[delete("path")]`: Specify the DELETE endpoint for removing resources
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
//! - `list(&self)`: List all resources
//! - `get(&self, id)`: Get a specific resource by ID
//! - `create(&self, item)`: Create a new resource
//! - `update(&self, id, item)`: Update a resource (full update)
//! - `patch(&self, id, item)`: Partially update a resource
//! - `delete(&self, id)`: Delete a resource

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

#[proc_macro_derive(Resource, attributes(rid, get, post, put, patch, delete, body_type))]
/// Derive macro for the Resource trait.
///
/// # Attributes
///
/// * `rid` - The id of the resource.
/// * `get` - The get path of the resource.
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
/// #[get("/posts/:id")]
/// #[post("/posts")]
/// #[put("/posts/:id")]
/// #[patch("/posts/:id")]
/// #[delete("/posts/:id")]
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
    let mut get_path: Option<String> = None;
    let mut post_path: Option<String> = None;
    let mut put_path: Option<String> = None;
    let mut patch_path: Option<String> = None;
    let mut delete_path: Option<String> = None;
    let mut body_type: Option<Ident> = None;
    for attr in ast.attrs {
        if attr.path().is_ident("get") {
            get_path = extract_path(&attr);
        } else if attr.path().is_ident("post") {
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

    if get_path.is_none() {
        panic!("missing path for get");
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

            fn get_path(&self) -> &str {
                #get_path
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
