#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use url::Url;

use crate::fs::io::Decompressor;
use crate::middleware::DeboaMiddleware;
use crate::runtimes::tokio::http1::Http1ConnectionPool;
use crate::runtimes::tokio::http2::Http2ConnectionPool;

pub mod client;
pub mod errors;
pub mod fs;
pub mod middleware;
pub mod request;
pub mod response;
mod runtimes;
#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum HttpVersion {
    Http1,
    Http2,
}

pub struct Deboa {
    base_url: Url,
    headers: Option<HashMap<::http::HeaderName, String>>,
    query_params: Option<HashMap<String, String>>,
    body: Arc<Vec<u8>>,
    retries: u32,
    connection_timeout: u64,
    request_timeout: u64,
    middlewares: Option<Vec<Box<dyn DeboaMiddleware>>>,
    encodings: Option<HashMap<String, Box<dyn Decompressor>>>,
    protocol: HttpVersion,
    #[cfg(feature = "http1")]
    http1_pool: Http1ConnectionPool,
    #[cfg(feature = "http2")]
    http2_pool: Http2ConnectionPool,
}

impl Debug for Deboa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deboa")
            .field("base_url", &self.base_url)
            .field("headers", &self.headers)
            .field("query_params", &self.query_params)
            .field("body", &self.body)
            .field("retries", &self.retries)
            .field("connection_timeout", &self.connection_timeout)
            .field("request_timeout", &self.request_timeout)
            .field("protocol", &self.protocol)
            .finish()
    }
}
