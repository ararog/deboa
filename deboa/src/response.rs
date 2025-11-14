use std::fmt::Debug;
use std::fs::write;

use http::{header, HeaderName, HeaderValue, Response};
use http_body_util::{BodyDataStream, BodyExt, Either, Full};
use hyper::body::{Bytes, Incoming};
use hyper::upgrade::on;
#[cfg(feature = "tokio-rt")]
use hyper_util::rt::TokioIo;
use serde::Deserialize;
#[cfg(feature = "smol-rt")]
use smol_hyper::rt::FuturesIo;

use crate::cookie::DeboaCookie;
use crate::errors::{ConnectionError, IoError};
use crate::{client::serde::ResponseBody, errors::DeboaError, Result};
use url::Url;

pub type DeboaBody = Either<Incoming, Full<Bytes>>;

pub trait IntoBody {
    fn into_body(self) -> DeboaBody;
}

impl IntoBody for Incoming {
    fn into_body(self) -> DeboaBody {
        DeboaBody::Left(self)
    }
}

impl IntoBody for &[u8] {
    fn into_body(self) -> DeboaBody {
        DeboaBody::Right(Full::<Bytes>::from(self.to_vec()))
    }
}

impl IntoBody for Vec<u8> {
    fn into_body(self) -> DeboaBody {
        DeboaBody::Right(Full::<Bytes>::from(self))
    }
}

impl IntoBody for Bytes {
    fn into_body(self) -> DeboaBody {
        DeboaBody::Right(Full::<Bytes>::new(self))
    }
}

impl IntoBody for Full<Bytes> {
    fn into_body(self) -> DeboaBody {
        DeboaBody::Right(self)
    }
}

pub struct DeboaResponseBuilder {
    url: Url,
    inner: Response<DeboaBody>,
}

impl DeboaResponseBuilder {
    /// Allow set response status at any time.
    ///
    /// # Arguments
    ///
    /// * `status` - The new status.
    ///
    /// # Returns
    ///
    /// * `Self` - The response builder.
    ///
    pub fn status(mut self, status: http::StatusCode) -> Self {
        *self
            .inner
            .status_mut() = status;
        self
    }

    /// Allow set response headers at any time.
    ///
    /// # Arguments
    ///
    /// * `headers` - The new headers.
    ///
    /// # Returns
    ///
    /// * `Self` - The response builder.
    ///
    pub fn headers(mut self, headers: http::HeaderMap) -> Self {
        *self
            .inner
            .headers_mut() = headers;
        self
    }

    /// Allow set response header at any time.
    ///
    /// # Arguments
    ///
    /// * `name` - The header name.
    /// * `value` - The header value.
    ///
    /// # Returns
    ///
    /// * `Self` - The response builder.
    ///
    pub fn header(mut self, name: HeaderName, value: &str) -> Self {
        let header_value = HeaderValue::from_str(value);
        if let Ok(header_value) = header_value {
            self.inner
                .headers_mut()
                .insert(name, header_value);
        }
        self
    }

    /// Allow set response body at any time.
    ///
    /// # Arguments
    ///
    /// * `body` - The new body.
    ///
    /// # Returns
    ///
    /// * `Self` - The response builder.
    ///
    pub fn body<B: IntoBody>(mut self, body: B) -> Self {
        *self
            .inner
            .body_mut() = body.into_body();
        self
    }

    /// Build the response. Consuming the builder.
    ///
    /// # Returns
    ///
    /// * `DeboaResponse` - The response.
    ///
    pub fn build(self) -> DeboaResponse {
        DeboaResponse { url: self.url, inner: self.inner }
    }
}

/// Represents an HTTP response received from a server.
///
/// `DeboaResponse` provides methods to access and manipulate the response status,
/// headers, and body. It supports various ways to consume the response body,
/// including streaming and buffered access.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```no_run
/// use deboa::request::get;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut client = deboa::Deboa::new();
/// let response = get("https://httpbin.org/get")?.send_with(&mut client).await?;
///
/// println!("Status: {}", response.status());
/// println!("Headers: {:?}", response.headers());
/// println!("Body: {}", response.text().await?);
/// # Ok(()) }
/// ```
///
/// ## JSON Deserialization
///
/// ```compile_fail
/// use deboa::request::get;
/// use deboa::client::serde::json::JsonBody;
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct Data {
///     origin: String,
///     url: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut client = deboa::Deboa::new();
///     let response = get("https://httpbin.org/get")?.with(&mut client).await?;
///     let data: Data = response.body_as(JsonBody).await?;
///     println!("Origin: {}", data.origin);
///     Ok(())
/// }
/// ```
///
/// # Fields
///
/// * `url` - The URL that the response came from
/// * `inner` - The underlying HTTP response
/// * `status` - The HTTP status code
/// * `headers` - The response headers
/// * `body` - The response body (can be streamed or buffered)
pub struct DeboaResponse {
    url: Url,
    inner: Response<DeboaBody>,
}

impl Debug for DeboaResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeboaResponse")
            .field("url", &self.url)
            .field("status", &self.inner.status())
            .field("headers", &self.inner.headers())
            .field("body", &self.inner.body())
            .finish()
    }
}

