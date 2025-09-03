#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[cfg(any(
    all(feature = "tokio-rt", feature = "smol-rt"),
    all(feature = "tokio-rt", feature = "compio-rt"),
    all(feature = "smol-rt", feature = "compio-rt")
))]
compile_error!("Only one runtime feature can be enabled at a time.");

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use url::Url;

use crate::http::io::DeboaConnection;
use crate::io::Decompressor;
use crate::middleware::DeboaMiddleware;

pub mod errors;
pub mod http;
pub mod io;
pub mod middleware;
pub mod request;
pub mod response;
mod runtimes;
#[cfg(test)]
mod tests;

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
    connection: Option<DeboaConnection>,
}

impl Debug for Deboa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deboa")
            .field("base_url", &self.base_url)
            .field("headers", &self.headers)
            .field("query_params", &self.query_params)
            .field("body", &self.body)
            .finish()
    }
}
