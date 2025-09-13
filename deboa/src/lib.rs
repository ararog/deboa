//! # Deboa - Core API Documentation
//!
//! Hello, and welcome to the core Deboa API documentation!
//!
//! This API documentation is highly technical and is purely a reference.
//! There's an [overview] of Deboa on the main site as well as a [full,
//! detailed guide]. If you'd like pointers on getting started, see the
//! [quickstart] or [getting started] chapters of the guide.
//!
//! [overview]: https://rocket.rs/master/overview
//! [full, detailed guide]: https://rocket.rs/master/guide
//! [quickstart]: https://rocket.rs/master/guide/quickstart
//! [getting started]: https://rocket.rs/master/guide/getting-started
//!
//! ## Usage
//!
//! Depend on `deboa` in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deboa = "0.0.5-alpha.3"
//! ```
//!
//! <small>Note that development versions, tagged with `-dev`, are not published
//! and need to be specified as [git dependencies].</small>
//!
//! See the [guide](https://rocket.rs/master/guide) for more information on how
//! to write Rocket applications. Here's a simple example to get you started:
//!
//! [git dependencies]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories
//!
//! ```rust,no_run
//! use deboa::{Deboa, request::DeboaRequest};
//!
//! #[tokio::main]
//! async fn main() -> () {
//!     let mut deboa = Deboa::builder()
//!         .build();
//!
//!     let response = DeboaRequest::get("https://httpbin.org/get")
//!         .send_with(&mut deboa)
//!         .await;
//!
//!     println!("Response: {:#?}", response);
//! }
//! ```
//!
//! ## Features
//!
//! To avoid compiling unused dependencies, Deboa feature-gates optional
//! functionality, some enabled by default:
//!
//! | Feature         | Default? | Description                                             |
//! |-----------------|----------|---------------------------------------------------------|
//! | `tokio_rt`      | Yes      | Enables the default Deboa tracing [subscriber].         |
//! | `smol_rt`       | Yes      | Enables the default Deboa tracing [subscriber].         |
//! | `http1`         | Yes      | Support for HTTP/2 (enabled by default).                |
//! | `http2`         | Yes      | Support for HTTP/2 (enabled by default).                |
//!
//! Disabled features can be selectively enabled in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deboa = { version = "0.0.5-alpha.3", features = ["tokio_rt", "http1", "http2"] }
//! ```
//!
//! Conversely, HTTP/2 can be disabled:
//!
//! ```toml
//! [dependencies]
//! deboa = { version = "0.0.5-alpha.3", default-features = false }
//! ```
//!
use std::fmt::Debug;

use bytes::{Buf, Bytes};
use http::{HeaderValue, Request};
use http_body_util::{BodyExt, Full};

use crate::client::conn::http::DeboaHttpConnection;
#[cfg(feature = "http1")]
use crate::client::conn::http::Http1Request;
#[cfg(feature = "http2")]
use crate::client::conn::http::Http2Request;

use crate::catcher::DeboaCatcher;
use crate::client::conn::pool::{DeboaHttpConnectionPool, HttpConnectionPool};
use crate::request::DeboaRequest;

use url::Url;

use crate::errors::DeboaError;
use crate::response::DeboaResponse;

pub mod cache;
pub mod catcher;
pub mod client;
pub mod cookie;
pub mod errors;
pub mod fs;
pub mod request;
pub mod response;
mod rt;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum HttpVersion {
    #[cfg(feature = "http1")]
    Http1,
    #[cfg(feature = "http2")]
    Http2,
}

pub struct DeboaBuilder {
    retries: u32,
    connection_timeout: u64,
    request_timeout: u64,
    catchers: Option<Vec<Box<dyn DeboaCatcher>>>,
    protocol: HttpVersion,
    #[cfg(feature = "http1")]
    #[allow(dead_code)]
    http1_pool: HttpConnectionPool<Http1Request>,
    #[cfg(feature = "http2")]
    #[allow(dead_code)]
    http2_pool: HttpConnectionPool<Http2Request>,
}

impl DeboaBuilder {
    /// Allow set request retries at any time.
    ///
    /// # Arguments
    ///
    /// * `retries` - The new retries.
    ///
    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    /// Allow set request connection timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `connection_timeout` - The new connection timeout.
    ///
    pub fn connection_timeout(mut self, connection_timeout: u64) -> Self {
        self.connection_timeout = connection_timeout;
        self
    }

