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

use deboa::response::DeboaResponse;
use http::Request;
use hyper_body_utils::HttpBody;
use std::{marker::PhantomData, net::IpAddr, sync::Arc};
use url::Url;

use crate::{
    cert::{Certificate, Identity},
    HttpVersion, Result,
};

#[cfg(feature = "http1")]
use deboa::request::Http1Request;

#[cfg(feature = "http2")]
use deboa::request::Http2Request;

#[cfg(feature = "http3")]
use deboa::request::Http3Request;

/// DNS resolution for the Deboa HTTP client.
///
/// This module provides DNS resolution functionality for the Deboa HTTP client.
pub mod dns;

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

/// Stream module for runtime-specific stream implementations.
///
/// This module provides stream implementations for different runtimes (Tokio, Smol, etc.).
pub(crate) mod stream;

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
pub(crate) mod tcp;

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
pub(crate) mod udp;

pub(crate) const fn default_protocol() -> HttpVersion {
    #[cfg(feature = "http1")]
    return HttpVersion::Http1;
    #[cfg(feature = "http2")]
    return HttpVersion::Http2;
    #[cfg(feature = "http3")]
    return HttpVersion::Http3;
}

/// Enum that represents the connection type.
///
/// # Variants
///
/// * `Http1` - The HTTP/1.1 connection.
/// * `Http2` - The HTTP/2 connection.
/// * `Http3` - The HTTP/3 connection.
pub enum DeboaConnection {
    /// HTTP/1.1 connection
    #[cfg(feature = "http1")]
    Http1(Box<BaseHttpConnection<Http1Request, HttpBody, HttpBody>>),
    /// HTTP/2 connection
    #[cfg(feature = "http2")]
    Http2(Box<BaseHttpConnection<Http2Request, HttpBody, HttpBody>>),
    /// HTTP/3 connection
    #[cfg(feature = "http3")]
    Http3(Box<BaseHttpConnection<Http3Request, HttpBody, HttpBody>>),
}

impl DeboaConnection {
    /// Send a request through the connection.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `request` - The request to send.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response from the server.
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

/// Builder for connection configuration.
pub struct ConnectionConfigBuilder<'a> {
    is_secure: bool,
    ip: IpAddr,
    host: &'a str,
    port: u16,
    protocol: HttpVersion,
    identity: Option<Identity>,
    certificate: Option<Certificate>,
    skip_cert_verification: bool,
    client_bind_addr: IpAddr,
}

impl<'a> ConnectionConfigBuilder<'a> {
    /// Create a new connection configuration builder.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_secure: false,
            ip: "127.0.0.1"
                .parse::<IpAddr>()
                .unwrap(),
            host: "",
            port: 80,
            protocol: default_protocol(),
            identity: None,
            certificate: None,
            skip_cert_verification: false,
            client_bind_addr: "0.0.0.0"
                .parse()
                .unwrap(),
        }
    }

    /// Set whether the connection is secure.
    pub fn is_secure(mut self, is_secure: bool) -> Self {
        self.is_secure = is_secure;
        self
    }

    /// Set the IP address for the connection.
    pub fn ip(mut self, ip: IpAddr) -> Self {
        self.ip = ip;
        self
    }

    /// Set the host for the connection.
    pub fn host(mut self, host: &'a str) -> Self {
        self.host = host;
        self
    }

    /// Set the port for the connection.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set the protocol for the connection.
    pub fn protocol(mut self, protocol: HttpVersion) -> Self {
        self.protocol = protocol;
        self
    }

    /// Set the identity for the connection.
    pub fn identity(mut self, identity: Option<Identity>) -> Self {
        self.identity = identity;
        self
    }

    /// Set the certificate for the connection.
    pub fn certificate(mut self, certificate: Option<Certificate>) -> Self {
        self.certificate = certificate;
        self
    }

    /// Set whether to skip certificate verification.
    pub fn skip_cert_verification(mut self, skip_cert_verification: bool) -> Self {
        self.skip_cert_verification = skip_cert_verification;
        self
    }

    /// Set the client bind address for the connection.
    pub fn client_bind_addr(mut self, client_bind_addr: IpAddr) -> Self {
        self.client_bind_addr = client_bind_addr;
        self
    }

    /// Build the connection configuration.
    pub fn build(self) -> ConnectionConfig<'a> {
        ConnectionConfig {
            is_secure: self.is_secure,
            ip: self.ip,
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

/// Connection configuration.
pub struct ConnectionConfig<'a> {
    is_secure: bool,
    ip: IpAddr,
    host: &'a str,
    port: u16,
    protocol: HttpVersion,
    identity: Option<Identity>,
    certificate: Option<Certificate>,
    skip_cert_verification: bool,
    client_bind_addr: IpAddr,
}

impl<'a> ConnectionConfig<'a> {
    /// Create a new connection configuration builder.
    pub fn builder() -> ConnectionConfigBuilder<'a> {
        ConnectionConfigBuilder::new()
    }

    /// Get whether the connection is secure.
    pub fn is_secure(&self) -> bool {
        self.is_secure
    }

    /// Get the IP address for the connection.
    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }

    /// Get the host for the connection.
    pub fn host(&self) -> &str {
        self.host
    }

    /// Get the port for the connection.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get the protocol for the connection.
    pub fn protocol(&self) -> &HttpVersion {
        &self.protocol
    }

    /// Get the identity for the connection.
    pub fn identity(&self) -> &Option<Identity> {
        &self.identity
    }

    /// Get the certificate for the connection.
    pub fn certificate(&self) -> &Option<Certificate> {
        &self.certificate
    }

    /// Get whether to skip certificate verification.
    pub fn skip_cert_verification(&self) -> bool {
        self.skip_cert_verification
    }

    /// Get the client bind address for the connection.
    pub fn client_bind_addr(&self) -> &IpAddr {
        &self.client_bind_addr
    }
}

/// Connection factory.
pub struct ConnectionFactory {}

impl ConnectionFactory {
    /// Create a new connection.
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
