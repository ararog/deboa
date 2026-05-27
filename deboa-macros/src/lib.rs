#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

#[macro_export]
/// Make a GET request to the specified URL.
///
/// The `get!` macro is used to make a GET request to the specified URL.
/// Its first argument is a string literal or a variable. Arrows are
/// used to specify the body serialization type and the output type.
///
/// You can use the `JsonBody`, `XmlBody`, `MsgPack` type for JSON, XML
/// and MessagePack serialization.
///
/// To help understand the macro arguments, here is an example:
///
/// get!(
///     url => url,
///     headers => headers,
///     client => &client
/// )
///
/// or
///
/// get!(
///     url => url,
///     client => &client,
///     res_body_ty => JsonBody,
///     res_ty => ty
/// )
///
/// or
///
/// get!(
///     url=> url,
///     headers => vec![("User-Agent", "deboa")],
///     client => &client,
///     res_body_ty => JsonBody,
///     res_ty => ty
/// )
///
/// # Arguments
///
/// * `url`         - The URL to make the GET request to.
/// * `client`      - The client variable to use for the request.
/// * `res_body_ty` - The body type of the response.
/// * `res_ty`      - The type of the response.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// use deboa_tokio::Client;
/// use deboa_macros::get;
/// use deboa_extras::http::serde::json::JsonBody;
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
macro_rules! get {
    (url => $url:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(&$client, $url)
            .await?
            .text()
            .await?
    };

    (url => $url:expr, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::get($url)?
                .headers($headers)
                .build()?,
        )
        .await?
        .text()
        .await?
    };

    (url => $url:expr, client => &$client:ident, res_body_ty => $res_body_ty:ident, res_ty => $res_ty:ty) => {
        deboa::HttpClient::execute(&$client, $url)
            .await?
            .body_as::<$res_body_ty, $res_ty>($res_body_ty)
            .await?
    };

    (url => $url:expr, headers => $headers:expr, client => &$client:ident, res_body_ty => $res_body_ty:ident, res_ty => $res_ty:ty) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::get($url)?
                .headers($headers)
                .build()?,
        )
        .await?
        .body_as::<$res_body_ty, $res_ty>($res_body_ty)
        .await?
    };
}

#[macro_export]
/// Make a POST request to the specified URL.
///
/// The `post!` macro is used to make a POST request to the specified URL.
///
/// It can be either:
///
/// post!(input, req_body_ty, url, &client)
///
/// or
///   
/// post!(input, req_body_ty, url, headers, &client)
///
/// or
///
/// post!(input, req_body_ty, url, &client, res_body_ty, res_ty)
///
/// or
///
/// post!(input, req_body_ty, url, headers, &client, res_body_ty, res_ty)
///
/// # Arguments
///
/// * `input`       - The input to send with the request.
/// * `req_body_ty` - The body serialization format of the request.
/// * `url`         - The URL to make the POST request to.
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
/// ```rust, no_run
/// use deboa_macros::post;
/// use deboa_extras::http::serde::json::JsonBody;
/// use deboa_tokio::Client;
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
///     let client = Client::default();
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
/// ```rust, no_run
/// use deboa_macros::post;
/// use deboa_extras::http::serde::json::JsonBody;
/// use deboa_tokio::Client;
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
///     let client = Client::default();
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
macro_rules! post {
    (data => $input:ident, url => $url:literal, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, url => $url:expr, client =>&$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, url => $url:literal, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .headers($headers)
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:literal, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty=> $req_body_ty:ident, url => $url:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .headers($headers)
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, client => &$client:ident, res_body_ty => $res_body_ty:ident, res_ty => $res_ty:ty) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
        .body_as::<$res_body_ty, $res_ty>($res_body_ty)
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, headers => $headers:expr, client => &$client:ident, res_body_ty => $res_body_ty:ident, res_ty => $res_ty:ty) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .headers($headers)
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
        .body_as::<$res_body_ty, $res_ty>($res_body_ty)
        .await?
    };
}

