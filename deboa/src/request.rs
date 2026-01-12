//! # HTTP Request Module
//!
//! This module provides comprehensive HTTP request building and handling functionality
//! for the Deboa HTTP client. It includes traits and structs for creating, configuring,
//! and executing HTTP requests with various features like authentication, headers,
//! cookies, and body serialization.
//!
//! ## Key Components
//!
//! - [`IntoRequest`]: Trait for converting various types into HTTP requests
//! - [`IntoHeaders`]: Trait for converting various types into HTTP headers
//! - [`DeboaRequest`]: Main request structure with full HTTP functionality
//! - Request builders for different HTTP methods (GET, POST, PUT, DELETE, etc.)
//! - Authentication mechanisms (Basic, Bearer token, custom)
//! - Header and cookie management
//! - Form data and JSON serialization support
//!
//! ## Features
//!
//! - Type-safe request building
//! - Automatic content-type handling
//! - Authentication support (Basic, Bearer, custom)
//! - Cookie jar integration
//! - Form data and JSON serialization
//! - File upload support
//! - Request retry mechanisms
//! - Custom headers and query parameters
//!
//! ## Examples
//!
//! ### Basic GET Request
//!
//! ```rust, ignore
//! use deboa::{Client, request::IntoRequest};
//!
//! let mut client = Client::new();
//! let response = "https://api.example.com/data".into_request().execute(&mut client).await?;
//! ```
//!
//! ### POST Request with JSON
//!
//! ```rust, ignore
//! use deboa::{Client, request::post};
//! use deboa_extras::http::serde::json::JsonBody;
//!
//! let mut client = Client::new();
//! let response = post("https://api.example.com/users")
//!     .body_as(JsonBody, json!({"name": "John", "age": 30}))?
//!     .send_with(&mut client)
//!     .await?;
//! ```
//!
//! ### Authentication
//!
//! ```rust, ignore
//! use deboa::{Client, request::get};
//!
//! let mut client = Client::new();
//! let response = get("https://api.example.com/protected")
//!     .basic_auth("username", "password")
//!     .send_with(&mut client)
//!     .await?;
//! ```

use std::{collections::HashMap, fmt::Debug, future::Future, str::FromStr, sync::Arc};

use bytes::Bytes;
#[cfg(feature = "http3")]
use h3_quinn::OpenStreams;
use http::{
    header::{self, HOST},
    HeaderMap, HeaderName, HeaderValue, Method,
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
#[cfg(any(feature = "http1", feature = "http2"))]
use http_body_util::Full;
use log::error;
use regex::Regex;
use serde::Serialize;
use url::Url;

use crate::{
    client::serde::RequestBody,
    cookie::DeboaCookie,
    errors::{DeboaError, RequestError},
    form::{DeboaForm, Form},
    response::DeboaResponse,
    url::IntoUrl,
    Client, Result,
};

#[cfg(feature = "http1")]
pub type Http1Request = hyper::client::conn::http1::SendRequest<Full<Bytes>>;
#[cfg(feature = "http2")]
pub type Http2Request = hyper::client::conn::http2::SendRequest<Full<Bytes>>;
#[cfg(feature = "http3")]
pub type Http3Request = h3::client::SendRequest<OpenStreams, Bytes>;

/// Trait to allow making a request from different types.
///
/// This trait provides a flexible way to convert various input types into
/// HTTP requests. It enables convenient request creation from strings, URLs,
/// and other request-like objects.
///
/// # Examples
///
/// ``` compile_fail
/// use deboa::{Client, request::IntoRequest};
///
/// let mut client = Client::new();
///
/// let response = "https://jsonplaceholder.typicode.com"
///   .into_request()
///   .await?;
/// assert_eq!(response.status(), 200);
/// ```
pub trait IntoRequest: private::IntoRequestSealed {
    fn into_request(self) -> Result<DeboaRequest>;
}

impl IntoRequest for DeboaRequest {
    fn into_request(self) -> Result<DeboaRequest> {
        Ok(self)
    }
}

impl IntoRequest for &str {
    fn into_request(self) -> Result<DeboaRequest> {
        DeboaRequest::get(self)?.build()
    }
}

impl IntoRequest for String {
    fn into_request(self) -> Result<DeboaRequest> {
        DeboaRequest::get(self)?.build()
    }
}

impl IntoRequest for Url {
    fn into_request(self) -> Result<DeboaRequest> {
        DeboaRequest::get(self)?.build()
    }
}

/// Trait to allow adding headers to a request.
///
/// This trait provides a flexible way to convert various input types into
/// HTTP headers.
///
/// # Examples
///
/// ``` compile_fail
/// use deboa::request::{IntoHeaders, DeboaRequest, DeboaRequestBuilder};
///
/// let headers = vec![("User-Agent", "deboa/0.1")];
/// let request = DeboaRequest::get("https://example.com")?
///     .headers(headers)
///     .build()?;
/// ```
pub trait IntoHeaders: private::IntoHeadersSealed {
    fn into_headers(self) -> Result<HeaderMap>;
}

impl IntoHeaders for HeaderMap {
    fn into_headers(self) -> Result<HeaderMap> {
        Ok(self)
    }
}

impl IntoHeaders for Vec<(HeaderName, String)> {
    fn into_headers(self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        for (key, value) in self {
            headers.insert(&key, HeaderValue::from_str(&value).expect("Invalid header value"));
        }
        Ok(headers)
    }
}

impl IntoHeaders for Vec<(String, String)> {
    fn into_headers(self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        for (key, value) in self {
            headers.insert(
                HeaderName::from_str(&key).expect("Invalid header name"),
                HeaderValue::from_str(&value).expect("Invalid header value"),
            );
        }
        Ok(headers)
    }
}

impl<'a> IntoHeaders for Vec<(&'a str, &'a str)> {
    fn into_headers(self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        for (key, value) in self {
            headers.insert(
                HeaderName::from_str(key).expect("Invalid header name"),
                HeaderValue::from_str(value).expect("Invalid header value"),
            );
        }
        Ok(headers)
    }
}

