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

#[cfg(feature = "http1")]
use deboa::request::Http1Request;
#[cfg(feature = "http2")]
use deboa::request::Http2Request;
#[cfg(feature = "http3")]
use deboa::request::Http3Request;
use deboa::{
    conn::{ConnectionConfig, HttpConnectionDispatcher, ProtoConnection},
    errors::DeboaError,
    response::DeboaResponse,
    HttpVersion, Result,
};
use http::Request;
use hyper_body_utils::HttpBody;
use std::{marker::PhantomData, sync::Arc};
use url::Url;

use crate::cert::{DeboaCertificate, DeboaIdentity};

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

impl HttpConnectionDispatcher for DeboaConnection {
    async fn send_request(
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
            #[allow(unreachable_patterns)]
            _ => {
                return Err(DeboaError::UnsupportedProtocol);
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
pub struct BaseHttpConnection<Sender, ReqBody, ResBody> {
    pub(crate) sender: Sender,
    pub(crate) req_body: PhantomData<ReqBody>,
    pub(crate) res_body: PhantomData<ResBody>,
}

impl<Sender, ReqBody, ResBody> BaseHttpConnection<Sender, ReqBody, ResBody> {
    pub(crate) fn new(sender: Sender) -> Self {
        Self { sender, req_body: PhantomData, res_body: PhantomData }
    }
}

pub struct ConnectionFactory {}

impl ConnectionFactory {
    pub async fn create_connection<'a>(
        protocol: &HttpVersion,
        config: &'a ConnectionConfig<'a, DeboaIdentity, DeboaCertificate>,
    ) -> Result<DeboaConnection> {
        let conn = match protocol {
            #[cfg(feature = "http1")]
            HttpVersion::Http1 => {
                let conn =
                    BaseHttpConnection::<Http1Request, HttpBody, HttpBody>::connect(config).await?;
                DeboaConnection::Http1(Box::new(conn))
            }
            #[cfg(feature = "http2")]
            HttpVersion::Http2 => {
                let conn =
                    BaseHttpConnection::<Http2Request, HttpBody, HttpBody>::connect(config).await?;
                DeboaConnection::Http2(Box::new(conn))
            }
            #[cfg(feature = "http3")]
            HttpVersion::Http3 => {
                let conn = BaseHttpConnection::<Http3Request, HttpBody, HttpBody>::connect(&config)
                    .await?;
                DeboaConnection::Http3(Box::new(conn))
            }
            _ => {
                return Err(DeboaError::UnsupportedProtocol);
            }
        };

        Ok(conn)
    }
}
