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
use crate::cert::{DeboaCertificate, DeboaIdentity};
#[cfg(feature = "http1")]
use deboa::request::Http1Request;
#[cfg(feature = "http2")]
use deboa::request::Http2Request;
use deboa::{
    conn::{ConnectionConfig, HttpConnectionDispatcher, ProtoConnection},
    errors::{DeboaError, RequestError},
    response::DeboaResponse,
    Result,
};
#[cfg(feature = "http3")]
use deboa_h3::compio::Http3Request;
use http::{Request, Version};
use hyper_body_utils::HttpBody;
use std::marker::PhantomData;

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
pub(crate) type Http1Connection = BaseHttpConnection<Http1Request, HttpBody, HttpBody>;
#[cfg(feature = "http2")]
pub(crate) type Http2Connection = BaseHttpConnection<Http2Request, HttpBody, HttpBody>;
#[cfg(feature = "http3")]
pub(crate) type Http3Connection = BaseHttpConnection<Http3Request, HttpBody, HttpBody>;

/// Enum that represents the connection type.
///
/// # Variants
///
/// * `Http1` - The HTTP/1.1 connection.
/// * `Http2` - The HTTP/2 connection.
/// * `Http3` - The HTTP/3 connection.
pub enum DeboaConnection {
    #[cfg(feature = "http1")]
    Http1(Box<Http1Connection>),
    #[cfg(feature = "http2")]
    Http2(Box<Http2Connection>),
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
    /// Send a request over the connection.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `request` - The request to send.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response or error.
    async fn send_request(&mut self, request: Request<HttpBody>) -> Result<DeboaResponse> {
        match self {
            #[cfg(feature = "http1")]
            DeboaConnection::Http1(ref mut conn) => {
                let (parts, body) = conn
                    .sender
                    .send_request(request)
                    .await
                    .map_err(|e| {
                        DeboaError::Request(RequestError::Send { message: e.to_string() })
                    })?
                    .into_parts();

                Ok(DeboaResponse::new(http::Response::from_parts(
                    parts,
                    HttpBody::from_incoming(body),
                )))
            }
            #[cfg(feature = "http2")]
            DeboaConnection::Http2(ref mut conn) => {
                let (parts, body) = conn
                    .sender
                    .send_request(request)
                    .await
                    .map_err(|e| {
                        DeboaError::Request(RequestError::Send { message: e.to_string() })
                    })?
                    .into_parts();

                Ok(DeboaResponse::new(http::Response::from_parts(
                    parts,
                    HttpBody::from_incoming(body),
                )))
            }
            #[cfg(feature = "http3")]
            DeboaConnection::Http3(ref mut conn) => {
                let response = conn
                    .sender
                    .send_request(request)
                    .await
                    .map_err(|e| {
                        DeboaError::Request(RequestError::Send { message: e.to_string() })
                    })?;

                Ok(DeboaResponse::new(response))
            }
            #[allow(unreachable_patterns, clippy::needless_return)]
            _ => {
                return Err(DeboaError::UnsupportedProtocol);
            }
        }
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
        protocol: &Version,
        config: &'a ConnectionConfig<'a, DeboaIdentity, DeboaCertificate>,
    ) -> Result<DeboaConnection> {
        let conn = match protocol {
            #[cfg(feature = "http1")]
            &Version::HTTP_11 => {
                let conn = Http1Connection::connect(config).await?;
                DeboaConnection::http1(conn)
            }
            #[cfg(feature = "http2")]
            &Version::HTTP_2 => {
                let conn = Http2Connection::connect(config).await?;
                DeboaConnection::http2(conn)
            }
            #[cfg(feature = "http3")]
            &Version::HTTP_3 => {
                let conn = Http3Connection::connect(config).await?;
                DeboaConnection::http3(conn)
            }
            _ => {
                return Err(DeboaError::UnsupportedProtocol);
            }
        };

        Ok(conn)
    }
}