/// Extension trait for HTTP methods to create requests.
/// Allows creating requests using method names as strings or Method enum values.
///
/// # Examples
/// ``` compile_fail
/// use http::Method;
/// use deboa::request::MethodExt;
///
/// // Using Method enum
/// let request = Method::GET.from_url("https://example.com")?;
///
/// // Using string
/// let request = "GET".from_url("https://example.com")?;
/// ```
pub trait MethodExt: private::MethodExtSealed {
    fn from_url(self, url: &str) -> Result<DeboaRequestBuilder>;
    fn to_url(self, url: &str) -> Result<DeboaRequestBuilder>;
}

impl MethodExt for Method {
    fn from_url(self, url: &str) -> Result<DeboaRequestBuilder> {
        match self {
            Method::GET => DeboaRequest::get(url),
            Method::POST => DeboaRequest::post(url),
            Method::PUT => DeboaRequest::put(url),
            Method::DELETE => DeboaRequest::delete(url),
            Method::PATCH => DeboaRequest::patch(url),
            _ => panic!("Method not supported"),
        }
    }

    fn to_url(self, url: &str) -> Result<DeboaRequestBuilder> {
        self.from_url(url)
    }
}

impl MethodExt for &str {
    fn from_url(self, url: &str) -> Result<DeboaRequestBuilder> {
        match self {
            "GET" | "get" => DeboaRequest::get(url),
            "POST" | "post" => DeboaRequest::post(url),
            "PUT" | "put" => DeboaRequest::put(url),
            "DELETE" | "delete" => DeboaRequest::delete(url),
            "PATCH" | "patch" => DeboaRequest::patch(url),
            _ => panic!("Method not supported"),
        }
    }

    fn to_url(self, url: &str) -> Result<DeboaRequestBuilder> {
        self.from_url(url)
    }
}

#[deprecated(note = "Use FetchWith trait instead", since = "0.0.8")]
/// Trait to allow make a get request from different types.
pub trait Fetch {
    /// Fetch the request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use deboa::{Client, request::Fetch};
    ///
    /// let client = Client::new();
    ///
    /// let response = "https://jsonplaceholder.typicode.com"
    ///   .fetch(&client)
    ///   .await?;
    /// assert_eq!(response.status(), 200);
    /// ```
    ///
    fn fetch<T>(&self, client: T) -> impl Future<Output = Result<DeboaResponse>>
    where
        T: AsRef<Client> + Send;
}

#[allow(deprecated)]
impl Fetch for &str {
    async fn fetch<T>(&self, client: T) -> Result<DeboaResponse>
    where
        T: AsRef<Client> + Send,
    {
        DeboaRequest::get(*self)?
            .send_with(client)
            .await
    }
}

/// Trait to allow make a get request from different types.
///
/// # Examples
///
/// ``` compile_fail
/// use deboa::{Deboa, request::FetchWith};
///
/// let client = Deboa::default();
///
/// let response = "https://jsonplaceholder.typicode.com"
///   .fetch_with(&client)
///   .await?;
/// assert_eq!(response.status(), 200);
/// ```
pub trait FetchWith {
    /// Fetch the request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use deboa::{Client, request::FetchWith};
    ///
    /// let client = Client::new();
    ///
    /// let response = "https://jsonplaceholder.typicode.com"
    ///   .fetch_with(&client)
    ///   .await?;
    /// assert_eq!(response.status(), 200);
    /// ```
    ///
    fn fetch_with<T>(&self, client: T) -> impl Future<Output = Result<DeboaResponse>>
    where
        T: AsRef<Client> + Send;
}

