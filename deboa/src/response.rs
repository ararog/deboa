use std::fmt::Debug;

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
    pub fn new(status: StatusCode, headers: HeaderMap, body: Vec<u8>) -> Self {
        Self { status, headers, body }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn text(&self) -> Result<String, DeboaError> {
        Ok(String::from_utf8_lossy(&self.body).to_string())
    }
}
