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
use crate::{
    cert::{DeboaCertificate, DeboaIdentity},
    HttpVersion,
};
#[cfg(feature = "http1")]
use deboa::{conn::SendRequest, request::Http1Request};
#[cfg(feature = "http2")]
use deboa::{conn::SendRequest, request::Http2Request};
use deboa::{
    conn::{ConnectionConfig, HttpConnectionDispatcher, ProtoConnection},
    errors::{DeboaError, RequestError},
    response::DeboaResponse,
    Result,
};
#[cfg(feature = "http3")]
use deboa_h3::generic::Http3Request;
use http::Request;
use hyper_body_utils::HttpBody;
use std::{marker::PhantomData, sync::Arc};
use url::Url;

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

#[cfg(feature = "http1")]
pub(crate) type Http1Connection =
    BaseHttpConnection<SendRequest<Http1Request, HttpBody>, HttpBody, HttpBody>;
#[cfg(feature = "http2")]
pub(crate) type Http2Connection =
    BaseHttpConnection<SendRequest<Http2Request, HttpBody>, HttpBody, HttpBody>;
#[cfg(feature = "http3")]
pub(crate) type Http3Connection = BaseHttpConnection<Http3Request, HttpBody, HttpBody>;

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
    Http1(Box<Http1Connection>),
    /// HTTP/2 connection
    #[cfg(feature = "http2")]
    Http2(Box<Http2Connection>),
    /// HTTP/3 connection
    #[cfg(feature = "http3")]
    Http3(Box<Http3Connection>),
}

impl DeboaConnection {
    #[cfg(feature = "http1")]
    pub fn http1(conn: Http1Connection) -> Self {
        DeboaConnection::Http1(Box::new(conn))
    }

    #[cfg(feature = "http2")]
    pub fn http2(conn: Http2Connection) -> Self {
        DeboaConnection::Http2(Box::new(conn))
    }

    #[cfg(feature = "http3")]
    pub fn http3(conn: Http3Connection) -> Self {
        DeboaConnection::Http3(Box::new(conn))
    }
}

impl HttpConnectionDispatcher for DeboaConnection {
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
    async fn send_request(
        &mut self,
        url: Arc<Url>,
        request: Request<HttpBody>,
    ) -> Result<DeboaResponse> {
        let url = url.clone();
        let conn = match self {
            #[cfg(feature = "http1")]
            DeboaConnection::Http1(ref mut conn) => conn,
            #[cfg(feature = "http2")]
            DeboaConnection::Http2(ref mut conn) => conn,
            #[cfg(feature = "http3")]
            DeboaConnection::Http3(ref mut conn) => conn,
            #[allow(unreachable_patterns)]
            _ => {
                return Err(DeboaError::UnsupportedProtocol);
            }
        };

        let response = conn
            .sender
            .send_request(request)
            .await
            .map_err(|e| DeboaError::Request(RequestError::Send { message: e.to_string() }))?;

        Ok(DeboaResponse::new(url, response))
    }
}

/// Connection factory.
pub struct ConnectionFactory {}

impl ConnectionFactory {
    /// Create a new connection.
    pub async fn create_connection<'a>(
        protocol: &HttpVersion,
        config: &'a ConnectionConfig<'a, DeboaIdentity, DeboaCertificate>,
    ) -> Result<DeboaConnection> {
        let conn = match protocol {
            #[cfg(feature = "http1")]
            HttpVersion::Http1 => {
                let conn = Http1Connection::connect(config).await?;
                DeboaConnection::http1(conn)
            }
            #[cfg(feature = "http2")]
            HttpVersion::Http2 => {
                let conn = Http2Connection::connect(config).await?;
                DeboaConnection::http2(conn)
            }
            #[cfg(feature = "http3")]
            HttpVersion::Http3 => {
                let conn = Http3Connection::connect(&config).await?;
                DeboaConnection::http3(conn)
            }
            _ => {
                return Err(DeboaError::UnsupportedProtocol);
            }
        };

        Ok(conn)
    }
}
