#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
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
/// use deboa_tokio::Client;
/// use serde::Deserialize;
/// use vamo::Vamo;
/// use vamo_macros::bora;
///
/// #[bora(api(get(name = "get_post", path = "/posts/<id:i32>")))]
/// pub struct PostService;
///
/// #[derive(Deserialize)]
/// struct Post {
///     id: u32,
///     title: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Vamo::<deboa_tokio::Client>::new("https://jsonplaceholder.typicode.com")?;
///     let mut post_service = PostService::new(client);
///     let post = post_service.get_post(1).await?;
///     println!("Post: {:?}", post);
///     Ok(())
/// }
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
/// use deboa_tokio::Client;
/// use serde::Deserialize;
/// use vamo::Vamo;
/// use vamo_macros::bora;
///
/// #[bora(api(post(name = "create_post", path = "/posts", req_body = "Post", res_body = "Post")))]
/// pub struct PostService;
///
/// #[derive(Deserialize)]
/// struct Post {
///     id: u32,
///     title: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {  
///     let post = Post {
///         id: 1,
///         title: "Post 1".to_string(),
///     };
///     
///     let client = Vamo::<deboa_tokio::Client>::new("https://jsonplaceholder.typicode.com")?;
///     let mut post_service = PostService::new(client);
///     let created_post = post_service.create_post(post).await?;
///     println!("Created post: {:?}", created_post);
///     Ok(())
/// }
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
/// use deboa_tokio::Client;
/// use vamo::Vamo;
/// use vamo_macros::bora;
///
/// #[bora(api(delete(name = "delete_post", path = "/posts/<id:i32>")))]
/// pub struct PostService;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Vamo::<Client>::new("https://jsonplaceholder.typicode.com")?;
///     let mut post_service = PostService::new(client);
///     post_service.delete_post(1).await?;
///     println!("Post deleted");
///     Ok(())
/// }
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
/// use deboa_tokio::Client;
/// use serde::Deserialize;
/// use vamo::Vamo;
/// use vamo_macros::bora;
///  
/// #[bora(api(put(name = "put_post", path = "/posts/<id:i32>", req_body = "Post", res_body = "Post")))]
/// pub struct PostService;
///
/// #[derive(Debug, Deserialize)]
/// pub struct Post {
///     pub id: u32,
///     pub title: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Vamo::<Client>::new("https://jsonplaceholder.typicode.com")?;
///     let mut post_service = PostService::new(client);
///     let post = post_service.put_post(1, Post { id: 1, title: "Post 1".to_string() }).await?;
///     println!("Post updated: {:?}", post);
///     Ok(())
/// }
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
/// use deboa_tokio::Client;
/// use serde::Deserialize;
/// use vamo::Vamo;
/// use vamo_macros::bora;
///
/// #[bora(api(patch(name = "patch_post", path = "/posts/<id:i32>", req_body = "Post", res_body = "Post")))]
/// pub struct PostService;
///
/// #[derive(Debug, Deserialize)]
/// pub struct Post {
///     pub id: u32,
///     pub title: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Vamo::<deboa_tokio::Client>::new("https://jsonplaceholder.typicode.com")?;
///     let mut post_service = PostService::new(client);
///     let post = post_service.patch_post(1, Post { id: 1, title: "Post 1".to_string() }).await?;
///     println!("Post updated: {:?}", post);
///     Ok(())
/// }
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
/// # Resource methods
///
/// The following methods are generated:
///
/// - `load`
/// - `create`
/// - `edit`
/// - `remove`
///
/// # Example
///
/// ```compile_fail
/// use deboa_tokio::Client;
/// use deboa_extras::http::serde::json::JsonBody;
/// use vamo_macros::Resource;
/// use vamo::{Vamo, ResourceMethod};
///
/// #[derive(Resource)]
/// #[name("posts")]
/// #[body_type(JsonBody)]
/// struct MyResource {
///     #[rid("id")]
///     id: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut vamo = Vamo::<Client>::new("https://api.example.com")?;
///
///     // post
///     let response = vamo
///         .create(user)
///         .await?
///         .send()
///         .await?;
///     Ok(())
/// }
/// ```
pub fn resource(input: TokenStream) -> TokenStream {
    resource_macro(input)
}