#[macro_export]
/// Make a PUT request to the specified URL.
///
/// The `put!` macro is used to make a PUT request to the specified URL
/// Its first argument is a string literal or a variable.
///
/// To help understand the macro arguments, here is an example:
///
/// put!(input, req_body_ty, url, &mut client)
///
/// # Arguments
///
/// * `input`       - The input to send with request.
/// * `req_body_ty` - The body serialization format of request.
/// * `url`         - The URL to make the PUT request to.
/// * `headers`     - The headers to send with request.
/// * `client`      - The client variable to use for request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// use deboa_macros::put;
/// use deboa_extras::http::serde::json::JsonBody;
/// use deboa_tokio::Client;
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
///     put!(
///         data => data,
///         req_body_ty => JsonBody,
///         url => "https://jsonplaceholder.typicode.com/posts/1",
///         client => &client
///     );
///     Ok(())
/// }
/// ```
macro_rules! put {
    (data => $input:ident, url => $url:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, url => $url:expr, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .headers($headers)
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .headers($headers)
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, client => &$client:ident, res_body_ty => $res_body_ty:ident, res_ty => $res_ty:ty) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
        .body_as::<$res_body_ty, $res_ty>($res_body_ty)
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, headers => $headers:expr, client => &$client:ident, res_body_ty => $res_body_ty:ident, res_ty => $res_ty:ty) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .headers($headers)
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
        .body_as::<$res_body_ty, $res_ty>($res_body_ty)
        .await?
    };
}

#[macro_export]
/// Make a PATCH request to the specified URL.
///
/// The `patch!` macro is used to make a PATCH request to the specified URL
/// Its first argument is a string literal or a variable.
///
/// To help understand the macro arguments, here is an example:
///
/// patch!(input, req_body_ty, url, &mut client)
///
/// # Arguments
///
/// * `input`       - The input to send with request.
/// * `req_body_ty` - The body serialization format of request.
/// * `url`         - The URL to make the PATCH request to.
/// * `headers`     - The headers to send with request.
/// * `client`      - The client variable to use for request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// use deboa_tokio::Client;
/// use deboa_macros::patch;
/// use deboa_extras::http::serde::json::JsonBody;
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
macro_rules! patch {
    (data => $input:ident, url => $url:literal, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:literal, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .headers($headers)
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .headers($headers)
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:expr, url => $url:expr, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .headers($headers)
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, client => &$client:ident, res_body_ty => $res_body_ty:ident, res_ty => $res_ty:ty) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
        .body_as::<$res_body_ty, $res_ty>($res_body_ty)
        .await?
    };

    (data => $input:ident, req_body_ty => $req_body_ty:ident, url => $url:expr, headers => $headers:expr, client => &$client:ident, res_body_ty => $res_body_ty:ident, res_ty => $res_ty:ty) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .headers($headers)
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
        .body_as::<$res_body_ty, $res_ty>($res_body_ty)
        .await?
    };
}

#[macro_export]
/// Make a DELETE request to the specified URL.
///
/// The `delete!` macro is used to make a DELETE request to the specified URL
/// Its first argument is a string literal or a variable.
///
/// To help understand the macro arguments, here is an example:
///
/// delete!(url, &mut client)
///
/// # Arguments
///
/// * `url`    - The URL to make the DELETE request to.
/// * `headers` - The headers to send with request.
/// * `client` - The client variable to use for request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// use deboa_macros::delete;
/// use deboa_tokio::Client;
///
/// #[derive(serde::Deserialize)]
/// struct Post {
///     id: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::default();
///     let response = delete!(url => "https://jsonplaceholder.typicode.com/posts/1", client => &client);
///     Ok(())
/// }
/// ```
macro_rules! delete {
    (url => $url:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(&$client, deboa::request::DeboaRequest::delete($url)?.build()?)
            .await?
    };

    (url => $url:expr, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::delete($url)?
                .headers($headers)
                .build()?,
        )
        .await?
    };
}

