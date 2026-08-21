//! Connection management for the Deboa HTTP client.
//!
//! This module provides the building blocks for managing HTTP connections,
//! including connection pooling and protocol-specific implementations.
//!
//! # Architecture
//!
//! - [`http`]: Core HTTP protocol implementations (HTTP/1.1, HTTP/2 and HTTP/3)
//! - [`pool`]: Connection pooling for efficient request handling
//!
//! # Features
//!
//! - Automatic connection pooling
//! - Protocol negotiation (HTTP/1.1, HTTP/2 and HTTP/3)
//! - Connection lifecycle management
//! - Thread-safe connection handling
//! ```
use crate::cert::{DeboaCertificate, DeboaIdentity};
#[cfg(feature = "rust-tls")]
use compio::net::TcpStream;
#[cfg(any(feature = "http1", feature = "http2"))]
use cyper_core::HyperStream;
#[cfg(feature = "http1")]
use deboa::request::Http1Request;
#[cfg(feature = "http2")]
use deboa::request::Http2Request;
use deboa::{
    conn::{ConnectionConfig, HttpConnectionDispatcher, ProtoConnection},
    dns::DnsResolver,
    errors::{ConnectionError, DeboaError, RequestError},
    response::DeboaResponse,
    Result,
};
#[cfg(feature = "http3")]
use deboa_h3::compio::Http3Request;
use http::{Request, Version};
use hyper_body_utils::HttpBody;
use log::info;
#[cfg(feature = "rust-tls")]
use std::borrow::Cow;
use std::{marker::PhantomData, time::Duration};

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

    async fn send(&mut self, request: Request<HttpBody>) -> Result<DeboaResponse> {
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
    async fn send_request(
        &mut self,
        request: Request<HttpBody>,
        timeout: Duration,
    ) -> Result<DeboaResponse> {
        compio::time::timeout(timeout, self.send(request))
            .await
            .map_err(|_| {
                DeboaError::Request(RequestError::Send { message: "Request timed out".to_string() })
            })?
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

#[cfg(feature = "rust-tls")]
async fn connect_with_rustls<'a>(
    tcp_stream: TcpStream,
    config: &ConnectionConfig<'a, DeboaIdentity, DeboaCertificate>,
) -> Result<(Version, HyperStream<TcpStream>)> {
    use crate::client::tls::rustls::{tcp::connect, TlsConnectionBuilder};
    let tls_config = TlsConnectionBuilder::default()
        .certificate(config.certificate())
        .identity(config.identity())
        .build_config()?;

    let stream = connect(tls_config, tcp_stream, config.host()).await?;

    if let Some(alpn) = stream.negotiated_alpn() {
        let Cow::Borrowed(alpn_code) = String::from_utf8_lossy(&alpn) else {
            return Err(DeboaError::Connection(ConnectionError::Tcp {
                message: "Invalid ALPN code".to_string(),
            }));
        };

        let version = match alpn_code {
            "http1.1" => Version::HTTP_11,
            "h2" => Version::HTTP_2,
            "h3" => Version::HTTP_3,
            _ => *config.protocol_version(),
        };

        info!("ALPN info found, switching connection to {:?}", version);
        Ok((version, HyperStream::new_tls(stream)))
    } else {
        info!("No ALPN info available, falling back to HTTP/1.1");
        Ok((*config.protocol_version(), HyperStream::new_tls(stream)))
    }
}

#[cfg(feature = "native-tls")]
async fn connect_with_nativels<'a>(
    tcp_stream: TcpStream,
    config: &ConnectionConfig<'a, DeboaIdentity, DeboaCertificate>,
) -> Result<(Version, HyperStream)> {
    use crate::client::tls::native::TlsConnectionBuilder;
    let stream = TlsConnectionBuilder::new(tcp_stream, config.host())
        .certificate(config.certificate())
        .identity(config.identity())
        .connect()
        .await?;

    if let Some(alpn) = stream.negotiated_alpn() {
        let Cow::Borrowed(alpn_code) = String::from_utf8_lossy(alpn) else {
            return Err(DeboaError::Connection(ConnectionError::Tcp {
                message: "Invalid ALPN code".to_string(),
            }));
        };

        let version = match alpn_code {
            "http1.1" => Version::HTTP_11,
            "h2" => Version::HTTP_2,
            "h3" => Version::HTTP_3,
            _ => *config.protocol_version(),
        };

        info!("ALPN info found, switching connection to {:?}", version);
        Ok((version, HyperStream::Tls(stream)))
    } else {
        info!("No ALPN info available, falling back to HTTP/1.1");
        Ok((Version::HTTP_11, HyperStream::Tls(stream)))
    }
}

