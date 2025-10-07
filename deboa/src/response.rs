use std::fs::write;
use std::{fmt::Debug, sync::Arc};

use http::header;
use serde::Deserialize;

use crate::cookie::DeboaCookie;
use crate::{client::serde::ResponseBody, errors::DeboaError, Result};
use url::Url;

#[derive(PartialEq)]
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
    body: Arc<Vec<u8>>,
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
    pub fn new(url: Url, status: http::StatusCode, headers: http::HeaderMap, body: &[u8]) -> Self {
        Self {
            url,
            status,
            headers: headers.into(),
            body: body.to_vec().into(),
        }
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

    /// Allow set raw body at any time.
    ///
    /// # Arguments
    ///
    /// * `body` - The body to be set.
    ///
    #[inline]
    pub fn set_raw_body(&mut self, body: &[u8]) {
        self.body = body.to_vec().into();
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
    pub fn body_as<T: ResponseBody, B: for<'a> Deserialize<'a>>(&self, body_type: T) -> Result<B> {
        let result = body_type.deserialize::<B>(self.body.to_vec())?;
        Ok(result)
    }

    /// Allow get raw body at any time.
    ///
    /// # Returns
    ///
    /// * `&[u8]` - The raw body of the response.
    ///
    #[inline]
    pub fn raw_body(&self) -> &[u8] {
        &self.body
    }

    /// Allow get text body at any time.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The text body or error.
    ///
    #[inline]
    pub fn text(&self) -> Result<String> {
        Ok(String::from_utf8_lossy(&self.body).to_string())
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
    pub fn to_file(&self, path: &str) -> Result<()> {
        let result = write(path, &*self.body);
        if let Err(e) = result {
            return Err(DeboaError::Io {
                message: e.to_string(),
            });
        }
        Ok(())
    }
}
