//! # Deboa - Core API Documentation
//!
//! Hello, and welcome to the core Deboa API documentation!
//!
//! This API documentation is highly technical and is purely a reference.
//!
//! Depend on `deboa` in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deboa = "0.0.5"
//! ```
//!
//! <small>Note that development versions, tagged with `-dev`, are not published
//! and need to be specified as [git dependencies].</small>
//!
//! ```rust,no_run
//! use deboa::{Deboa, Result, errors::DeboaError, request::DeboaRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let deboa = Deboa::builder()
//!         .build();
//!
//!     let response = DeboaRequest::get("https://httpbin.org/get")?
//!         .go(deboa)
//!         .await?;
//!
//!     println!("Response: {:#?}", response);
//!
//!     Ok(())
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
//! | `tokio_rt`      | Yes      | Support tokio runtime (enabled by default).             |
//! | `smol_rt`       | No       | Support smol runtime.                                   |
//! | `http1`         | Yes      | Support for HTTP/1 (enabled by default).                |
//! | `http2`         | Yes      | Support for HTTP/2 (enabled by default).                |
//!
//! Disabled features can be selectively enabled in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deboa = { version = "0.0.5", features = ["tokio_rt", "http1", "http2"] }
//! ```
//!
//! Conversely, HTTP/2 can be disabled:
//!
//! ```toml
//! [dependencies]
//! deboa = { version = "0.0.5", default-features = false }
//! ```
//!

#[cfg(all(feature = "tokio-rt", feature = "smol-rt"))]
compile_error!("Only one runtime feature can be enabled at a time.");

#[cfg(not(any(feature = "http1", feature = "http2")))]
compile_error!("At least one HTTP version feature must be enabled.");

pub(crate) const MAX_ERROR_MESSAGE_SIZE: usize = 50000;

use std::fmt::{Debug, Display};

use std::ops::Shl;

use bytes::Bytes;
use http::{header, HeaderValue, Request, Response};
use http_body_util::Full;
use hyper::body::Incoming;

use crate::cert::ClientCert;
use crate::client::conn::http::{DeboaConnection, DeboaHttpConnection};

use crate::catcher::DeboaCatcher;
use crate::client::conn::pool::{DeboaHttpConnectionPool, HttpConnectionPool};
use crate::errors::DeboaError;
use crate::request::{DeboaRequest, IntoRequest};
use crate::response::{DeboaResponse, IntoBody};
use crate::url::IntoUrl;

pub use async_trait::async_trait;

pub mod cache;
pub mod catcher;
pub mod cert;
pub mod client;
pub mod cookie;
pub mod errors;
pub mod form;
pub mod fs;
pub mod request;
pub mod response;
pub mod rt;
pub mod url;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, DeboaError>;

impl Shl<&str> for &Deboa {
    type Output = DeboaRequest;

    fn shl(self, other: &str) -> Self::Output {
        DeboaRequest::get(other)
            .expect("Invalid url!")
            .build()
            .expect("Could not build request!")
    }
}

#[derive(PartialEq, Debug)]
/// Enum that represents the HTTP version.
///
/// # Variants
///
/// * `Http1` - The HTTP/1.1 version.
/// * `Http2` - The HTTP/2 version.
pub enum HttpVersion {
    #[cfg(feature = "http1")]
    Http1,
    #[cfg(feature = "http2")]
    Http2,
    #[cfg(feature = "http3")]
    Http3,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "http1")]
            HttpVersion::Http1 => write!(f, "HTTP/1.1"),
            #[cfg(feature = "http2")]
            HttpVersion::Http2 => write!(f, "HTTP/2"),
            #[cfg(feature = "http3")]
            HttpVersion::Http3 => write!(f, "HTTP/3"),
        }
    }
}

/// Struct that represents the Deboa builder.
///
/// # Fields
///
/// * `retries` - The number of retries.
/// * `connection_timeout` - The connection timeout.
/// * `request_timeout` - The request timeout.
/// * `catchers` - The catchers.
/// * `protocol` - The protocol to use.
pub struct DeboaBuilder {
    connection_timeout: u64,
    request_timeout: u64,
    client_cert: Option<ClientCert>,
    catchers: Option<Vec<Box<dyn DeboaCatcher>>>,
    protocol: HttpVersion,
}

