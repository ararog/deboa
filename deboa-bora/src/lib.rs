//! # bora - api Documentation
//!
//! Hello, and welcome to the bora API documentation!
//!
//! This API documentation is highly technical and is purely a reference.
//!
//! Depend on `bora` in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! bora = "0.0.1"
//! ```
//!
//! <small>Note that development versions, tagged with `-dev`, are not published
//! and need to be specified as [git dependencies].</small>
//!
//! ```rust,no_run
//! use deboa::errors::DeboaError;
//! use deboa_bora::bora;
//! use vamo::Vamo;
//!
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, Debug)]
//! pub struct Post {
//!     pub id: u32,
//!     pub title: String,
//! }
//!
//! #[bora(
//!     api(
//!         get(name="get_all", path="/posts", res_body=Vec<Post>, format="json"),
//!         get(name="get_by_id", path="/posts/<id:i32>", res_body=Post, format="json"),
//!         get(name="query_by_id", path="/posts?<id:i32>", res_body=Vec<Post>, format="json"),
//!         get(name="query_by_title", path="/posts?<id:i32>&<title:&str>", res_body=Vec<Post>, format="json")
//!     )
//! )]
//! pub struct PostService;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = Vamo::new("https://jsonplaceholder.typicode.com")?;
//!
//!     let mut post_service = PostService::new(client);
//!
//!     let post = post_service.get_by_id(1).await?;
//!
//!     println!("id...: {}", post.id);
//!     println!("title: {}", post.title);
//!
//!     assert_eq!(post.id, 1);
//!     Ok(())
//! }
//! ```
//!
//! Disabled features can be selectively enabled in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! bora = { version = "0.0.1", features = ["tokio_rt", "http1", "http2"] }
//! vamo = { version = "0.0.1" }
//! deboa-extras = { version = "0.0.1" }
//! ```
//!

use proc_macro::TokenStream;

mod bora;
mod parser;
mod token;

use crate::bora::api::bora as bora_macro;

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