#[macro_export]
/// Make a GET request to the specified URL.
///
/// The `fetch!` macro is a more generic version of the `get!` macro.
/// Its first argument is a string literal or a variable. Arrows are
/// used to specify the body serialization type and the output type.
///
/// You can use the `JsonBody`, `XmlBody`, `MsgPack` type for JSON, XML
/// and MessagePack serialization.
///
/// To help understand the macro arguments, here is an example:
///
/// fetch!(url, &mut client, body, ty)
///
/// # Arguments
///
/// * `url`         - The URL to make the GET request to.
/// * `headers`     - The headers to send with request.
/// * `client`      - The client variable to use request.
/// * `res_body_ty` - The body serialization format of response.
/// * `res_ty`      - The type of response.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// use deboa_macros::fetch;
/// use deboa_extras::http::serde::json::JsonBody;
/// use deboa_tokio::Client;
///
/// #[derive(serde::Deserialize)]
/// struct Post {
///     id: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::default();
///     let response = fetch!(url => "https://jsonplaceholder.typicode.com/posts", client => &client, res_body_ty => JsonBody, res_ty => Post);
///     assert_eq!(response.id, 1);
///     Ok(())
/// }
/// ```
macro_rules! fetch {
    (url => $url:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(&$client, $url).await?
    };

    (url => $url:expr, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::get($url)?
                .headers($headers)
                .build()?,
        )
        .await?
    };

    (url => $url:expr, client => &$client:ident, res_body_ty => $res_body_ty:ident, res_ty => $res_ty:ty) => {
        deboa::HttpClient::execute(&$client, $url)
            .await?
            .body_as::<$res_body_ty, $res_ty>($res_body_ty)
            .await?
    };

    (url => $url:expr, headers => $headers:expr, client => &$client:ident, res_body_ty => $res_body_ty:ident, res_ty => $res_ty:ty) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::get($url)?
                .headers($headers)
                .build()?,
        )
        .await?
        .body_as::<$res_body_ty, $res_ty>($res_body_ty)
        .await?
    };
}
#[macro_export]
/// Submit a request to the specified URL.
///
/// The `submit!` macro is a more generic version of the `get!` macro.
/// Its first argument is a string literal or a variable. Arrows are
/// used to specify the body serialization type and the output type.
///
/// You can use the `JsonBody`, `XmlBody`, `MsgPack` type for JSON, XML
/// and MessagePack serialization.
///
/// To help understand the macro arguments, here is an example:
///
/// fetch!(url, &mut client, body, ty)
///
/// # Arguments
///
/// * `method`      - The HTTP method to use.
/// * `input`       - The input to send with request.
/// * `url`         - The URL to make the GET request to.
/// * `headers`     - The headers to send with request.
/// * `client`      - The client variable to use for request.
/// * `res_body_ty` - The body serialization format of response.
/// * `res_ty`      - The type of response.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// use deboa_macros::submit;
/// use deboa_tokio::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::default();
///     submit!(method => http::Method::POST, data => "user=deboa", url => "https://jsonplaceholder.typicode.com/posts", client => &client);
///     Ok(())
/// }
/// ```
macro_rules! submit {
    (method => $method:expr, data => $input:expr, url => $url:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::at($url, $method)?
                .text($input)
                .build()?,
        )
        .await?
    };

    (method => $method:expr, data => $input:expr, url => $url:expr, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::at($url, $method)?
                .headers($headers)
                .text($input)
                .build()?,
        )
        .await?
    };
}

#[macro_export]
/// Make a GET request to the specified URL, returning a stream.
///
/// The `stream!` macro is used to make a GET request to the specified URL
/// Its first argument is a string literal or a variable.
///
/// To help understand the macro arguments, here is an example:
///
/// stream!(url, &mut client)
///
/// # Arguments
///
/// * `url`    - The URL to make the GET request to.
/// * `headers` - The headers to send with request.
/// * `client` - The client variable to use for request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// use deboa_macros::stream;
/// use deboa_tokio::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::default();
///     let response = stream!(url => "https://jsonplaceholder.typicode.com/posts", client => &client);
///     Ok(())
/// }
/// ```
macro_rules! stream {
    (url => $url:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(&$client, $url)
            .await?
            .stream()
    };

    (url => $url:expr, headers => $headers:expr, client => &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::get($url)?
                .headers($headers)?
                .build()?,
        )
        .await?
        .stream()
    };
}