impl FetchWith for &str {
    async fn fetch_with<T>(&self, client: T) -> Result<DeboaResponse>
    where
        T: AsRef<Client> + Send,
    {
        DeboaRequest::get(*self)?
            .send_with(client)
            .await
    }
}

impl FetchWith for String {
    async fn fetch_with<T>(&self, client: T) -> Result<DeboaResponse>
    where
        T: AsRef<Client> + Send,
    {
        DeboaRequest::get(self)?
            .send_with(client)
            .await
    }
}

/// A utility function to create a GET request within DeboaRequest.
///
/// # Arguments
///
/// * `url` - The url to connect.
///
/// # Returns
///
/// * `Result<DeboaRequestBuilder>` - The request builder.
///
/// # Examples
///
/// ``` compile_fail
/// use deboa::{Client, request::get};
///
/// let client = Client::new();
///
/// let request = get("https://jsonplaceholder.typicode.com").unwrap();
/// let response = request.send_with(&client).await?;
/// assert_eq!(response.status(), 200);
/// ```
///
#[inline]
pub fn get<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
    DeboaRequest::get(url)
}

/// A utility function to create a POST request within DeboaRequest.
///
/// # Arguments
///
/// * `url` - The url to connect.
///
/// # Returns
///
/// * `Result<DeboaRequestBuilder>` - The request builder.
///
/// # Examples
///
/// ``` compile_fail
/// use deboa::{Client, request::post};
///
/// let client = Client::new();
///
/// let request = post("https://jsonplaceholder.typicode.com/posts")?
///   .raw_body(b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}")
///   .build()?;
/// let response = request.send_with(&client).await?;
/// assert_eq!(response.status(), 201);
/// ```
///
#[inline]
pub fn post<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
    DeboaRequest::post(url)
}

/// A utility function to create a PUT request within DeboaRequest.
///
/// # Arguments
///
/// * `url` - The url to connect.
///
/// # Returns
///
/// * `Result<DeboaRequestBuilder>` - The request builder.
///
/// # Examples
///
/// ``` compile_fail
/// use deboa::{Client, request::put};
///
/// let client = Client::new();
///
/// let request = put("https://jsonplaceholder.typicode.com/posts/1")?
///   .raw_body(b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}")
///   .build()?;
/// let response = request.send_with(&client).await?;
/// assert_eq!(response.status(), 200);
/// ```
#[inline]
pub fn put<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
    DeboaRequest::put(url)
}

/// A utility function to create a DELETE request within DeboaRequest.
///
/// # Arguments
///
/// * `url` - The url to connect.
///
/// # Returns
///
/// * `Result<DeboaRequestBuilder>` - The request builder.
///
/// # Examples
///
/// ``` compile_fail
/// use deboa::{Client, request::delete};
///
/// let client = Client::new();
///
/// let request = delete("https://jsonplaceholder.typicode.com/posts/1").build();
/// let response = request.send_with(&client).await?;
/// assert_eq!(response.status(), 200);
/// ```
#[inline]
pub fn delete<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
    DeboaRequest::delete(url)
}

/// A utility function to create a PATCH request within DeboaRequest.
///
/// # Arguments
///
/// * `url` - The url to connect.
///
/// # Returns
///
/// * `Result<DeboaRequestBuilder>` - The request builder.
///
/// # Examples
///
/// ``` compile_fail
/// use deboa::{Client, request::patch};
///
/// let client = Client::new();
///
/// let request = patch("https://jsonplaceholder.typicode.com/posts/1")?
///   .raw_body(b"{\"title\": \"foo\"}")
///   .build()?;
/// let response = request.send_with(&client).await?;
/// assert_eq!(response.status(), 200);
/// ```
#[inline]
pub fn patch<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
    DeboaRequest::patch(url)
}

/// A builder for constructing HTTP requests with various configurations.
///
/// `DeboaRequestBuilder` provides a fluent interface for building and customizing
/// HTTP requests. It supports setting headers, cookies, request bodies, and more.
///
/// # Examples
///
/// ```ignore
/// use deboa::{request::post, Result};
/// use http::header;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///   let request = post("https://httpbin.org/post")?
///     .header(header::CONTENT_TYPE, "application/json")
///     .header(header::ACCEPT, "application/json")
///     .text(r#"{"key":"value"}"#)
///     .build()?;
///   Ok(())
/// }
/// ```
///
/// # Fields
///
/// * `retries` - Number of retry attempts for failed requests
/// * `url` - The target URL for the request
/// * `headers` - HTTP headers to include in the request
/// * `cookies` - Optional cookies to include in the request
/// * `method` - The HTTP method (GET, POST, etc.)
/// * `body` - The request body as raw bytes
/// * `form` - Optional form data for form submissions
pub struct DeboaRequestBuilder {
    retries: u32,
    url: Arc<Url>,
    headers: HeaderMap,
    cookies: Option<HashMap<String, DeboaCookie>>,
    method: http::Method,
    body: Arc<[u8]>,
    form: Option<Form>,
}

