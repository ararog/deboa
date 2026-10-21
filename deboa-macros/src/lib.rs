#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::actions::{
    delete::DeleteArgs, get::GetArgs, patch::PatchArgs, post::PostArgs, put::PutArgs,
    query::QueryArgs, submit::SubmitArgs,
};
use proc_macro::TokenStream as OldTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse_macro_input;

mod actions;

#[proc_macro]
/// Make a GET request to the specified URL.
///
/// The `get!` macro is used to make a GET request to the specified URL.
///
/// You can use the `JsonBody`, `XmlBody`, `MsgPack` type for JSON, XML
/// and MessagePack serialization.
///
/// get!(
///     url=> url,
///     headers => vec![("User-Agent", "deboa")],
///     client => &client,
///     res_body_ty => JsonBody,
///     res_ty => Article
/// )
///
/// # Arguments
///
/// * `url`         - URL to make the GET request to.
/// * `client`      - Client variable to use for the request.
/// * `res_body_ty` - Body type of the response.
/// * `res_ty`      - Type of the response.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, ignore
/// use deboa_tokio::Client;
/// use deboa_macros::get;
/// use deboa_extras::serde::json::JsonBody;
///
/// #[derive(serde::Deserialize)]
/// struct Post {
///     id: u32,
///     title: String,
///     body: String,
///     userId: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut client = Client::default();
///     let response = get!(
///         url => "https://jsonplaceholder.typicode.com/posts",
///         client => &client,
///         res_body_ty => JsonBody,
///         res_ty => Vec<Post>
///     );
///     assert_eq!(response.len(), 100);
///     Ok(())
/// }
/// ```
pub fn get(item: OldTokenStream) -> OldTokenStream {
    let args = parse_macro_input!(item as actions::get::GetArgs);
    expanded_get(args).into()
}

