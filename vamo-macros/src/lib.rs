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
//! use vamo::Vamo;
//! use serde::{Deserialize, Serialize};
//! use vamo_macros::Resource;
//! use deboa::Result;
//! use deboa_extras::http::serde::json::JsonBody;
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
//!     let mut vamo = Vamo::new("https://jsonplaceholder.typicode.com")?;
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

use proc_macro::TokenStream;

extern crate proc_macro;

mod bora;
mod resource;

use crate::bora::bora as bora_macro;
use crate::resource::resource as resource_macro;

#[proc_macro_attribute]
///
/// The `bora` attribute macro is used to generate a Deboa client.
/// With this macro you can define the API endpoints and their methods.
/// You can define multiple endpoints and methods in the same macro.
///
/// A basic definition is:
///
/// #[bora(api(operation)))]
///
/// Where 'operation' is one or more of the following:
///
/// - get
/// - post
/// - delete
/// - put
/// - patch
///
/// # get
///
/// The `get` operation is used to retrieve data from the API.
///
/// It has the following arguments:
///
/// - name: The name of the operation.
/// - path: The path of the operation.
/// - res_body: The type of the response body.
/// - format: The format of the response body.
///
/// ## Example
///
/// ```compile_fail
/// #[bora(api(get(name = "get_post", path = "/posts/<id:i32>")))]
/// pub struct PostService;
/// ```
///
/// # post
///
/// The `post` operation is used to create data in the API.
///
/// It has the following arguments:
///
/// - name: The name of the operation.
/// - path: The path of the operation.
/// - req_body: The type of the request body.
/// - res_body: The type of the response body.
/// - format: The format of the response body.
///
/// ## Example
///
/// ```compile_fail
/// #[bora(api(post(name = "post_post", path = "/posts", req_body = "Post", res_body = "Post")))]
/// pub struct PostService;
/// ```
///
/// # delete
///
/// The `delete` operation is used to delete data from the API.
///
/// It has the following arguments:
///
/// - name: The name of the operation.
/// - path: The path of the operation.
///
/// ## Example
///
/// ```compile_fail
/// #[bora(api(delete(name = "delete_post", path = "/posts/<id:i32>")))]
/// pub struct PostService;
/// ```
///
/// # put
///
/// The `put` operation is used to update data in the API.
///
/// It has the following arguments:
///
/// - name: The name of the operation.
/// - path: The path of the operation.
/// - req_body: The type of the request body.
/// - res_body: The type of the response body.
/// - format: The format of the response body.
///
/// ## Example
///
/// ```compile_fail
/// #[bora(api(put(name = "put_post", path = "/posts/<id:i32>", req_body = "Post", res_body = "Post")))]
/// pub struct PostService;
/// ```
///
/// # patch
///
/// The `patch` operation is used to update data in the API.
///
/// It has the following arguments:
///
/// - name: The name of the operation.
/// - path: The path of the operation.
/// - req_body: The type of the request body.
/// - res_body: The type of the response body.
/// - format: The format of the response body.
///
/// ## Example
///
/// ```compile_fail
/// #[bora(api(patch(name = "patch_post", path = "/posts/<id:i32>", req_body = "Post", res_body = "Post")))]
/// pub struct PostService;
/// ```
pub fn bora(attr: TokenStream, item: TokenStream) -> TokenStream {
    bora_macro(attr, item)
}

#[proc_macro_derive(Resource, attributes(rid, name, body_type))]
/// Derive macro for the Resource trait.
///
/// # Attributes
///
/// * `rid` - The id of resource.
/// * `name` - The name of resource.
/// * `body_type` - The body type of resource (impl RequestBody from deboa-extras).
///
/// # Example
///
/// ```compile_fail
/// use deboa_extras::http::serde::json::JsonBody;
///
/// #[derive(Resource)]
/// #[name("posts")]
/// #[body_type(JsonBody)]
/// struct MyResource {
///     #[rid("id")]
///     id: String,
/// }
/// ```
pub fn resource(input: TokenStream) -> TokenStream {
    resource_macro(input)
}