impl DeboaRequestBuilder {
    /// Allow set request retries at any time.
    ///
    /// # Arguments
    ///
    /// * `retries` - The new retries.
    ///
    #[inline]
    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `method` - The method.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    #[inline]
    pub fn method(mut self, method: http::Method) -> Self {
        self.method = method;
        self
    }

    /// Set the body of the request as raw bytes.
    ///
    /// # Arguments
    ///
    /// * `body` - The body.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    #[inline]
    pub fn raw_body(mut self, body: &[u8]) -> Self {
        self.body = body.into();
        self
    }

    /// Set the headers of the request.
    ///
    /// # Arguments
    ///
    /// * `headers` - The headers.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    #[inline]
    pub fn headers<I>(mut self, headers: I) -> Self
    where
        I: IntoHeaders,
    {
        self.headers = headers
            .into_headers()
            .unwrap_or_default();
        self
    }

    /// Add a header to the request.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key.
    /// * `value` - The header value.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use deboa::request::post;
    /// use http::header;
    ///
    /// let request = post("https://jsonplaceholder.typicode.com/posts")?
    ///   .header(header::CONTENT_TYPE, "application/json")
    ///   .raw_body(b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}")
    ///   .build()?;
    /// let response = request.send_with(&mut client).await?;
    /// assert_eq!(response.status(), 201);
    /// ```
    ///
    #[inline]
    pub fn header(mut self, key: HeaderName, value: &str) -> Self {
        self.headers
            .insert(key, HeaderValue::from_str(value).unwrap());
        self
    }

    /// Set the cookies of the request.
    ///
    /// # Arguments
    ///
    /// * `cookies` - The cookies.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    #[inline]
    pub fn cookies(mut self, cookies: HashMap<String, DeboaCookie>) -> Self {
        self.cookies = Some(cookies);
        self
    }

    /// Add a cookie to the request.
    ///
    /// # Arguments
    ///
    /// * `cookie` - The cookie.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    #[inline]
    pub fn cookie(mut self, cookie: DeboaCookie) -> Self {
        if let Some(cookies) = &mut self.cookies {
            cookies.insert(
                cookie
                    .name()
                    .to_string(),
                cookie,
            );
        } else {
            self.cookies = Some(HashMap::from([(
                cookie
                    .name()
                    .to_string(),
                cookie,
            )]));
        }
        self
    }

    /// Set multipart form of the request.
    /// Content-Type will be set to `multipart/form-data` or `application/x-www-form-urlencoded`
    /// based on the enum variant.
    ///
    /// # Arguments
    ///
    /// * `form` - The form.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    /// # Examples
    ///
    /// ```compile_fail
    /// use deboa::request::post;
    /// use deboa::form::MultiPartForm;
    ///
    /// let mut form = MultiPartForm::builder();
    /// form.field("name", "deboa");
    /// form.field("version", "0.0.1");
    ///
    /// let request = post("https://jsonplaceholder.typicode.com/posts")?
    ///   .form(form.into())
    ///   .build()?;
    /// let response = request.send_with(&mut client).await?;
    /// assert_eq!(response.status(), 201);
    /// ```
    #[inline]
    pub fn form(mut self, form: Form) -> Self {
        self.form = Some(form);
        self
    }

    /// Set the body of the request as text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    /// # Examples
    ///
    /// ```compile_fail
    /// use deboa::request::post;
    ///
    /// let request = post("https://jsonplaceholder.typicode.com/posts")?
    ///   .header(header::CONTENT_TYPE, "application/json")
    ///   .text("text")
    ///   .build()?;
    /// let response = request.send_with(&mut client).await?;
    /// assert_eq!(response.status(), 201);
    /// ```
    #[inline]
    pub fn text(mut self, text: &str) -> Self {
        self.body = text
            .as_bytes()
            .into();
        self
    }