impl DeboaBuilder {
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

    /// Allow set client certificate at any time.
    ///
    /// # Arguments
    ///
    /// * `client_cert` - The client certificate.
    ///
    pub fn client_cert(mut self, client_cert: ClientCert) -> Self {
        self.client_cert = Some(client_cert);
        self
    }

    /// Allow add catcher at any time.
    ///
    /// # Arguments
    ///
    /// * `catcher` - The catcher to be added.
    ///
    pub fn catch<C: DeboaCatcher>(mut self, catcher: C) -> Self {
        if let Some(catchers) = &mut self.catchers {
            catchers.push(Box::new(catcher));
        } else {
            self.catchers = Some(vec![Box::new(catcher)]);
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
            connection_timeout: self.connection_timeout,
            request_timeout: self.request_timeout,
            client_cert: self.client_cert,
            catchers: self.catchers,
            protocol: self.protocol,
            pool: HttpConnectionPool::new(),
        }
    }
}

/// Struct that represents the Deboa instance.
///
/// # Fields
///
/// * `connection_timeout` - The connection timeout.
/// * `request_timeout` - The request timeout.
/// * `catchers` - The catchers.
/// * `protocol` - The protocol to use.
/// * `pool` - The connection pool.
//
pub struct Deboa {
    connection_timeout: u64,
    request_timeout: u64,
    client_cert: Option<ClientCert>,
    catchers: Option<Vec<Box<dyn DeboaCatcher>>>,
    protocol: HttpVersion,
    pool: HttpConnectionPool,
}

impl AsRef<Deboa> for Deboa {
    fn as_ref(&self) -> &Deboa {
        self
    }
}

impl AsMut<Deboa> for Deboa {
    fn as_mut(&mut self) -> &mut Deboa {
        self
    }
}

