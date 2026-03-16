//! Connection management for the Deboa HTTP client.
//!
//! This module provides the building blocks for managing HTTP connections,
//! including connection pooling and protocol-specific implementations.
//!
//! # Architecture
//!
//! - [`http`]: Core HTTP protocol implementations (HTTP/1.1, HTTP/2)
//! - [`pool`]: Connection pooling for efficient request handling
//!
//! # Features
//!
//! - Automatic connection pooling
//! - Protocol negotiation (HTTP/1.1, HTTP/2)
//! - Connection lifecycle management
//! - Thread-safe connection handling
//! ```

use std::{marker::PhantomData, net::IpAddr, sync::Arc};

use http::Request;

use hyper_body_utils::HttpBody;
use url::Url;

use crate::{
    cert::{Certificate, Identity},
    response::DeboaResponse,
    HttpVersion, Result,
};

#[cfg(feature = "http1")]
use crate::request::Http1Request;

#[cfg(feature = "http2")]
use crate::request::Http2Request;

#[cfg(feature = "http3")]
use crate::request::Http3Request;

/// TCP protocol implementations.
///
/// This module contains the core HTTP protocol implementations, including:
/// - HTTP/1.1 support
/// - HTTP/2 support (when enabled)
/// - Connection management
/// - Request/response handling
///
/// # Features
///
/// - `http1`: Enables HTTP/1.1 support
/// - `http2`: Enables HTTP/2 support (requires TLS)
#[cfg(any(feature = "http1", feature = "http2"))]
pub mod tcp;

/// UDP protocol implementations.
///
/// This module contains the core HTTP protocol implementations, including:
/// - HTTP/1.1 support
/// - HTTP/2 support (when enabled)
/// - Connection management
/// - Request/response handling
///
/// # Features
///
/// - `http3`: Enables HTTP/3 support (requires TLS)
#[cfg(feature = "http3")]
pub mod udp;

/// Connection pooling for efficient HTTP connections.
///
/// This module provides connection pooling functionality to reuse connections
/// across multiple requests, reducing latency and resource usage.
///
/// # Features
///
/// - Automatic connection reuse
/// - Connection lifecycle management
/// - Thread-safe operation
/// - Configurable pool size (coming soon)
pub mod pool;

/// Internal stream handling utilities for connection establishment.
/// Provides low-level connection creation functions for both secure and insecure connections.
/// Used internally by the HTTP connection implementations.
///
/// # Modules
///
/// - `plain_connection`: Creates plain (non-TLS) TCP connections
/// - `tls_connection`: Creates TLS-encrypted connections with optional client certificates
///
/// # Examples
///
/// ```compile_fail, rust
/// use deboa::client::conn::stream::{plain_connection, tls_connection};
///
/// // Create a plain TCP connection
/// let stream = plain_connection("example.com:80").await?;
///
/// // Create a TLS connection
/// let stream = tls_connection("example.com:443", None).await?;
/// ```
pub(crate) mod stream;

/// Enum that represents the connection type.
///
/// # Variants
///
/// * `Http1` - The HTTP/1.1 connection.
/// * `Http2` - The HTTP/2 connection.
/// * `Http3` - The HTTP/3 connection.
pub enum DeboaConnection {
    #[cfg(feature = "http1")]
    Http1(Box<BaseHttpConnection<Http1Request, HttpBody, HttpBody>>),
    #[cfg(feature = "http2")]
    Http2(Box<BaseHttpConnection<Http2Request, HttpBody, HttpBody>>),
    #[cfg(feature = "http3")]
    Http3(Box<BaseHttpConnection<Http3Request, HttpBody, HttpBody>>),
}

impl DeboaConnection {
    pub async fn send_request(
        &mut self,
        url: Arc<Url>,
        request: Request<HttpBody>,
    ) -> Result<DeboaResponse> {
        let url = url.clone();
        let response = match self {
            #[cfg(feature = "http1")]
            DeboaConnection::Http1(ref mut conn) => {
                use crate::client::conn::tcp::DeboaTcpConnection;
                let response = conn
                    .send_request(request)
                    .await?;
                DeboaResponse::new(url, response)
            }
            #[cfg(feature = "http2")]
            DeboaConnection::Http2(ref mut conn) => {
                use crate::client::conn::tcp::DeboaTcpConnection;
                let response = conn
                    .send_request(request)
                    .await?;
                DeboaResponse::new(url, response)
            }
            #[cfg(feature = "http3")]
            DeboaConnection::Http3(ref mut conn) => {
                use crate::client::conn::udp::DeboaUdpConnection;
                let response = conn
                    .send_request(request)
                    .await?;
                DeboaResponse::new(url, response)
            }
        };

        Ok(response)
    }
}

/// Struct that represents the connection.
///
/// # Fields
///
/// * `sender` - The sender to use.
pub struct BaseHttpConnection<T, ReqBody, ResBody> {
    pub(crate) sender: T,
    pub(crate) req_body: PhantomData<ReqBody>,
    pub(crate) res_body: PhantomData<ResBody>,
}