    /// Allow set request request timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `request_timeout` - The new request timeout.
    ///
    pub fn request_timeout(mut self, request_timeout: u64) -> Self {
        self.request_timeout = request_timeout;
        self
    }

    /// Allow add catcher at any time.
    ///
    /// # Arguments
    ///
    /// * `catcher` - The catcher to be added.
    ///
    pub fn catch<C: DeboaCatcher>(mut self, catch: C) -> Self {
        if let Some(catchers) = &mut self.catchers {
            catchers.push(Box::new(catch));
        } else {
            self.catchers = Some(vec![Box::new(catch)]);
        }
        self
    }

    /// Allow set request protocol at any time.
    ///
    /// # Arguments
    ///
    /// * `protocol` - The new protocol.
    ///
    pub fn protocol(mut self, protocol: HttpVersion) -> Self {
        self.protocol = protocol;
        self
    }

    /// Allow build Deboa instance.
    ///
    /// # Returns
    ///
    /// * `Deboa` - The new Deboa instance.
    ///
    pub fn build(self) -> Deboa {
        Deboa {
            retries: self.retries,
            connection_timeout: self.connection_timeout,
            request_timeout: self.request_timeout,
            catchers: self.catchers,
            protocol: self.protocol,
            #[cfg(feature = "http1")]
            http1_pool: HttpConnectionPool::<Http1Request>::new(),
            #[cfg(feature = "http2")]
            http2_pool: HttpConnectionPool::<Http2Request>::new(),
        }
    }
}

pub struct Deboa {
    retries: u32,
    connection_timeout: u64,
    request_timeout: u64,
    catchers: Option<Vec<Box<dyn DeboaCatcher>>>,
    protocol: HttpVersion,
    #[cfg(feature = "http1")]
    http1_pool: HttpConnectionPool<Http1Request>,
    #[cfg(feature = "http2")]
    http2_pool: HttpConnectionPool<Http2Request>,
}

impl Debug for Deboa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deboa")
            .field("retries", &self.retries)
            .field("connection_timeout", &self.connection_timeout)
            .field("request_timeout", &self.request_timeout)
            .field("protocol", &self.protocol)
            .finish()
    }
}

