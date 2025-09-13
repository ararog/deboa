use std::fs::write;
use std::{fmt::Debug, sync::Arc};

use serde::Deserialize;

use crate::{client::serde::ResponseBody, errors::DeboaError};

#[derive(PartialEq)]
pub struct DeboaResponse {
    status: http::StatusCode,
    headers: Arc<http::HeaderMap>,
    body: Arc<Vec<u8>>,
}

impl Debug for DeboaResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeboaResponse")
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
    /// * `status` - The status code of the response.
    /// * `headers` - The headers of the response.
    /// * `body` - The body of the response.
    ///
    pub fn new(status: http::StatusCode, headers: http::HeaderMap, body: &[u8]) -> Self {
        Self {
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
    pub fn status(&self) -> http::StatusCode {
        self.status
    }

    /// Allow get headers at any time.
    ///
    /// # Returns
    ///
    /// * `&http::HeaderMap` - The headers of the response.
    ///
    pub fn headers(&self) -> &http::HeaderMap {
        &self.headers
    }

    /// Allow set raw body at any time.
    ///
    /// # Arguments
    ///
    /// * `body` - The body to be set.
    ///
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
    /// * `Result<B, DeboaError>` - The body or error.
    ///
    pub fn body_as<T: ResponseBody, B: for<'a> Deserialize<'a>>(&self, body_type: T) -> Result<B, DeboaError> {
        let result = body_type.deserialize::<B>(self.body.to_vec())?;
        Ok(result)
    }

    /// Allow get raw body at any time.
    ///
    /// # Returns
    ///
    /// * `&[u8]` - The raw body of the response.
    ///
    pub fn raw_body(&self) -> &[u8] {
        &self.body
    }

    /// Allow get text body at any time.
    ///
    /// # Returns
    ///
    /// * `Result<String, DeboaError>` - The text body or error.
    ///
    pub fn text(&self) -> Result<String, DeboaError> {
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
    /// * `Result<(), DeboaError>` - The result or error.
    ///
    pub fn to_file(&self, path: &str) -> Result<(), DeboaError> {
        let result = write(path, &*self.body);
        if let Err(e) = result {
            return Err(DeboaError::Io { message: e.to_string() });
        }
        Ok(())
    }
}
