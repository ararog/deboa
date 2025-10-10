use std::fs::write;
use std::{fmt::Debug, sync::Arc};

use http::header;
use http_body_util::{BodyExt, Either, Full};
use hyper::body::{Bytes, Incoming};
use serde::Deserialize;

use crate::cookie::DeboaCookie;
use crate::{client::serde::ResponseBody, errors::DeboaError, Result};
use url::Url;

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

    /// Allow get raw body at any time.
    ///
    /// # Returns
    ///
    /// * `&[u8]` - The raw body of the response.
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

    /// Allow get body at any time.
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

    /// Allow get text body at any time.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The text body or error.
    ///
    #[inline]
    pub async fn text(&mut self) -> Result<String> {
        Ok(String::from_utf8_lossy(&self.raw_body().await).to_string())
    }

    /// Allow save response body to file at any time.
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
        let result = write(path, &*self.raw_body().await);
        if let Err(e) = result {
            return Err(DeboaError::Io {
                message: e.to_string(),
            });
        }
        Ok(())
    }
}
