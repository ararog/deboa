use http::{HeaderMap, StatusCode};

use crate::errors::DeboaError;

#[derive(Debug, PartialEq)]
pub struct DeboaResponse {
    pub(crate) status: StatusCode,
    headers: HeaderMap,
    body: Vec<u8>,
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

    pub fn raw_body(&self) -> Vec<u8> {
        self.body.clone()
    }

    pub fn text(&self) -> Result<String, DeboaError> {
        Ok(String::from_utf8_lossy(&self.body).to_string())
    }
}