    /// Set the body of the request as a type.
    ///
    /// # Arguments
    ///
    /// * `body_type` - The body type.
    /// * `body` - The body.
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - The request builder.
    ///
    /// # Examples
    ///
    /// ```compile_fail
    /// use deboa::request::post;
    /// use deboa_extras::http::serde::JsonBody;
    ///
    /// let body = serde_json::json!({
    ///   "name": "deboa",
    ///   "version": "0.0.1"
    /// });
    ///
    /// let request = post("https://some.api.com/ping")?
    ///   .header(header::CONTENT_TYPE, "application/json")
    ///   .body_as(JsonBody, body)
    ///   .build()?;
    /// let response = request.send_with(&mut client).await?;
    /// assert_eq!(response.status(), 200);
    /// ```
    #[inline]
    pub fn body_as<T: RequestBody, B: Serialize>(mut self, body_type: T, body: B) -> Result<Self> {
        self.body = body_type
            .serialize(body)?
            .into();
        Ok(self)
    }

    /// Add bearer auth to the request.
    ///
    /// # Arguments
    ///
    /// * `token` - The token.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    /// # Examples
    ///
    /// ```compile_fail
    /// use deboa::request::post;
    ///
    /// let request = post("https://jsonplaceholder.typicode.com/posts")?
    ///   .header(header::CONTENT_TYPE, "application/json")
    ///   .bearer_auth("token")
    ///   .raw_body(b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}")
    ///   .build()?;
    /// let response = request.send_with(&mut client).await?;
    /// assert_eq!(response.status(), 201);
    /// ```
    #[inline]
    pub fn bearer_auth(self, token: &str) -> Self {
        self.header(header::AUTHORIZATION, format!("Bearer {token}").as_str())
    }

    /// Add basic auth to the request.
    ///
    /// # Arguments
    ///
    /// * `username` - The username.
    /// * `password` - The password.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    /// # Examples
    ///
    /// ```compile_fail
    /// use deboa::request::post;
    ///
    /// let request = post("https://jsonplaceholder.typicode.com/posts")?
    ///   .header(header::CONTENT_TYPE, "application/json")
    ///   .basic_auth("username", "password")
    ///   .raw_body(b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}")
    ///   .build()?;
    /// let response = request.send_with(&mut client).await?;
    /// assert_eq!(response.status(), 201);
    /// ```
    #[inline]
    pub fn basic_auth(self, username: &str, password: &str) -> Self {
        self.header(
            header::AUTHORIZATION,
            format!("Basic {}", STANDARD.encode(format!("{username}:{password}"))).as_str(),
        )
    }

    /// Build the request. Consuming the builder.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaRequest>` - The request.
    ///
    /// # Panics
    ///
    /// * If an error occurs while building the request
    ///
    #[inline]
    pub fn build(self) -> Result<DeboaRequest> {
        let mut request = DeboaRequest {
            url: self.url,
            headers: self.headers,
            cookies: self.cookies,
            retries: self.retries,
            method: self.method,
            body: self.body,
        };

        if let Some(host) = request.url().host() {
            request.add_header(
                header::HOST,
                host.to_string()
                    .as_str(),
            );
        }

        let content_length = request
            .raw_body()
            .len();
        request.add_header(header::CONTENT_LENGTH, &content_length.to_string());

        if let Some(form) = self.form {
            request.set_form(form);
        }

        Ok(request)
    }

    /// Send the request. Consuming the builder.
    ///
    /// # Arguments
    ///
    /// * `client` - The client to be used.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    /// # Examples
    ///
    /// ```compile_fail
    /// use deboa::request::post;
    ///
    /// let request = post("https://jsonplaceholder.typicode.com/posts")?
    ///   .header(header::CONTENT_TYPE, "application/json")
    ///   .raw_body(b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}")
    ///   .build()?;
    /// let response = request.send_with(&mut client).await?;
    /// assert_eq!(response.status(), 201);
    /// ```
    #[deprecated(note = "Use `send_with` method instead", since = "0.0.8")]
    #[inline]
    pub async fn go<T>(self, client: T) -> Result<DeboaResponse>
    where
        T: AsRef<Client>,
    {
        client
            .as_ref()
            .execute(self.build()?)
            .await
    }

    /// Send the request. Consuming the builder.
    ///
    /// # Arguments
    ///
    /// * `client` - The client to be used.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    /// # Panics
    ///
    /// * If an error occurs while sending the request
    ///
    /// # Examples
    ///
    /// ```compile_fail
    /// use deboa::request::post;
    ///
    /// let request = post("https://jsonplaceholder.typicode.com/posts")?
    ///   .header(header::CONTENT_TYPE, "application/json")
    ///   .raw_body(b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}")
    ///   .build()?;
    /// let response = request.send_with(&mut client).await?;
    /// assert_eq!(response.status(), 201);
    /// ```
    #[inline]
    pub async fn send_with<T>(self, client: T) -> Result<DeboaResponse>
    where
        T: AsRef<Client>,
    {
        client
            .as_ref()
            .execute(self.build()?)
            .await
    }
}