impl Debug for Deboa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deboa")
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
            connection_timeout: 0,
            request_timeout: 0,
            client_cert: None,
            catchers: None,
            protocol: HttpVersion::Http1,
            pool: HttpConnectionPool::new(),
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
            connection_timeout: 0,
            request_timeout: 0,
            client_cert: None,
            catchers: None,
            protocol: HttpVersion::Http1,
        }
    }

    /// Allow get protocol at any time.
    ///
    /// # Returns
    ///
    /// * `&HttpVersion` - The protocol.
    ///
    #[inline]
    pub fn protocol(&self) -> &HttpVersion {
        &self.protocol
    }

    /// Allow change protocol at any time.
    ///
    /// # Arguments
    ///
    /// * `protocol` - The protocol to be used.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
    ///
    pub fn set_protocol(&mut self, protocol: HttpVersion) -> &mut Self {
        self.protocol = protocol;
        self
    }

    /// Allow get request connection timeout at any time.
    ///
    /// # Returns
    ///
    /// * `u64` - The timeout.
    ///
    #[inline]
    pub fn connection_timeout(&self) -> u64 {
        self.connection_timeout
    }

    /// Allow change request connection timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new timeout.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
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
    #[inline]
    pub fn request_timeout(&self) -> u64 {
        self.request_timeout
    }

    /// Allow change request request timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new timeout.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
    ///
    pub fn set_request_timeout(&mut self, timeout: u64) -> &mut Self {
        self.request_timeout = timeout;
        self
    }

    /// Allow get client certificate at any time.
    ///
    /// # Returns
    ///
    /// * `Option<ClientCert>` - The client certificate.
    ///
    #[inline]
    pub fn client_cert(&self) -> Option<&ClientCert> {
        self.client_cert.as_ref()
    }

    /// Allow change client certificate at any time.
    ///
    /// # Arguments
    ///
    /// * `client_cert` - The client certificate to be used.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
    ///
    pub fn set_client_cert(&mut self, client_cert: Option<ClientCert>) -> &mut Self {
        self.client_cert = client_cert;
        self
    }

    /// Allow add catcher at any time.
    ///
    /// # Arguments
    ///
    /// * `catcher` - The catcher to be added.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
    ///
    pub fn catch<C: DeboaCatcher>(&mut self, catcher: C) -> &mut Self {
        if let Some(catchers) = &mut self.catchers {
            catchers.push(Box::new(catcher));
        } else {
            self.catchers = Some(vec![Box::new(catcher)]);
        }
        self
    }

    /// Allow execute a request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to be executed.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    pub async fn execute<R>(&mut self, request: R) -> Result<DeboaResponse>
    where
        R: IntoRequest,
    {
        let mut request = request.into_request()?;

        if let Some(catchers) = &self.catchers {
            let mut response = None;
            for catcher in catchers {
                response = catcher.on_request(request.as_mut()).await?;
            }

            if let Some(response) = response {
                let mut new_response = response;
                for catcher in catchers {
                    catcher.on_response(new_response.as_mut()).await?;
                }
                return Ok(new_response);
            }
        }

        let mut retry_count: u32 = 0;
        let response = loop {
            let response = self.send_request(request.as_mut()).await;
            if let Err(err) = response {
                if retry_count == request.retries() {
                    break Err(err);
                }
                #[cfg(feature = "tokio-rt")]
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    2_u32.pow(retry_count) as u64
                ))
                .await;
                #[cfg(feature = "smol-rt")]
                smol::Timer::after(std::time::Duration::from_secs(2_u32.pow(retry_count) as u64))
                    .await;
                retry_count += 1;
                continue;
            }

            let response = response.unwrap();

            if response.status().is_redirection() {
                let location = response.headers().get(header::LOCATION);
                if let Some(location) = location {
                    let location = location.to_str().unwrap();
                    let result = request.as_mut().set_url(location);
                    if let Err(err) = result {
                        break Err(err);
                    }
                }
                continue;
            }

            break Ok(response);
        };

        let res_url = request.url().to_string();
        let mut response = self.process_response(res_url, response?).await?;

        if let Some(catchers) = &self.catchers {
            for catcher in catchers {
                catcher.on_response(response.as_mut()).await?;
            }
        }

        Ok(response)
    }

    /// Allow send a request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to be sent.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Incoming>>` - The response.
    ///
    async fn send_request<R>(&mut self, request: &R) -> Result<Response<Incoming>>
    where
        R: AsRef<DeboaRequest>,
    {
        let url = request.as_ref().url();
        let mut uri = url.path().to_string();
        if let Some(query) = url.query() {
            uri.push('?');
            uri.push_str(query);
        }

        let method = request.as_ref().method();

        let mut builder = Request::builder()
            .uri(uri)
            .method(method.to_string().as_str());
        {
            let req_headers = builder.headers_mut().unwrap();

            request
                .as_ref()
                .headers()
                .into_iter()
                .fold(&mut *req_headers, |acc, (key, value)| {
                    acc.insert(key, value.into());
                    acc
                });

            if let Some(deboa_cookies) = request.as_ref().cookies() {
                let mut cookies = Vec::<String>::new();

                for cookie in deboa_cookies.values() {
                    cookies.push(cookie.to_string());
                }

                if let Ok(cookie_header) = HeaderValue::from_str(&cookies.join("; ")) {
                    req_headers.insert(header::COOKIE, cookie_header);
                }
            }
        }

        let request = builder.body(Full::new(Bytes::from(request.as_ref().raw_body().to_vec())));
        if let Err(err) = request {
            return Err(DeboaError::Request(errors::RequestError::Send {
                url: url.to_string(),
                method: method.to_string(),
                message: err.to_string(),
            }));
        }

        let request = request.unwrap();

        let conn = self
            .pool
            .create_connection(url, &self.protocol, &self.client_cert)
            .await?;
        match *conn {
            #[cfg(feature = "http1")]
            DeboaConnection::Http1(ref mut conn) => conn.send_request(request).await,
            #[cfg(feature = "http2")]
            DeboaConnection::Http2(ref mut conn) => conn.send_request(request).await,
        }
    }

    /// Allow process a response.
    ///
    /// # Arguments
    ///
    /// * `response` - The response to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    async fn process_response<U>(
        &self,
        url: U,
        response: Response<Incoming>,
    ) -> Result<DeboaResponse>
    where
        U: IntoUrl,
    {
        let response = response.map(|body| body.into_body());
        let response = DeboaResponse::new(url.into_url()?, response);
        Ok(response)
    }
}