impl AsRef<DeboaResponse> for DeboaResponse {
    fn as_ref(&self) -> &DeboaResponse {
        self
    }
}

impl AsMut<DeboaResponse> for DeboaResponse {
    fn as_mut(&mut self) -> &mut DeboaResponse {
        self
    }
}

impl DeboaResponse {
    /// Allow create a new DeboaResponse instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The url of the response.
    /// * `inner` - The inner response.
    ///
    pub fn new(url: Url, inner: Response<DeboaBody>) -> Self {
        Self { url, inner }
    }

    #[inline]
    pub fn builder(url: Url) -> DeboaResponseBuilder {
        DeboaResponseBuilder {
            url,
            inner: Response::new(DeboaBody::Right(Full::<Bytes>::from(Bytes::new()))),
        }
    }

    /// Allow get url at any time.
    ///
    /// # Returns
    ///
    /// * `&Url` - The url of the response.
    ///
    #[inline]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Allow get status code at any time.
    ///
    /// # Returns
    ///
    /// * `http::StatusCode` - The status code of the response.
    ///
    #[inline]
    pub fn status(&self) -> http::StatusCode {
        self.inner.status()
    }

    /// Allow get mutable status code at any time.
    ///
    /// # Returns
    ///
    /// * `&mut http::StatusCode` - The status code of the response.
    ///
    #[inline]
    pub fn status_mut(&mut self) -> &mut http::StatusCode {
        self.inner
            .status_mut()
    }

    /// Allow get headers at any time.
    ///
    /// # Returns
    ///
    /// * `&http::HeaderMap` - The headers of the response.
    ///
    #[inline]
    pub fn headers(&self) -> &http::HeaderMap {
        self.inner.headers()
    }

    /// Allow get mutable headers at any time.
    ///
    /// # Returns
    ///
    /// * `&mut http::HeaderMap` - The headers of the response.
    ///
    #[inline]
    pub fn headers_mut(&mut self) -> &mut http::HeaderMap {
        self.inner
            .headers_mut()
    }

    /// Allow get header value at any time.
    /// It will return an error if the Content-Type header is missing or
    /// has an invalid value.
    ///
    /// # Arguments
    ///
    /// * `header` - The header name.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The header value.
    ///
    #[inline]
    fn header_value(&self, header: HeaderName) -> Result<String> {
        let header_name = header.as_str();
        let header_value = self
            .headers()
            .get(header_name);
        if header_value.is_none() {
            return Err(DeboaError::Header { message: "Header is missing".to_string() });
        }
        let header_value = header_value.unwrap();
        let header_value = header_value.to_str();
        if let Err(e) = header_value {
            return Err(DeboaError::Header {
                message: format!("Failed to read {}:: {}", header_name, e),
            });
        }
        Ok(header_value
            .unwrap()
            .to_string())
    }

    /// Allow get the length of the response body.
    /// It will return an error if the Content-Length header is missing or
    /// has an invalid value or if it fails to parse the value.
    ///
    /// # Returns
    ///
    /// * `Result<u64>` - The length of the response body.
    ///
    #[inline]
    pub fn content_length(&self) -> Result<u64> {
        let header = self.header_value(header::CONTENT_LENGTH)?;
        let header = header.parse::<u64>();
        if let Err(e) = header {
            return Err(DeboaError::Header {
                message: format!("Failed to parse content-length: {}", e),
            });
        }

        Ok(header.unwrap())
    }

    /// Allow get the content type of the response body.
    /// It will return an error if the Content-Type header is missing or
    /// has an invalid value.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The content type of the response body.
    ///
    #[inline]
    pub fn content_type(&self) -> Result<String> {
        let header = self.header_value(header::CONTENT_TYPE)?;
        Ok(header)
    }

    /// Retrieves cookies from response headers. If cookies are not found, returns None.
    /// Please note that this method will parse the cookies from the response headers.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<DeboaCookie>>` - The cookies of the response.
    ///
    #[inline]
    pub fn cookies(&self) -> Result<Option<Vec<DeboaCookie>>> {
        let view = self
            .headers()
            .get_all(header::SET_COOKIE);
        let cookies = view
            .into_iter()
            .map(|cookie| {
                let cookie = cookie.to_str();
                if let Ok(cookie) = cookie {
                    DeboaCookie::parse_from_header(cookie)
                } else {
                    Err(DeboaError::Cookie { message: "Invalid cookie header".to_string() })
                }
            })
            .collect::<Result<Vec<DeboaCookie>>>()
            .unwrap();

        if cookies.is_empty() {
            Ok(None)
        } else {
            Ok(Some(cookies))
        }
    }