pub struct DeboaRequest {
    url: Arc<Url>,
    headers: HeaderMap,
    cookies: Option<HashMap<String, DeboaCookie>>,
    retries: u32,
    method: http::Method,
    body: Arc<[u8]>,
}

impl Debug for DeboaRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeboaRequest")
            .field("url", &self.url)
            .field("headers", &self.headers)
            .field("cookies", &self.cookies)
            .field("retries", &self.retries)
            .field("method", &self.method)
            .field("body", &self.body)
            .finish()
    }
}

/// Parse a string into a DeboaRequest.
///
/// # Arguments
///
/// * `s` - The string to parse.
///
/// # Returns
///
/// * `Result<DeboaRequest>` - The parsed request.
///
/// # Examples
///
/// ```compile_fail
/// use deboa::request::DeboaRequest;
///
/// let request = DeboaRequest::from_str("GET https://jsonplaceholder.typicode.com/posts").unwrap();
/// assert_eq!(request.method(), http::Method::GET);
/// assert_eq!(request.url(), "https://jsonplaceholder.typicode.com/posts");
/// ```
impl FromStr for DeboaRequest {
    type Err = DeboaError;

    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines();

        let mut headers = HeaderMap::new();
        let mut url = String::new();
        let mut method = String::new();
        let mut body = Vec::new();
        let mut is_reading_body = false;

        let method_url_regex =
            Regex::new(r"(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)\s+(https?://[^\s]+)");
        if let Err(e) = method_url_regex {
            error!("Failed to parse request: {}", e);
            return Err(DeboaError::Request(RequestError::Parse { message: e.to_string() }));
        }

        for line in lines {
            let line = line.trim();
            if !is_reading_body {
                let regex = method_url_regex
                    .as_ref()
                    .unwrap();
                let captures = regex.captures(line);
                if let Some(captures) = captures {
                    let method_cap = captures.get(1);
                    if method_cap.is_none() {
                        error!("Missing method in request format");
                        return Err(DeboaError::Request(RequestError::Parse {
                            message: "Missing method in request format".into(),
                        }));
                    }
                    let url_cap = captures.get(2);
                    if url_cap.is_none() {
                        error!("Missing url in request format");
                        return Err(DeboaError::Request(RequestError::Parse {
                            message: "Missing url in request format".into(),
                        }));
                    }
                    method = method_cap
                        .unwrap()
                        .as_str()
                        .to_string();
                    url = url_cap
                        .unwrap()
                        .as_str()
                        .to_string();
                    continue;
                }

                let header = line.split_once(':');
                if let Some(header) = header {
                    let header_name = HeaderName::from_bytes(
                        header
                            .0
                            .trim()
                            .as_bytes(),
                    )
                    .map_err(|_| {
                        error!("Invalid header name");
                        DeboaError::Request(RequestError::Parse {
                            message: "Invalid header name".into(),
                        })
                    })?;

                    let header_value = HeaderValue::from_bytes(
                        header
                            .1
                            .trim()
                            .as_bytes(),
                    )
                    .map_err(|_| {
                        error!("Invalid header value");
                        DeboaError::Request(RequestError::Parse {
                            message: "Invalid header value".into(),
                        })
                    })?;

                    headers.insert(header_name, header_value);
                    continue;
                }
            }

            if line.is_empty() && !url.is_empty() && !headers.is_empty() {
                is_reading_body = true;
                continue;
            }

            if is_reading_body {
                body.extend_from_slice(line.as_bytes());
            }
        }

        let url = url.parse_url()?;
        if headers
            .get(header::HOST)
            .is_none()
        {
            let authority = url.authority();
            headers.insert(header::HOST, HeaderValue::from_str(authority).unwrap());
        }

        Ok(DeboaRequest {
            url: Arc::new(url),
            headers,
            cookies: None,
            retries: 0,
            method: method
                .parse::<http::Method>()
                .unwrap(),
            body: body.into(),
        })
    }
}

impl AsRef<DeboaRequest> for DeboaRequest {
    fn as_ref(&self) -> &DeboaRequest {
        self
    }
}

impl AsMut<DeboaRequest> for DeboaRequest {
    fn as_mut(&mut self) -> &mut DeboaRequest {
        self
    }
}

