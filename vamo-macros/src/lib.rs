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