pub struct ConnectionFactory {}

impl ConnectionFactory {
    /// Create a new connection.
    pub async fn create_connection<'a, D>(
        config: &'a ConnectionConfig<'a, DeboaIdentity, DeboaCertificate>,
        dns_resolver: &D,
    ) -> Result<DeboaConnection>
    where
        D: DnsResolver,
    {
        let ips = dns_resolver
            .resolve(
                config
                    .host()
                    .to_string(),
                config.port(),
            )
            .await?;
        let ips = if config
            .client_bind_addr()
            .is_ipv4()
        {
            ips.into_iter()
                .filter(|ip| ip.is_ipv4())
                .collect::<Vec<_>>()
        } else {
            ips.into_iter()
                .filter(|ip| ip.is_ipv6())
                .collect::<Vec<_>>()
        };

        let Some(ip) = ips.first() else {
            return Err(DeboaError::Request(RequestError::Send {
                message: format!("No IP addresses found for hostname: {}", config.host()),
            }));
        };

        #[cfg(any(feature = "http1", feature = "http2"))]
        let conn_pair = {
            use compio::net::TcpStream;
            use cyper_core::HyperStream;

            let tcp_stream = TcpStream::connect(format!("{}:{}", ip, config.port()))
                .await
                .map_err(|e| {
                    DeboaError::Connection(ConnectionError::Tcp { message: e.to_string() })
                })?;
            let use_tls = config.scheme() == "https" || config.scheme() == "wss";
            if !use_tls {
                (Version::HTTP_11, HyperStream::new_plain(tcp_stream))
            } else {
                #[cfg(feature = "rust-tls")]
                {
                    connect_with_rustls(tcp_stream, config).await?
                }

                #[cfg(feature = "native-tls")]
                {
                    connect_with_nativels(tcp_stream, config).await?
                }
            }
        };

        let conn = match conn_pair.0 {
            #[cfg(feature = "http1")]
            Version::HTTP_11 => {
                let conn = Http1Connection::connect(conn_pair.1).await?;
                DeboaConnection::http1(conn)
            }
            #[cfg(feature = "http2")]
            Version::HTTP_2 => {
                let conn = Http2Connection::connect(conn_pair.1).await?;
                DeboaConnection::http2(conn)
            }
            #[cfg(feature = "http3")]
            Version::HTTP_3 => {
                let stream = {
                    use crate::client::tls::rustls::{udp::connect, TlsConnectionBuilder};
                    use compio_quic::Endpoint;
                    use deboa::errors::ConnectionError;
                    use std::net::SocketAddr;

                    let mut client_endpoint =
                        Endpoint::client(SocketAddr::new(*config.client_bind_addr(), 0))
                            .await
                            .map_err(|e| {
                                DeboaError::Connection(ConnectionError::Udp {
                                    message: e.to_string(),
                                })
                            })?;

                    let tls_config = TlsConnectionBuilder::default()
                        .certificate(config.certificate())
                        .identity(config.identity())
                        .build_config()?;

                    connect(
                        tls_config,
                        &mut client_endpoint,
                        SocketAddr::new(*ip, config.port()),
                        config.host(),
                    )
                    .await?
                };

                let conn = Http3Connection::connect(stream).await?;
                DeboaConnection::http3(conn)
            }
            _ => {
                return Err(DeboaError::UnsupportedProtocol);
            }
        };

        Ok(conn)
    }
}