impl DeboaRequest {
    /// Allow make a request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    /// * `method` - The method to be used.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    /// # Panics
    ///
    /// * If URL is invalid
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use deboa::request::post;
    ///
    /// let request = at("https://jsonplaceholder.typicode.com/posts", http::Method::POST)?
    ///   .header("Content-Type", "application/json")
    ///   .raw_body(b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}")
    ///   .build()?;
    /// let response = request.send_with(&mut client).await?;
    /// assert_eq!(response.status(), 201);
    /// ```
    ///
    #[inline]
    pub fn at<T: IntoUrl>(url: T, method: http::Method) -> Result<DeboaRequestBuilder> {
        let parsed_url = url.into_url();
        if let Err(e) = parsed_url {
            error!("Failed to parse url: {}", e);
            return Err(DeboaError::Request(RequestError::UrlParse { message: e.to_string() }));
        }

        let url = parsed_url.unwrap();
        let authority = url.authority();
        let mut headers = HeaderMap::new();
        headers.insert(header::HOST, HeaderValue::from_str(authority).unwrap());

        Ok(DeboaRequestBuilder {
            url: url.into(),
            headers,
            cookies: None,
            retries: 0,
            method,
            body: Arc::new([]),
            form: None,
        })
    }

    /// Allow make a GET request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    /// # Panics
    ///
    /// * If URL is invalid
    ///
    #[inline]
    pub fn from<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        DeboaRequest::at(url, Method::GET)
    }

    /// Allow make a POST request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    /// # Panics
    ///
    /// * If URL is invalid
    ///
    #[inline]
    pub fn to<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        DeboaRequest::at(url, Method::POST)
    }

    /// Allow make a GET request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    /// # Panics
    ///
    /// * If URL is invalid
    ///
    #[inline]
    pub fn get<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::from(url)?.method(Method::GET))
    }

    /// Allow make a POST request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    /// # Panics
    ///
    /// * If URL is invalid
    ///
    #[inline]
    pub fn post<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::to(url)?.method(Method::POST))
    }

    /// Allow make a PUT request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    /// # Panics
    ///
    /// * If URL is invalid
    ///
    #[inline]
    pub fn put<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::to(url)?.method(Method::PUT))
    }

    /// Allow make a PATCH request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    /// # Panics
    ///
    /// * If URL is invalid
    ///
    #[inline]
    pub fn patch<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::to(url)?.method(Method::PATCH))
    }

    /// Allow make a DELETE request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    /// # Panics
    ///
    /// * If URL is invalid
    ///
    #[inline]
    pub fn delete<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::from(url)?.method(Method::DELETE))
    }

    /// Allow change request method at any time.
    ///
    /// # Arguments
    ///
    /// * `method` - The new method.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    #[inline]
    pub fn set_method(&mut self, method: http::Method) -> &mut Self {
        self.method = method;
        self
    }

    /// Get request method at any time.
    ///
    /// # Returns
    ///
    /// * `http::Method` - The method.
    ///
    #[inline]
    pub fn method(&self) -> &http::Method {
        &self.method
    }

    /// Allow change request url at any time.
    ///
    /// # Arguments
    ///
    /// * `url` - The new url.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self>` - The request.
    ///
    #[inline]
    pub fn set_url<T: IntoUrl>(&mut self, url: T) -> Result<&mut Self> {
        let parsed_url = url.into_url();
        if let Err(e) = parsed_url {
            error!("Failed to parse url: {}", e);
            return Err(DeboaError::Request(RequestError::UrlParse { message: e.to_string() }));
        }

        let parsed_url = parsed_url.unwrap();
        if self.has_header(&header::HOST) {
            self.headers
                .remove(&header::HOST);
            self.add_header(HOST, parsed_url.authority());
        }

        self.url = parsed_url.into();
        Ok(self)
    }

    /// Allow get request url at any time.
    ///
    /// # Returns
    ///
    /// * `Url` - The url.
    ///
    #[inline]
    pub fn url(&self) -> Arc<Url> {
        Arc::clone(&self.url)
    }

    /// Allow get retries at any time.
    ///
    /// # Returns
    ///
    /// * `u32` - The retries.
    ///
    #[inline]
    pub fn retries(&self) -> u32 {
        self.retries
    }

    /// Allow get request headers at any time.
    ///
    /// # Returns
    ///
    /// * `HeaderMap` - The headers.
    ///
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Return mutable headers
    ///
    /// # Returns
    ///
    /// * `&mut HeaderMap` - The headers.
    ///
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Allow add header at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key to add.
    /// * `value` - The header value to add.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    #[inline]
    pub fn add_header(&mut self, key: HeaderName, value: &str) -> &mut Self {
        self.headers
            .insert(key, HeaderValue::from_str(value).unwrap());
        self
    }

    /// Allow check if header exists at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key to check.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the header exists, false otherwise.
    ///
    #[inline]
    fn has_header(&self, key: &HeaderName) -> bool {
        self.headers
            .contains_key(key)
    }

    /// Allow add bearer auth at any time.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to be used in the Authorization header.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    #[inline]
    pub fn add_bearer_auth(&mut self, token: &str) -> &mut Self {
        let auth = format!("Bearer {token}");
        self.add_header(header::AUTHORIZATION, &auth);
        self
    }

    /// Allow add basic auth at any time.
    ///
    /// # Arguments
    ///
    /// * `username` - The username.
    /// * `password` - The password.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    #[inline]
    pub fn add_basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        let auth = format!("Basic {}", STANDARD.encode(format!("{username}:{password}")));
        self.add_header(header::AUTHORIZATION, &auth);
        self
    }

    /// Allow add cookie at any time.
    ///
    /// # Arguments
    ///
    /// * `cookie` - The cookie to be added.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    #[inline]
    pub fn add_cookie(&mut self, cookie: DeboaCookie) -> &mut Self {
        if let Some(cookies) = &mut self.cookies {
            cookies.insert(
                cookie
                    .name()
                    .to_string(),
                cookie,
            );
        } else {
            self.cookies = Some(HashMap::from([(
                cookie
                    .name()
                    .to_string(),
                cookie,
            )]));
        }
        self
    }

    /// Allow remove cookie at any time.
    ///
    /// # Arguments
    ///
    /// * `name` - The cookie name.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    #[inline]
    pub fn remove_cookie(&mut self, name: &str) -> &mut Self {
        if let Some(cookies) = &mut self.cookies {
            cookies.remove(name);
        }
        self
    }

    /// Allow check if cookie exists at any time.
    ///
    /// # Arguments
    ///
    /// * `name` - The cookie name to check.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the cookie exists, false otherwise.
    ///
    #[inline]
    pub fn has_cookie(&self, name: &str) -> bool {
        if let Some(cookies) = &self.cookies {
            cookies.contains_key(name)
        } else {
            false
        }
    }

    /// Allow add cookies at any time.
    ///
    /// # Arguments
    ///
    /// * `cookies` - The cookies to be added.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    #[inline]
    pub fn set_cookies(&mut self, cookies: HashMap<String, DeboaCookie>) -> &mut Self {
        self.cookies = Some(cookies);
        self
    }

    /// Allow get cookies at any time.
    ///
    /// # Returns
    ///
    /// * `Option<&HashMap<String, DeboaCookie>>` - The cookies.
    ///
    #[inline]
    pub fn cookies(&self) -> Option<&HashMap<String, DeboaCookie>> {
        self.cookies
            .as_ref()
    }

    /// Allow set form at any time.
    ///
    /// # Arguments
    ///
    /// * `form` - The form to be set.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    #[inline]
    pub fn set_form(&mut self, form: Form) -> &mut Self {
        let (content_type, body) = match form {
            Form::EncodedForm(form) => (form.content_type(), form.build()),
            Form::MultiPartForm(form) => (form.content_type(), form.build()),
        };
        self.add_header(header::CONTENT_TYPE, &content_type);
        self.set_raw_body(&body);
        self
    }

    /// Allow set text body at any time.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to be set.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    #[inline]
    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.set_raw_body(text.as_bytes());
        self
    }

    /// Allow set body at any time.
    ///
    /// # Arguments
    ///
    /// * `body` - The body to be set.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    #[inline]
    pub fn set_raw_body(&mut self, body: &[u8]) -> &mut Self {
        self.add_header(
            header::CONTENT_LENGTH,
            &body
                .len()
                .to_string(),
        );
        self.body = body.into();
        self
    }

    /// Allow get raw body at any time.
    ///
    /// # Returns
    ///
    /// * `&Vec<u8>` - The raw body.
    ///
    #[inline]
    pub fn raw_body(&self) -> &[u8] {
        &self.body
    }

    /// Allow set body at any time.
    ///
    /// # Arguments
    ///
    /// * `body_type` - The body type to be set.
    /// * `body` - The body to be set.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self>` - The request.
    ///
    #[inline]
    pub fn set_body_as<T: RequestBody, B: Serialize>(
        &mut self,
        body_type: T,
        body: B,
    ) -> Result<&mut Self> {
        body_type.register_content_type(self);
        let body = body_type.serialize(body)?;
        self.set_raw_body(&body);
        Ok(self)
    }
}
mod private {
    pub trait IntoRequestSealed {}
    pub trait IntoHeadersSealed {}
    pub trait MethodExtSealed {}
}

impl private::IntoRequestSealed for DeboaRequest {}

impl private::IntoRequestSealed for &str {}

impl private::IntoRequestSealed for String {}

impl private::IntoRequestSealed for Url {}

impl private::IntoHeadersSealed for HeaderMap {}

impl private::IntoHeadersSealed for Vec<(HeaderName, String)> {}

impl private::IntoHeadersSealed for Vec<(String, String)> {}

impl<'a> private::IntoHeadersSealed for Vec<(&'a str, &'a str)> {}

impl private::MethodExtSealed for Method {}

impl private::MethodExtSealed for &str {}