pub struct ConnectionConfigBuilder<'a> {
    is_secure: bool,
    host: &'a str,
    port: u16,
    protocol: HttpVersion,
    identity: Option<Identity>,
    certificate: Option<Certificate>,
    skip_cert_verification: bool,
    client_bind_addr: IpAddr,
}

impl<'a> ConnectionConfigBuilder<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_secure: false,
            host: "",
            port: 80,
            #[cfg(all(feature = "http1", not(feature = "http2"), not(feature = "http3")))]
            protocol: HttpVersion::Http1,
            #[cfg(all(feature = "http1", feature = "http2", not(feature = "http3")))]
            protocol: HttpVersion::Http1,
            #[cfg(all(feature = "http1", feature = "http3", not(feature = "http2")))]
            protocol: HttpVersion::Http1,
            #[cfg(all(feature = "http2", not(feature = "http1"), not(feature = "http3")))]
            protocol: HttpVersion::Http2,
            #[cfg(all(feature = "http2", feature = "http3", not(feature = "http1")))]
            protocol: HttpVersion::Http2,
            #[cfg(all(feature = "http3", not(feature = "http1"), not(feature = "http2")))]
            protocol: HttpVersion::Http3,
            identity: None,
            certificate: None,
            skip_cert_verification: false,
            client_bind_addr: "0.0.0.0"
                .parse()
                .unwrap(),
        }
    }

    pub fn is_secure(mut self, is_secure: bool) -> Self {
        self.is_secure = is_secure;
        self
    }

    pub fn host(mut self, host: &'a str) -> Self {
        self.host = host;
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn protocol(mut self, protocol: HttpVersion) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn identity(mut self, identity: Option<Identity>) -> Self {
        self.identity = identity;
        self
    }

    pub fn certificate(mut self, certificate: Option<Certificate>) -> Self {
        self.certificate = certificate;
        self
    }

    pub fn skip_cert_verification(mut self, skip_cert_verification: bool) -> Self {
        self.skip_cert_verification = skip_cert_verification;
        self
    }

    pub fn client_bind_addr(mut self, client_bind_addr: IpAddr) -> Self {
        self.client_bind_addr = client_bind_addr;
        self
    }

    pub fn build(self) -> ConnectionConfig<'a> {
        ConnectionConfig {
            is_secure: self.is_secure,
            host: self.host,
            port: self.port,
            protocol: self.protocol,
            identity: self.identity,
            certificate: self.certificate,
            skip_cert_verification: self.skip_cert_verification,
            client_bind_addr: self.client_bind_addr,
        }
    }
}

pub struct ConnectionConfig<'a> {
    is_secure: bool,
    host: &'a str,
    port: u16,
    protocol: HttpVersion,
    identity: Option<Identity>,
    certificate: Option<Certificate>,
    skip_cert_verification: bool,
    client_bind_addr: IpAddr,
}

impl<'a> ConnectionConfig<'a> {
    pub fn builder() -> ConnectionConfigBuilder<'a> {
        ConnectionConfigBuilder::new()
    }

    pub fn is_secure(&self) -> bool {
        self.is_secure
    }

    pub fn host(&self) -> &str {
        self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn protocol(&self) -> &HttpVersion {
        &self.protocol
    }

    pub fn identity(&self) -> &Option<Identity> {
        &self.identity
    }

    pub fn certificate(&self) -> &Option<Certificate> {
        &self.certificate
    }

    pub fn skip_cert_verification(&self) -> bool {
        self.skip_cert_verification
    }

    pub fn client_bind_addr(&self) -> &IpAddr {
        &self.client_bind_addr
    }
}

pub struct ConnectionFactory {}

impl ConnectionFactory {
    pub async fn create_connection<'a>(
        protocol: &HttpVersion,
        config: &'a ConnectionConfig<'a>,
    ) -> Result<DeboaConnection> {
        let conn = match protocol {
            #[cfg(feature = "http1")]
            HttpVersion::Http1 => {
                use crate::client::conn::tcp::DeboaTcpConnection;
                let conn =
                    BaseHttpConnection::<Http1Request, HttpBody, HttpBody>::connect(config).await?;
                DeboaConnection::Http1(Box::new(conn))
            }
            #[cfg(feature = "http2")]
            HttpVersion::Http2 => {
                use crate::client::conn::tcp::DeboaTcpConnection;
                let conn =
                    BaseHttpConnection::<Http2Request, HttpBody, HttpBody>::connect(config).await?;
                DeboaConnection::Http2(Box::new(conn))
            }
            #[cfg(feature = "http3")]
            HttpVersion::Http3 => {
                use crate::client::conn::udp::DeboaUdpConnection;
                let conn = BaseHttpConnection::<Http3Request, HttpBody, HttpBody>::connect(&config)
                    .await?;
                DeboaConnection::Http3(Box::new(conn))
            }
        };

        Ok(conn)
    }
}
