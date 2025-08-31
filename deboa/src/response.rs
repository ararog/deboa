use std::fmt::Debug;
use std::fs::write;

use http::{HeaderMap, StatusCode};

use crate::errors::DeboaError;

#[derive(PartialEq)]
pub struct DeboaResponse {
    pub(crate) status: StatusCode,
    headers: HeaderMap,
    body: Vec<u8>,
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

impl DeboaResponse {
    /// Allow create a new DeboaResponse instance.
    ///
    /// # Arguments
    ///
    /// * `status` - The status code of the response.
    /// * `headers` - The headers of the response.
    /// * `body` - The body of the response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::response::DeboaResponse;
    /// use http::{HeaderMap, StatusCode};
    ///
    /// let response = DeboaResponse::new(StatusCode::OK, HeaderMap::new(), Vec::new());
    /// ```
    ///
    pub fn new(status: StatusCode, headers: HeaderMap, body: Vec<u8>) -> Self {
        Self { status, headers, body }
    }

    /// Allow get status code at any time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::response::DeboaResponse;
    /// use http::{HeaderMap, StatusCode};
    ///
    /// let response = DeboaResponse::new(StatusCode::OK, HeaderMap::new(), Vec::new());
    /// assert_eq!(response.status(), StatusCode::OK);
    /// ```
    ///
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Allow get headers at any time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::response::DeboaResponse;
    /// use http::{HeaderMap, StatusCode};
    ///
    /// let response = DeboaResponse::new(StatusCode::OK, HeaderMap::new(), Vec::new());
    /// assert_eq!(response.headers(), HeaderMap::new());
    /// ```
    ///
    pub fn headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    /// Allow set raw body at any time.
    ///
    /// # Arguments
    ///
    /// * `body` - The body to be set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::response::DeboaResponse;
    /// use http::{HeaderMap, StatusCode};
    ///
    /// let mut response = DeboaResponse::new(StatusCode::OK, HeaderMap::new(), Vec::new());
    /// response.set_raw_body(Vec::new());
    /// ```
    ///
    pub fn set_raw_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    /// Allow get raw body at any time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::response::DeboaResponse;
    /// use http::{HeaderMap, StatusCode};
    ///
    /// let response = DeboaResponse::new(StatusCode::OK, HeaderMap::new(), b"Hello, world!".to_vec());
    /// assert_eq!(response.raw_body(), &b"Hello, world!".to_vec());
    /// ```
    ///
    pub fn raw_body(&self) -> &Vec<u8> {
        &self.body
    }

    /// Allow get text body at any time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::response::DeboaResponse;
    /// use http::{HeaderMap, StatusCode};
    ///
    /// let response = DeboaResponse::new(StatusCode::OK, HeaderMap::new(), b"Hello, world!".to_vec());
    /// assert_eq!(response.text(), Ok(String::from_utf8_lossy("Hello, world!".as_bytes()).to_string()));
    /// ```
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
    /// # Examples
    ///
    /// ```rust
    /// use deboa::response::DeboaResponse;
    /// use http::{HeaderMap, StatusCode};
    ///
    /// let response = DeboaResponse::new(StatusCode::OK, HeaderMap::new(), b"Hello, world!".to_vec());
    /// response.to_file("test.txt").unwrap();
    /// ```
    ///
    pub fn to_file(&self, path: &str) -> Result<(), DeboaError> {
        let result = write(path, &self.body);
        if let Err(e) = result {
            return Err(DeboaError::Io { message: e.to_string() });
        }
        Ok(())
    }
}