fn expanded_get(args: GetArgs) -> TokenStream {
    let url = match args.url {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'url'")
                .to_compile_error()
                .into();
        }
    };
    let client = match args.client {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'client'")
                .to_compile_error()
                .into();
        }
    };

    let headers = match args.headers {
        Some(headers) => quote! { .headers(#headers) },
        None => TokenStream::new(),
    };

    let body = match (args.res_body_ty, args.res_ty) {
        (Some(res_body_ty), Some(res_ty)) => {
            quote! { .body_as::<#res_body_ty, #res_ty>(#res_body_ty).await? }
        }
        (Some(_), None) => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'res_ty'")
                .to_compile_error()
                .into();
        }
        _ => TokenStream::new(),
    };

    let expanded = quote! {
        deboa::HttpClient::execute(
            #client,
            deboa::request::DeboaRequest::get(#url)?
                #headers
                .build()?,
        )
        .await?
        #body
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Make a QUERY request to the specified URL.
///
/// The `query!` macro is used to make a QUERY request to the specified URL.
///
/// query!(
///     data => data,
///     req_body_ty => XmlBody,
///     url => "http://example.com",
///     client => &client
/// );
///
/// # Arguments
///
/// * `input`       - The input to send with the request.
/// * `req_body_ty` - The body serialization format of the request.
/// * `url`         - The URL to make the QUERY request to.
/// * `headers`     - The headers to send with the request.
/// * `client`      - The client variable to use for the request.
/// * `res_body_ty` - The body serialization format of the response.
/// * `res_ty`      - The type of the response.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ## Without response body deserialization
///
/// ```rust, ignore
/// use deboa_macros::query;
/// use deboa_extras::serde::json::JsonBody;
/// use deboa_tokio::TokioClient;
///
/// #[derive(serde::Serialize)]
/// struct Search {
///     title: String,
///     body: String,
///     userId: u32,
/// }
///
/// #[derive(serde::Deserialize)]
/// struct Post {
///     id: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TokioClient::default();
///     let data = Search {
///         title: "foo".to_string(),
///         body: "bar".to_string(),
///         userId: 1,
///     };
///     let response = query!(
///         data => data,
///         req_body_ty => JsonBody,
///         url => "https://jsonplaceholder.typicode.com/posts",
///         headers => vec!(("Content-Type", "application/json")),
///         client => &client
///     );
///     Ok(())
/// }
/// ```
///
/// ## With response body deserialization
///
/// ```rust, ignore
/// use deboa_macros::post;
/// use deboa_extras::serde::json::JsonBody;
/// use deboa_tokio::TokioClient;
///
/// #[derive(serde::Serialize)]
/// struct Post {
///     title: String,
///     body: String,
///     userId: u32,
/// }
///
/// #[derive(serde::Deserialize)]
/// struct CreatedPost {
///     id: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TokioClient::default();
///     let data = Post {
///         title: "foo".to_string(),
///         body: "bar".to_string(),
///         userId: 1,
/// };
///     let response = post!(
///         data => data,
///         req_body_ty => JsonBody,
///         url => "https://jsonplaceholder.typicode.com/posts",
///         client => &client,
///         res_body_ty => JsonBody,
///         res_ty => CreatedPost
///     );
///     assert_eq!(response.id, 1);
///     Ok(())
/// }
/// ```
pub fn query(item: OldTokenStream) -> OldTokenStream {
    let args = parse_macro_input!(item as actions::query::QueryArgs);
    expanded_query(args).into()
}

fn expanded_query(args: QueryArgs) -> TokenStream {
    let url = match args.url {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'url'")
                .to_compile_error()
                .into();
        }
    };
    let client = match args.client {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'client'")
                .to_compile_error()
                .into();
        }
    };

    let headers = match args.headers {
        Some(headers) => quote! { .headers(#headers) },
        None => TokenStream::new(),
    };

    let req_body = match (args.req_body_ty, args.data) {
        (Some(req_body_ty), Some(data)) => quote! { .body_as(#req_body_ty, #data)? },
        (Some(_), None) => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'data'")
                .to_compile_error()
                .into();
        }
        _ => TokenStream::new(),
    };

    let res_body = match (args.res_body_ty, args.res_ty) {
        (Some(res_body_ty), Some(res_ty)) => {
            quote! { .body_as::<#res_body_ty, #res_ty>(#res_body_ty).await? }
        }
        (Some(_), None) => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'res_ty'")
                .to_compile_error()
                .into();
        }
        _ => TokenStream::new(),
    };

    // Generate the final token stream
    let expanded = quote! {
        deboa::HttpClient::execute(
            #client,
            deboa::request::DeboaRequest::query(#url)?
                #headers
                #req_body
                .build()?,
        )
        .await?
        #res_body
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Make a POST request to the specified URL.
///
/// The `post!` macro is used to make a POST request to the specified URL.
///
/// post!(
///     data => data,
///     req_body_ty => JsonBody,
///     url => "http://example.com",
///     headers => headers,
///     client => &client,
///     res_body_ty => JsonBody,
///     res_ty => Post
/// );
///
/// # Arguments
///
/// * `data`        - Data to send with the request.
/// * `req_body_ty` - Body serialization format of the request.
/// * `url`         - URL to make the POST request to.
/// * `headers`     - Headers to send with the request.
/// * `client`      - Client variable to use for the request.
/// * `res_body_ty` - Body serialization format of the response.
/// * `res_ty`      - Type of the response.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ## Without response body deserialization
///
/// ```rust, ignore
/// use deboa_macros::post;
/// use deboa_extras::serde::json::JsonBody;
/// use deboa_tokio::TokioClient;
///
/// #[derive(serde::Serialize)]
/// struct Post {
///     title: String,
///     body: String,
///     userId: u32,
/// }
///
/// #[derive(serde::Deserialize)]
/// struct CreatedPost {
///     id: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TokioClient::default();
///     let data = Post {
///         title: "foo".to_string(),
///         body: "bar".to_string(),
///         userId: 1,
///     };
///     let response = post!(
///         data => data,
///         req_body_ty => JsonBody,
///         url => "https://jsonplaceholder.typicode.com/posts",
///         headers => vec!(("Content-Type", "application/json")),
///         client => &client
///     );
///     Ok(())
/// }
/// ```
///
/// ## With response body deserialization
///
/// ```rust, ignore
/// use deboa_macros::post;
/// use deboa_extras::serde::json::JsonBody;
/// use deboa_tokio::TokioClient;
///
/// #[derive(serde::Serialize)]
/// struct Post {
///     title: String,
///     body: String,
///     userId: u32,
/// }
///
/// #[derive(serde::Deserialize)]
/// struct CreatedPost {
///     id: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TokioClient::default();
///     let data = Post {
///         title: "foo".to_string(),
///         body: "bar".to_string(),
///         userId: 1,
///     };
///     let response = post!(
///         data => data,
///         req_body_ty => JsonBody,
///         url => "https://jsonplaceholder.typicode.com/posts",
///         client => &client,
///         res_body_ty => JsonBody,
///         res_ty => CreatedPost
///     );
///     assert_eq!(response.id, 1);
///     Ok(())
/// }
/// ```
pub fn post(item: OldTokenStream) -> OldTokenStream {
    let args = parse_macro_input!(item as actions::post::PostArgs);
    expanded_post(args).into()
}

fn expanded_post(args: PostArgs) -> TokenStream {
    let url = match args.url {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'url'")
                .to_compile_error()
                .into();
        }
    };
    let client = match args.client {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'client'")
                .to_compile_error()
                .into();
        }
    };

    let headers = match args.headers {
        Some(headers) => quote! { .headers(#headers) },
        None => TokenStream::new(),
    };

    let req_body = match (args.req_body_ty, args.data) {
        (Some(req_body_ty), Some(data)) => quote! { .body_as(#req_body_ty, #data)? },
        (Some(_), None) => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'data'")
                .to_compile_error()
                .into();
        }
        _ => TokenStream::new(),
    };

    let res_body = match (args.res_body_ty, args.res_ty) {
        (Some(res_body_ty), Some(res_ty)) => {
            quote! { .body_as::<#res_body_ty, #res_ty>(#res_body_ty).await? }
        }
        (Some(_), None) => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'res_ty'")
                .to_compile_error()
                .into();
        }
        _ => TokenStream::new(),
    };

    // Generate the final token stream
    let expanded = quote! {
        deboa::HttpClient::execute(
            #client,
            deboa::request::DeboaRequest::post(#url)?
                #headers
                #req_body
                .build()?,
        )
        .await?
        #res_body
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Make a PUT request to the specified URL.
///
/// put!(
///     data => data,
///     req_body_ty => JsonBody,
///     url => "http://example.com",
///     client => &mut client
/// );
///
/// # Arguments
///
/// * `data`        - Data to send with request.
/// * `req_body_ty` - Body serialization format of request.
/// * `url`         - URL to make the PUT request to.
/// * `headers`     - Headers to send with request.
/// * `client`      - Client variable to use for request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, ignore
/// use deboa_extras::serde::json::JsonBody;
/// use deboa_macros::put;
/// use deboa_tokio::TokioClient;
///
/// #[derive(serde::Serialize)]
/// struct Post {
///     title: String,
///     body: String,
///     userId: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TokioClient::default();
///     let data = Post {
///         title: "foo".to_string(),
///         body: "bar".to_string(),
///         userId: 1,
///     };
///     put!(
///         data => data,
///         req_body_ty => JsonBody,
///         url => "https://jsonplaceholder.typicode.com/posts/1",
///         client => &client
///     );
///     Ok(())
/// }
/// ```
pub fn put(item: OldTokenStream) -> OldTokenStream {
    let args = parse_macro_input!(item as actions::put::PutArgs);
    expanded_put(args).into()
}

fn expanded_put(args: PutArgs) -> TokenStream {
    let url = match args.url {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'url'")
                .to_compile_error()
                .into();
        }
    };
    let client = match args.client {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'client'")
                .to_compile_error()
                .into();
        }
    };

    let headers = match args.headers {
        Some(headers) => quote! { .headers(#headers) },
        None => TokenStream::new(),
    };

    let req_body = match (args.req_body_ty, args.data) {
        (Some(req_body_ty), Some(data)) => quote! { .body_as(#req_body_ty, #data)? },
        (Some(_), None) => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'data'")
                .to_compile_error()
                .into();
        }
        _ => TokenStream::new(),
    };

    let res_body = match (args.res_body_ty, args.res_ty) {
        (Some(res_body_ty), Some(res_ty)) => {
            quote! { .body_as::<#res_body_ty, #res_ty>(#res_body_ty).await? }
        }
        (Some(_), None) => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'res_ty'")
                .to_compile_error()
                .into();
        }
        _ => TokenStream::new(),
    };

    // Generate the final token stream
    let expanded = quote! {
        deboa::HttpClient::execute(
            #client,
            deboa::request::DeboaRequest::put(#url)?
                #headers
                #req_body
                .build()?,
        )
        .await?
        #res_body
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Make a PATCH request to the specified URL.
///
/// patch!(
///     data => data,
///     req_body_ty => JsonBody,
///     url => "http://example.com",
///     client => &mut client
/// );
///
/// # Arguments
///
/// * `data`        - Data to send with request.
/// * `req_body_ty` - Body serialization format of request.
/// * `url`         - URL to make the PATCH request to.
/// * `headers`     - Headers to send with request.
/// * `client`      - Client variable to use for request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, ignore
/// use deboa_extras::serde::json::JsonBody;
/// use deboa_macros::patch;
/// use deboa_tokio::TokioClient;
///
/// #[derive(serde::Serialize)]
/// struct Post {
///     title: String,
///     body: String,
///     userId: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::default();
///     let data = Post {
///         title: "foo".to_string(),
///         body: "bar".to_string(),
///         userId: 1,
///     };
///     patch!(
///         data => data,
///         req_body_ty => JsonBody,
///         url => "https://jsonplaceholder.typicode.com/posts/1",
///         client => &client
///     );
///     Ok(())
/// }
/// ```
pub fn patch(item: OldTokenStream) -> OldTokenStream {
    let args = parse_macro_input!(item as actions::patch::PatchArgs);
    expanded_patch(args).into()
}

fn expanded_patch(args: PatchArgs) -> TokenStream {
    let url = match args.url {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'url'")
                .to_compile_error()
                .into();
        }
    };
    let client = match args.client {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'client'")
                .to_compile_error()
                .into();
        }
    };

    let headers = match args.headers {
        Some(headers) => quote! { .headers(#headers) },
        None => TokenStream::new(),
    };

    let req_body = match (args.req_body_ty, args.data) {
        (Some(req_body_ty), Some(data)) => quote! { .body_as(#req_body_ty, #data)? },
        (Some(_), None) => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'data'")
                .to_compile_error()
                .into();
        }
        _ => TokenStream::new(),
    };

    let res_body = match (args.res_body_ty, args.res_ty) {
        (Some(res_body_ty), Some(res_ty)) => {
            quote! { .body_as::<#res_body_ty, #res_ty>(#res_body_ty).await? }
        }
        (Some(_), None) => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'res_ty'")
                .to_compile_error()
                .into();
        }
        _ => TokenStream::new(),
    };

    // Generate the final token stream
    let expanded = quote! {
        deboa::HttpClient::execute(
            #client,
            deboa::request::DeboaRequest::patch(#url)?
                #headers
                #req_body
                .build()?,
        )
        .await?
        #res_body
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Make a DELETE request to the specified URL.
///
/// delete!(
///     url => "http://example.com",
///     client => &mut client
/// );
///
/// # Arguments
///
/// * `url`     - URL to make the DELETE request to.
/// * `headers` - Headers to send with request.
/// * `client`  - Client variable to use for request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, ignore
/// use deboa_macros::delete;
/// use deboa_tokio::TokioClient;
///
/// #[derive(serde::Deserialize)]
/// struct Post {
///     id: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TokioClient::default();
///     let response = delete!(url => "https://jsonplaceholder.typicode.com/posts/1", client => &client);
///     Ok(())
/// }
/// ```
pub fn delete(item: OldTokenStream) -> OldTokenStream {
    let args = parse_macro_input!(item as actions::delete::DeleteArgs);
    expanded_delete(args).into()
}

fn expanded_delete(args: DeleteArgs) -> TokenStream {
    // Enforce required fields
    let url = match args.url {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'url'")
                .to_compile_error()
                .into();
        }
    };
    let client = match args.client {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'client'")
                .to_compile_error()
                .into();
        }
    };

    let headers = match args.headers {
        Some(headers) => quote! { .headers(#headers) },
        None => TokenStream::new(),
    };

    // Generate the final token stream
    let expanded = quote! {
        deboa::HttpClient::execute(
            #client,
            deboa::request::DeboaRequest::delete(#url)?
                #headers
                .build()?,
        )
        .await?
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Make a GET request to the specified URL.
///
/// The `fetch!` macro is a more generic version of the `get!` macro.
///
/// You can use the `JsonBody`, `XmlBody`, `MsgPack` type for JSON, XML
/// and MessagePack serialization.
///
/// fetch!(
///     url => "http://example.com",
///     client => &mut client,
///     res_body_ty => JsonBody,
///     res_ty => User
/// );
///
/// # Arguments
///
/// * `url`         - URL to make the GET request to.
/// * `headers`     - Headers to send with request.
/// * `client`      - Client variable to use request.
/// * `res_body_ty` - Body serialization format of response.
/// * `res_ty`      - Type of response.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, ignore
/// use deboa_extras::serde::json::JsonBody;
/// use deboa_macros::fetch;
/// use deboa_tokio::TokioClient;
///
/// #[derive(serde::Deserialize)]
/// struct Post {
///     id: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TokioClient::default();
///     let response = fetch!(
///         url => "https://jsonplaceholder.typicode.com/posts",
///         client => &client,
///         res_body_ty => JsonBody,
///         res_ty => Post
///     );
///     assert_eq!(response.id, 1);
///     Ok(())
/// }
/// ```
pub fn fetch(item: OldTokenStream) -> OldTokenStream {
    let args = parse_macro_input!(item as actions::get::GetArgs);
    expanded_fetch(args).into()
}

fn expanded_fetch(args: GetArgs) -> TokenStream {
    // Enforce required fields
    let url = match args.url {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'url'")
                .to_compile_error()
                .into();
        }
    };
    let client = match args.client {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'client'")
                .to_compile_error()
                .into();
        }
    };

    let headers = match args.headers {
        Some(headers) => quote! { .headers(#headers) },
        None => TokenStream::new(),
    };

    let body = match (args.res_body_ty, args.res_ty) {
        (Some(res_body_ty), Some(res_ty)) => {
            quote! { .body_as::<#res_body_ty, #res_ty>(#res_body_ty).await? }
        }
        (Some(_), None) => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'res_ty'")
                .to_compile_error()
                .into();
        }
        _ => TokenStream::new(),
    };

    // Generate the final token stream
    let expanded = quote! {
        deboa::HttpClient::execute(
            #client,
            deboa::request::DeboaRequest::get(#url)?
                #headers
                .build()?,
        )
        .await?
        #body
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Submit a request to the specified URL.
///
/// The `submit!` macro is a more generic version of the `post!` macro.
///
/// subtmi!(
///     method => Method::POST,
///     url => "http://example.com",
///     client => &mut client,
///     data => "something"
/// )
///
/// # Arguments
///
/// * `method`      - HTTP method to use.
/// * `data`        - Data to send with request.
/// * `url`         - URL to make the GET request to.
/// * `headers`     - Headers to send with request.
/// * `client`      - Client variable to use for request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, ignore
/// use deboa_macros::submit;
/// use deboa_tokio::TokioClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TokioClient::default();
///     submit!(
///         method => http::Method::POST,
///         data => "user=deboa",
///         url => "https://jsonplaceholder.typicode.com/posts",
///         client => &client
///     );
///     Ok(())
/// }
/// ```
pub fn submit(item: OldTokenStream) -> OldTokenStream {
    let args = parse_macro_input!(item as actions::submit::SubmitArgs);
    expanded_submit(args).into()
}

fn expanded_submit(args: SubmitArgs) -> TokenStream {
    // Enforce required fields
    let url = match args.url {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'url'")
                .to_compile_error()
                .into();
        }
    };
    let client = match args.client {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'client'")
                .to_compile_error()
                .into();
        }
    };
    let method = match args.method {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'method'")
                .to_compile_error()
                .into();
        }
    };

    let headers = match args.headers {
        Some(headers) => quote! { .headers(#headers) },
        None => TokenStream::new(),
    };

    let data = match args.data {
        Some(data) => quote! { .text(#data) },
        None => TokenStream::new(),
    };

    // Generate the final token stream
    let expanded = quote! {
        deboa::HttpClient::execute(
            #client,
            deboa::request::DeboaRequest::at(#url, #method)?
                #headers
                #data
                .build()?,
        )
        .await?
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Make a GET request to the specified URL, returning a stream.
///
/// The `stream!` macro is used to make a GET request to the specified URL
///
/// stream!(
///     url => "http://example.com",
///     client => &mut client
/// );
///
/// # Arguments
///
/// * `url`     - URL to make the GET request to.
/// * `headers` - Headers to send with request.
/// * `client`  - Client variable to use for request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, ignore
/// use deboa_macros::stream;
/// use deboa_tokio::TokioClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TokioClient::default();
///     let response = stream!(url => "https://jsonplaceholder.typicode.com/posts", client => &client);
///     Ok(())
/// }
/// ```
pub fn stream(item: OldTokenStream) -> OldTokenStream {
    let args = parse_macro_input!(item as actions::get::GetArgs);
    expanded_stream(args).into()
}

fn expanded_stream(args: GetArgs) -> TokenStream {
    // Enforce required fields
    let url = match args.url {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'url'")
                .to_compile_error()
                .into();
        }
    };
    let client = match args.client {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'client'")
                .to_compile_error()
                .into();
        }
    };

    let headers = match args.headers {
        Some(headers) => quote! { .headers(#headers) },
        None => TokenStream::new(),
    };

    // Generate the final token stream
    let expanded = quote! {
        deboa::HttpClient::execute(
            #client,
            deboa::request::DeboaRequest::get(#url)?
                #headers
                .build()?,
        )
        .await?
        .stream()
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod test {
    use crate::{
        DeleteArgs, GetArgs, PatchArgs, PostArgs, PutArgs, QueryArgs, SubmitArgs, expanded_delete,
        expanded_fetch, expanded_get, expanded_patch, expanded_post, expanded_put, expanded_query,
        expanded_stream, expanded_submit,
    };
    use syn::parse_quote;

    #[test]
    fn test_expand_get() {
        let args: GetArgs = parse_quote!(
            url => "https://jsonplaceholder.typicode.com/posts",
            client => &client,
            res_body_ty => JsonBody,
            res_ty => Vec<Post>
        );
        expanded_get(args);
    }

    #[test]
    fn test_expand_query() {
        let args: QueryArgs = parse_quote!(
            data => data,
            req_body_ty => JsonBody,
            url => "https://jsonplaceholder.typicode.com/posts",
            client => &client,
            res_body_ty => JsonBody,
            res_ty => CreatedPost
        );
        expanded_query(args);
    }

    #[test]
    fn test_expand_post() {
        let args: PostArgs = parse_quote!(
            data => data,
            req_body_ty => JsonBody,
            url => "https://jsonplaceholder.typicode.com/posts",
            headers => vec!(("Content-Type", "application/json")),
            client => &client
        );
        expanded_post(args);
    }

    #[test]
    fn test_expand_put() {
        let args: PutArgs = parse_quote!(
            data => data,
            req_body_ty => JsonBody,
            url => "https://jsonplaceholder.typicode.com/posts/1",
            client => &client
        );
        expanded_put(args);
    }

    #[test]
    fn test_expand_patch() {
        let args: PatchArgs = parse_quote!(
            data => data,
            req_body_ty => JsonBody,
            url => "https://jsonplaceholder.typicode.com/posts/1",
            client => &client
        );
        expanded_patch(args);
    }

    #[test]
    fn test_expand_delete() {
        let args: DeleteArgs = parse_quote!(
            url => "https://jsonplaceholder.typicode.com/posts/1",
            client => &client
        );
        expanded_delete(args);
    }

    #[test]
    fn test_expand_fetch() {
        let args: GetArgs = parse_quote!(
            url => "https://jsonplaceholder.typicode.com/posts",
            client => &client,
            res_body_ty => JsonBody,
            res_ty => Post
        );
        expanded_fetch(args);
    }

    #[test]
    fn test_expand_submit() {
        let args: SubmitArgs = parse_quote!(
            method => http::Method::POST,
            data => "user=deboa",
            url => "https://jsonplaceholder.typicode.com/posts",
            client => &client
        );
        expanded_submit(args);
    }

    #[test]
    fn test_expand_stream() {
        let args: GetArgs = parse_quote!(
            url => "https://jsonplaceholder.typicode.com/posts",
            client => &client
        );
        expanded_stream(args);
    }
}
