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
/// get!(url, headers, &mut client)
///
/// or
///
/// get!(url, &mut client, JsonBody, ty)
///
/// or
///
/// get!(url, vec![("User-Agent", "deboa")], &mut client, JsonBody, ty)
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
///     let response = get!("https://jsonplaceholder.typicode.com/posts", &client, JsonBody, Vec<Post>);
///     assert_eq!(response.len(), 100);
///     Ok(())
/// }
/// ```
macro_rules! get {
    ($url:expr, &$client:ident) => {
        deboa::HttpClient::execute(&$client, $url)
            .await?
            .text()
            .await?
    };

    ($url:expr, $headers:expr, &$client:ident) => {
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

    ($url:expr, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
        deboa::HttpClient::execute(&$client, $url)
            .await?
            .body_as::<$res_body_ty, $res_ty>($res_body_ty)
            .await?
    };

    ($url:expr, $headers:expr, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
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
///         data,
///         JsonBody,
///         "https://jsonplaceholder.typicode.com/posts",
///         vec!(("Content-Type", "application/json")),
///         &client
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
///     let response = post!(data, JsonBody, "https://jsonplaceholder.typicode.com/posts", &client, JsonBody, CreatedPost);
///     assert_eq!(response.id, 1);
///     Ok(())
/// }
/// ```
macro_rules! post {
    ($input:ident, $url:literal, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $url:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $url:literal, $headers:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .headers($headers)
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:literal, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:expr, $headers:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::post($url)?
                .headers($headers)
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:expr, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
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

    ($input:ident, $req_body_ty:ident, $url:expr, $headers:expr, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
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
///     put!(data, JsonBody, "https://jsonplaceholder.typicode.com/posts/1", &client);
///     Ok(())
/// }
/// ```
macro_rules! put {
    ($input:ident, $url:literal, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $url:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $url:literal, $headers:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .headers($headers)
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:literal, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:expr, $headers:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::put($url)?
                .headers($headers)
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:expr, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
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

    ($input:ident, $req_body_ty:ident, $url:expr, $headers:expr, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
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
///     patch!(data, JsonBody, "https://jsonplaceholder.typicode.com/posts/1", &client);
///     Ok(())
/// }
/// ```
macro_rules! patch {
    ($input:ident, $url:literal, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .body_as(deboa_extras::http::serde::json::JsonBody, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:literal, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:literal, $headers:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::patch($url)?
                .headers($headers)
                .body_as($req_body_ty, $input)?
                .build()?,
        )
        .await?
    };

    ($input:ident, $req_body_ty:ident, $url:expr, $headers:expr, &$client:ident) => {
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

    ($input:ident, $req_body_ty:ident, $url:expr, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
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

    ($input:ident, $req_body_ty:ident, $url:expr, $headers:expr, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
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
///     let response = delete!("https://jsonplaceholder.typicode.com/posts/1", &client);
///     Ok(())
/// }
/// ```
macro_rules! delete {
    ($url:literal, &$client:ident) => {
        deboa::HttpClient::execute(&$client, deboa::request::DeboaRequest::delete($url)?.build()?)
            .await?
    };

    ($url:expr, &$client:ident) => {
        deboa::HttpClient::execute(&$client, deboa::request::DeboaRequest::delete($url)?.build()?)
            .await?
    };

    ($url:expr, $headers:expr, &$client:ident) => {
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
///     let response = fetch!("https://jsonplaceholder.typicode.com/posts", &client, JsonBody, Post);
///     assert_eq!(response.id, 1);
///     Ok(())
/// }
/// ```
macro_rules! fetch {
    ($url:expr, &$client:ident) => {
        deboa::HttpClient::execute(&$client, $url).await?
    };

    ($url:expr, $headers:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::get($url)?
                .headers($headers)
                .build()?,
        )
        .await?
    };

    ($url:literal, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
        deboa::HttpClient::execute(&$client, $url)
            .await?
            .body_as::<$res_body_ty, $res_ty>($res_body_ty)
            .await?
    };

    ($url:expr, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
        deboa::HttpClient::execute(&$client, $url)
            .await?
            .body_as::<$res_body_ty, $res_ty>($res_body_ty)
            .await?
    };

    ($url:expr, $headers:expr, &$client:ident, $res_body_ty:ident, $res_ty:ty) => {
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
///     submit!(http::Method::POST, "user=deboa", "https://jsonplaceholder.typicode.com/posts", &client);
///     Ok(())
/// }
/// ```
macro_rules! submit {
    ($method:expr, $input:expr, $url:expr, &$client:ident) => {
        deboa::HttpClient::execute(
            &$client,
            deboa::request::DeboaRequest::at($url, $method)?
                .text($input)
                .build()?,
        )
        .await?
    };

    ($method:expr, $input:expr, $url:expr, $headers:expr, &$client:ident) => {
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
///     let response = stream!("https://jsonplaceholder.typicode.com/posts", &client);
///     Ok(())
/// }
/// ```
macro_rules! stream {
    ($url:expr, &$client:ident) => {
        deboa::HttpClient::execute(&$client, $url)
            .await?
            .stream()
    };

    ($url:expr, $headers:expr, &$client:ident) => {
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
