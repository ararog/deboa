pub use bora::bora;

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
/// get!(url -> JsonBody -> ty, using &mut client)
///
/// Is the same as:
///
/// get from url with body serialization type and return type, using client variable.
///
/// # Arguments
///
/// * `url`         - The URL to make the GET request to.
/// * `res_body_ty` - The body type of the response.
/// * `res_ty`      - The type of the response.
/// * `client`      - The client variable to use for the request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// let mut client = Deboa::new();
/// let response = get!("https://jsonplaceholder.typicode.com/posts" -> JsonBody -> Vec<Post>, using &mut client);
/// assert_eq!(response.len(), 100);
/// ```
macro_rules! get {
    ($url:literal -> $res_body_ty:ident -> $res_ty:ty, using &mut $client:ident) => {
        $client.execute($url).await?.body_as::<$res_body_ty, $res_ty>($res_body_ty)?
    };

    ($url:ident -> $res_body_ty:ident -> $res_ty:ty, using &mut $client:ident) => {
        $client.execute($url).await?.body_as::<$res_body_ty, $res_ty>($res_body_ty)?
    };
}

#[macro_export]
/// Make a POST request to the specified URL.
///
/// The `post!` macro is used to make a POST request to the specified URL.
///
/// To help understand the macro arguments, here is an example:
///
/// post!(input -> req_body_ty -> url using &mut client)
///
/// Is the same as:
///
/// post input with body serialization type to url using client variable.
///
/// # Arguments
///
/// * `input`       - The input to send with the request.
/// * `req_body_ty` - The body serialization type of the request.
/// * `url`         - The URL to make the POST request to.
/// * `client`      - The client variable to use for the request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// let mut client = Deboa::new();
/// let response = post!(data -> JsonBody -> "https://jsonplaceholder.typicode.com/posts" using &mut client);
/// assert_eq!(response.id, 1);
/// ```
macro_rules! post {
    ($input:ident -> $req_body_ty:ident -> $url:literal using &mut $client:ident) => {
        $client
            .execute(deboa::request::DeboaRequest::post($url)?.body_as($req_body_ty, $input)?.build()?)
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
/// delete!(url using &mut client)
///
/// Is the same as:
///
/// delete from url using client variable.
///
/// # Arguments
///
/// * `url`    - The URL to make the DELETE request to.
/// * `client` - The client variable to use for the request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// let mut client = Deboa::new();
/// let response = delete!("https://jsonplaceholder.typicode.com/posts/1" using &mut client);
/// assert_eq!(response.id, 1);
/// ```
macro_rules! delete {
    ($url:literal using &mut $client:ident) => {
        $client.execute(deboa::request::DeboaRequest::delete($url)?.build()?).await?
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
/// fetch!(url -> body -> ty, using &mut client)
///
/// Is the same as:
///
/// fetch from url with response body type and return type, using client variable.
///
/// # Arguments
///
/// * `url`         - The URL to make the GET request to.
/// * `res_body_ty` - The body type of the response.
/// * `res_ty`      - The type of the response.
/// * `client`      - The client variable to use for the request.
///
/// Please note url can be a string literal or a variable.
///
/// # Example
///
/// ```rust, no_run
/// let mut client = Deboa::new();
/// let response = fetch!("https://jsonplaceholder.typicode.com/posts" -> JsonBody -> Post, using &mut client);
/// assert_eq!(response.id, 1);
/// ```
macro_rules! fetch {
    ($url:literal -> $res_body_ty:ident -> $res_ty:ty, using &mut $client:ident) => {
        $client.execute($url).await?.body_as::<$res_body_ty, $res_ty>($res_body_ty)?
    };

    ($url:ident -> $res_body_ty:ident -> $res_ty:ty, using &mut $client:ident) => {
        $client.execute($url).await?.body_as::<$res_body_ty, $res_ty>($res_body_ty)?
    };
}