impl Deboa {
    /// Allow create a new Deboa instance.
    ///
    /// # Returns
    ///
    /// * `Deboa` - The new Deboa instance.
    ///
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Deboa {
            retries: 0,
            connection_timeout: 0,
            request_timeout: 0,
            catchers: None,
            protocol: HttpVersion::Http1,
            #[cfg(feature = "http1")]
            http1_pool: HttpConnectionPool::<Http1Request>::new(),
            #[cfg(feature = "http2")]
            http2_pool: HttpConnectionPool::<Http2Request>::new(),
        }
    }

    /// Allow create a new Deboa instance.
    ///
    /// # Returns
    ///
    /// * `DeboaBuilder` - The new DeboaBuilder instance.
    ///
    pub fn builder() -> DeboaBuilder {
        DeboaBuilder {
            retries: 0,
            connection_timeout: 0,
            request_timeout: 0,
            catchers: None,
            protocol: HttpVersion::Http1,
            #[cfg(feature = "http1")]
            http1_pool: HttpConnectionPool::<Http1Request>::new(),
            #[cfg(feature = "http2")]
            http2_pool: HttpConnectionPool::<Http2Request>::new(),
        }
    }

    /// Allow get protocol at any time.
    ///
    /// # Returns
    ///
    /// * `&HttpVersion` - The protocol.
    ///
    pub fn protocol(&self) -> &HttpVersion {
        &self.protocol
    }

    /// Allow change protocol at any time.
    ///
    /// # Arguments
    ///
    /// * `protocol` - The protocol to be used.
    ///
    pub fn set_protocol(&mut self, protocol: HttpVersion) -> &mut Self {
        self.protocol = protocol;
        self
    }

    /// Allow get request retries at any time.
    ///
    /// # Returns
    ///
    /// * `u32` - The retries.
    ///
    pub fn retries(&self) -> u32 {
        self.retries
    }

    /// Allow change request retries at any time.
    ///
    /// # Arguments
    ///
    /// * `retries` - The new retries.
    ///
    pub fn set_retries(&mut self, retries: u32) -> &mut Self {
        self.retries = retries;
        self
    }

    /// Allow get request connection timeout at any time.
    ///
    /// # Returns
    ///
    /// * `u64` - The timeout.
    ///
    pub fn connection_timeout(&self) -> u64 {
        self.connection_timeout
    }

    /// Allow change request connection timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new timeout.
    ///
    pub fn set_connection_timeout(&mut self, timeout: u64) -> &mut Self {
        self.connection_timeout = timeout;
        self
    }

    /// Allow get request request timeout at any time.
    ///
    /// # Returns
    ///
    /// * `u64` - The timeout.
    ///
    pub fn request_timeout(&self) -> u64 {
        self.request_timeout
    }

    /// Allow change request request timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new timeout.
    ///
    pub fn set_request_timeout(&mut self, timeout: u64) -> &mut Self {
        self.request_timeout = timeout;
        self
    }

    /// Allow add catcher at any time.
    ///
    /// # Arguments
    ///
    /// * `catcher` - The catcher to be added.
    ///
    pub fn catch<C: DeboaCatcher>(&mut self, catcher: C) -> &mut Self {
        if let Some(catchers) = &mut self.catchers {
            catchers.push(Box::new(catcher));
        } else {
            self.catchers = Some(vec![Box::new(catcher)]);
        }
        self
    }

    pub async fn execute(&mut self, mut request: DeboaRequest) -> Result<DeboaResponse, DeboaError> {
        if let Some(catchers) = &self.catchers {
            let mut response = catchers.iter().filter_map(|catcher| catcher.on_request(&mut request).unwrap());

            if let Some(mut response) = response.next() {
                catchers.iter().for_each(|catcher| catcher.on_response(&mut response));
                return Ok(response);
            }
        }

        let url = Url::parse(&request.url());
        if let Err(e) = url {
            return Err(DeboaError::UrlParse { message: e.to_string() });
        }

        let url = url.unwrap();
        let method = request.method();
        let authority = url.authority();

        let mut builder = Request::builder()
            .uri(url.as_str())
            .method(method.to_string().as_str())
            .header(hyper::header::HOST, authority);
        {
            let req_headers = builder.headers_mut().unwrap();
            request.headers().iter().fold(req_headers, |acc, (key, value)| {
                acc.insert(key, HeaderValue::from_str(value).unwrap());
                acc
            });
        }

        let body = request.raw_body();

        let request = builder.body(Full::new(Bytes::from(body.to_vec())));
        if let Err(err) = request {
            return Err(DeboaError::Request {
                host: url.host().unwrap().to_string(),
                path: url.path().to_string(),
                method: method.to_string(),
                message: err.to_string(),
            });
        }

        let request = request.unwrap();

        #[cfg(all(feature = "http1", feature = "http2"))]
        let response = if self.protocol == HttpVersion::Http1 {
            let conn = self.http1_pool.create_connection(&url).await?;
            conn.send_request(request).await?
        } else {
            let conn = self.http2_pool.create_connection(&url).await?;
            conn.send_request(request).await?
        };

        #[cfg(all(feature = "http1", not(feature = "http2")))]
        let response = {
            let conn = self.http1_pool.create_connection(&url).await?;
            conn.send_request(hyper_request).await?
        };

        #[cfg(all(feature = "http2", not(feature = "http1")))]
        let response = {
            let conn = self.http2_pool.create_connection(&url).await?;
            conn.send_request(hyper_request).await?
        };

        let status_code = response.status();
        let headers = response.headers().clone();

        let result = response.collect().await;
        if let Err(err) = result {
            return Err(DeboaError::ProcessResponse { message: err.to_string() });
        }

        let mut response_body = result.unwrap().aggregate();

        let raw_body = response_body.copy_to_bytes(response_body.remaining()).to_vec();

        let mut response = DeboaResponse::new(status_code, headers, &raw_body);

        if let Some(catchers) = &self.catchers {
            catchers.iter().for_each(|catcher| catcher.on_response(&mut response));
        }

        Ok(response)
    }
}