    /// Allow get inner response body at any time.
    ///
    /// # Returns
    ///
    /// * `DeboaBody` - The inner response body.
    ///
    #[inline]
    pub fn inner_body(self) -> DeboaBody {
        self.inner
            .into_body()
    }

    /// Returns the response body as a vector of bytes, consuming body.
    /// Useful for small responses. For larger responses, consider using `stream`.
    ///
    /// # Returns
    ///
    /// * `Vec<u8>` - The raw body of the response.
    ///
    #[inline]
    pub async fn raw_body(&mut self) -> Vec<u8> {
        let mut data = Vec::<u8>::new();
        while let Some(chunk) = self
            .inner
            .body_mut()
            .frame()
            .await
        {
            let frame = chunk.unwrap();
            if let Some(bytes) = frame.data_ref() {
                data.extend_from_slice(bytes);
            }
        }
        data
    }

    /// Allow set raw body at any time.
    ///
    /// # Arguments
    ///
    /// * `body` - The body to be set.
    ///
    #[inline]
    pub fn set_raw_body(&mut self, body: Bytes) {
        *self
            .inner
            .body_mut() = Either::Right(Full::<Bytes>::from(body));
    }

    /// Allow get stream body at any time.
    ///
    /// # Returns
    ///
    /// * `Either<Incoming, Full<Bytes>>` - The stream body of the response.
    ///
    #[inline]
    pub fn stream(self) -> BodyDataStream<http::Response<DeboaBody>> {
        self.inner
            .into_data_stream()
    }

    /// Returns the response body as a deserialized type, consuming body.
    /// Useful for small responses. For larger responses, consider using `stream`.
    ///
    /// # Arguments
    ///
    /// * `body_type` - The body type to be deserialized.
    ///
    /// # Returns
    ///
    /// * `Result<B>` - The body or error.
    ///
    /// # Examples
    ///
    /// ```compile_fail
    /// use deboa::request::get;
    /// use deboa_extras::http::serde::json::JsonBody;
    ///
    /// let response = get("https://jsonplaceholder.typicode.com/posts")?.send_with(client).await?;
    /// let posts: Vec<Post> = response.body_as(JsonBody).await?;
    /// ```
    ///
    #[inline]
    pub async fn body_as<T: ResponseBody, B: for<'a> Deserialize<'a>>(
        mut self,
        body_type: T,
    ) -> Result<B> {
        let result = body_type.deserialize::<B>(
            self.raw_body()
                .await,
        )?;
        Ok(result)
    }

    /// Returns the response body as a string, consuming body.
    /// Useful for small responses. For larger responses, consider using `stream`.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The text body or error.
    ///
    /// # Examples
    ///
    /// ```compile_fail
    /// use deboa::request::get;
    ///
    /// let response = get("https://jsonplaceholder.typicode.com/posts")?.send_with(client).await?;
    /// let text = response.text().await?;
    /// ```
    ///
    #[inline]
    pub async fn text(mut self) -> Result<String> {
        let body = self
            .raw_body()
            .await;
        Ok(String::from_utf8_lossy(&body).to_string())
    }

    /// Save response body to file, consuming body.
    /// Useful for small responses. For larger responses, consider using
    /// ToFile trait available on utils feature of deboa-extras crate.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to save the file.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - The result or error.
    ///
    /// # Examples
    ///
    /// ```compile_fail
    /// use deboa::request::get;
    ///
    /// let response = get("https://jsonplaceholder.typicode.com/posts")?.send_with(client).await?;
    /// response.to_file("posts.json").await?;
    /// ```
    ///
    pub async fn to_file(mut self, path: &str) -> Result<()> {
        let body = self
            .raw_body()
            .await;
        let result = write(path, body);
        if let Err(e) = result {
            return Err(DeboaError::Io(IoError::File { message: e.to_string() }));
        }
        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    pub async fn upgrade(self) -> Result<hyper_util::rt::TokioIo<hyper::upgrade::Upgraded>> {
        if self.inner.version() != http::Version::HTTP_11 {
            return Err(DeboaError::Connection(ConnectionError::Upgrade {
                message: "Upgrade is only supported for HTTP/1.1".to_string(),
            }));
        }

        let upgrade = on(self.inner).await;
        if let Err(e) = upgrade {
            return Err(DeboaError::Connection(ConnectionError::Upgrade {
                message: e.to_string(),
            }));
        }
        Ok(TokioIo::new(upgrade.unwrap()))
    }

    #[cfg(feature = "smol-rt")]
    pub async fn upgrade(self) -> Result<FuturesIo<hyper::upgrade::Upgraded>> {
        if self.inner.version() != http::Version::HTTP_11 {
            return Err(DeboaError::Io {
                message: "Upgrade is only supported for HTTP/1.1".to_string(),
            });
        }

        let upgrade = on(self.inner).await;
        if let Err(e) = upgrade {
            return Err(DeboaError::Io { message: e.to_string() });
        }
        Ok(FuturesIo::new(upgrade.unwrap()))
    }
}
