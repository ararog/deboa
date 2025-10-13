use std::fs::write;
use std::marker::PhantomData;
use std::{fmt::Debug, sync::Arc};

use http::{header, HeaderMap, HeaderName};
use http_body_util::{BodyExt, Either, Full};
use hyper::body::{Bytes, Incoming};
use serde::Deserialize;

use crate::cookie::DeboaCookie;
use crate::{client::serde::ResponseBody, errors::DeboaError, Result};
use url::Url;

pub enum DeboaBody {
    Incoming(Incoming),
    Full(Full<Bytes>),
}

pub trait ResponseState {}
impl ResponseState for IncomingResponse {}
impl ResponseState for UpgradeResponse {}

pub struct BaseResponse<S: ResponseState> {
    state: Box<S>,
    marker: PhantomData<S>
} 

pub struct IncomingResponse {
    url: Url,
    status: http::StatusCode,
    headers: Arc<http::HeaderMap>,
    body: DeboaBody,
}

pub struct UpgradeResponse {
    stream: String,
}

impl BaseResponse<IncomingResponse> {

    fn url(&self) -> &Url {
        &self.state.url
    }

    fn status(&self) -> http::StatusCode {
        self.state.status
    }

    fn headers(&self) -> &http::HeaderMap {
        &self.state.headers
    }

    fn body(&self) -> &DeboaBody {
        &self.state.body
    }
}

impl BaseResponse<UpgradeResponse> {
    fn stream(&self) -> &String {
        &self.state.stream
    }
}

pub trait IntoBody {
    fn into_body(self) -> Either<Incoming, Full<Bytes>>;
}

impl IntoBody for Incoming {
    fn into_body(self) -> Either<Incoming, Full<Bytes>> {
        Either::Left(self)
    }
}

impl IntoBody for &[u8] {
    fn into_body(self) -> Either<Incoming, Full<Bytes>> {
        Either::Right(Full::<Bytes>::from(self.to_vec()))
    }
}

impl IntoBody for Vec<u8> {
    fn into_body(self) -> Either<Incoming, Full<Bytes>> {
        Either::Right(Full::<Bytes>::from(self))
    }
}

impl IntoBody for Bytes {
    fn into_body(self) -> Either<Incoming, Full<Bytes>> {
        Either::Right(Full::<Bytes>::new(self))
    }
}

impl IntoBody for Full<Bytes> {
    fn into_body(self) -> Either<Incoming, Full<Bytes>> {
        Either::Right(self)
    }
}

pub struct DeboaResponseBuilder {
    url: Url,
    status: http::StatusCode,
    headers: Arc<http::HeaderMap>,
    body: Either<Incoming, Full<Bytes>>,
}

impl DeboaResponseBuilder {
    pub fn status(mut self, status: http::StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn headers(mut self, headers: http::HeaderMap) -> Self {
        self.headers = headers.into();
        self
    }

    pub fn body<B: IntoBody>(mut self, body: B) -> Self {
        self.body = body.into_body();
        self
    }

    pub fn build(self) -> DeboaResponse {
        DeboaResponse {
            url: self.url,
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
}

/// Struct that represents the response.
///
/// # Fields
///
/// * `status` - The status code of the response.
/// * `headers` - The headers of the response.
/// * `body` - The body of the response.
pub struct DeboaResponse {
    url: Url,
    status: http::StatusCode,
    headers: Arc<http::HeaderMap>,
    body: Either<Incoming, Full<Bytes>>,
}

impl Debug for DeboaResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeboaResponse")
            .field("url", &self.url)
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &self.body)
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
    /// * `status` - The status code of the response.
    /// * `headers` - The headers of the response.
    /// * `body` - The body of the response.
    ///
    pub fn new<B>(url: Url, status: http::StatusCode, headers: http::HeaderMap, body: B) -> Self
    where
        B: IntoBody,
    {
        Self {
            url,
            status,
            headers: headers.into(),
            body: body.into_body(),
        }
    }

    #[inline]
    pub fn builder(url: Url) -> DeboaResponseBuilder {
        DeboaResponseBuilder {
            url,
            status: http::StatusCode::OK,
            headers: HeaderMap::new().into(),
            body: Either::Right(Full::new(Bytes::new())),
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
        self.status
    }

    /// Allow get mutable status code at any time.
    ///
    /// # Returns
    ///
    /// * `&mut http::StatusCode` - The status code of the response.
    ///
    #[inline]
    pub fn status_mut(&mut self) -> &mut http::StatusCode {
        &mut self.status
    }

    /// Allow get headers at any time.
    ///
    /// # Returns
    ///
    /// * `&http::HeaderMap` - The headers of the response.
    ///
    #[inline]
    pub fn headers(&self) -> &http::HeaderMap {
        &self.headers
    }

    /// Allow get mutable headers at any time.
    ///
    /// # Returns
    ///
    /// * `&mut Arc<http::HeaderMap>` - The headers of the response.
    ///
    #[inline]
    pub fn headers_mut(&mut self) -> &mut Arc<http::HeaderMap> {
        &mut self.headers
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
        let header_value = self.headers.get(header_name);
        if header_value.is_none() {
            return Err(DeboaError::InvalidHeader {
                message: "Header is missing".to_string(),
            });
        }
        let header_value = header_value.unwrap();
        let header_value = header_value.to_str();
        if let Err(e) = header_value {
            return Err(DeboaError::InvalidHeader {
                message: format!("Failed to read {}:: {}", header_name, e),
            });
        }
        Ok(header_value.unwrap().to_string())
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
            return Err(DeboaError::InvalidHeader {
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
        let view = self.headers.get_all(header::SET_COOKIE);
        let cookies = view
            .into_iter()
            .map(|cookie| {
                let cookie = cookie.to_str();
                if let Ok(cookie) = cookie {
                    DeboaCookie::parse_from_header(cookie)
                } else {
                    Err(DeboaError::Cookie {
                        message: "Invalid cookie header".to_string(),
                    })
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

    /// Allow get stream body at any time.
    ///
    /// # Returns
    ///
    /// * `Either<Incoming, Full<Bytes>>` - The stream body of the response.
    ///
    #[inline]
    pub fn stream(&mut self) -> &mut Either<Incoming, Full<Bytes>> {
        &mut self.body
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
        while let Some(chunk) = self.body.frame().await {
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
        self.body = Either::Right(Full::<Bytes>::from(body));
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
    #[inline]
    pub async fn body_as<T: ResponseBody, B: for<'a> Deserialize<'a>>(
        &mut self,
        body_type: T,
    ) -> Result<B> {
        let result = body_type.deserialize::<B>(self.raw_body().await)?;
        Ok(result)
    }

    /// Returns the response body as a string, consuming body.
    /// Useful for small responses. For larger responses, consider using `stream`.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The text body or error.
    ///
    #[inline]
    pub async fn text(&mut self) -> Result<String> {
        let body = self.raw_body().await;
        Ok(String::from_utf8_lossy(&body).to_string())
    }

    /// Save response body to file, consuming body.
    /// Useful for small responses. For larger responses, consider using
    /// ToFile trait available on stream feature of deboa-extras crate.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to save the file.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - The result or error.
    ///
    pub async fn to_file(&mut self, path: &str) -> Result<()> {
        let body = self.raw_body().await;
        let result = write(path, body);
        if let Err(e) = result {
            return Err(DeboaError::Io {
                message: e.to_string(),
            });
        }
        Ok(())
    }
}
