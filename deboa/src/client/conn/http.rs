use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response, StatusCode, Version};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
#[cfg(feature = "smol-rt")]
use smol::net::TcpStream;
#[cfg(feature = "tokio-rt")]
use tokio::net::TcpStream;
#[cfg(feature = "tokio-native-tls")]
use tokio_native_tls::native_tls::{Certificate, Identity, TlsConnector};
#[cfg(feature = "tokio-rust-tls")]
use tokio_rustls::TlsConnector;

use crate::{
    cert::ClientCert,
    errors::{ConnectionError, DeboaError, RequestError, ResponseError},
    rt::tokio::stream::TokioStream,
    Result, MAX_ERROR_MESSAGE_SIZE,
};

#[derive(Debug)]
/// Enum that represents the connection type.
///
/// # Variants
///
/// * `Http1` - The HTTP/1.1 connection.
/// * `Http2` - The HTTP/2 connection.
pub enum DeboaConnection {
    #[cfg(feature = "http1")]
    Http1(Box<BaseHttpConnection<Http1Request>>),
    #[cfg(feature = "http2")]
    Http2(Box<BaseHttpConnection<Http2Request>>),
}

#[derive(Debug, Clone)]
/// Struct that represents the connection.
///
/// # Fields
///
/// * `sender` - The sender to use.
pub struct BaseHttpConnection<T> {
    pub(crate) sender: T,
}

#[cfg(feature = "http1")]
pub type Http1Request = hyper::client::conn::http1::SendRequest<Full<Bytes>>;
#[cfg(feature = "http2")]
pub type Http2Request = hyper::client::conn::http2::SendRequest<Full<Bytes>>;

#[async_trait]
/// Trait that represents the HTTP connection.
///
/// # Type Parameters
///
/// * `Sender` - The sender to use.
///
pub trait DeboaHttpConnection: private::DeboaHttpConnectionSealed {
    type Sender;

    #[cfg(feature = "tokio-rt")]
    async fn plain_connection(host: &str) -> Result<TokioStream> {
        let stream = { TcpStream::connect(host).await };

        if let Err(e) = stream {
            return Err(DeboaError::Connection(ConnectionError::Tcp {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        Ok(TokioStream::Plain(stream.unwrap()))
    }

    #[cfg(feature = "smol-rt")]
    async fn plain_connection(host: &str) -> Result<SmolStream> {
        let stream = { TcpStream::connect(host).await };

        if let Err(e) = stream {
            return Err(DeboaError::Connection(ConnectionError::Tcp {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        Ok(SmolStream::Plain(stream.unwrap()))
    }

    #[cfg(all(feature = "tokio-rt", feature = "tokio-native-tls"))]
    async fn tls_connection(host: &str, client_cert: &Option<ClientCert>) -> Result<TokioStream> {
        let stream = { TcpStream::connect(host).await };

        if let Err(e) = stream {
            return Err(DeboaError::Connection(ConnectionError::Tcp {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let socket = stream.unwrap();
        let mut builder = TlsConnector::builder();
        if let Some(client_cert) = client_cert {
            let file = std::fs::read(client_cert.cert());
            if let Err(e) = file {
                return Err(DeboaError::ClientCert { message: e.to_string() });
            }
            let identity = Identity::from_pkcs12(&file.unwrap(), client_cert.pw());
            if let Err(e) = identity {
                return Err(DeboaError::ClientCert { message: e.to_string() });
            }
            builder.identity(identity.unwrap());

            if let Some(ca) = client_cert.ca() {
                let pem = std::fs::read(ca);
                if let Err(e) = pem {
                    return Err(DeboaError::ClientCert { message: e.to_string() });
                }
                let cert = Certificate::from_pem(&pem.unwrap());
                builder.add_root_certificate(cert.unwrap());
            }
        }

        let connector = builder
            .build()
            .unwrap();
        let connector = tokio_native_tls::TlsConnector::from(connector);
        let stream = connector
            .connect(host, socket)
            .await;

        if let Err(e) = stream {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let stream = stream.unwrap();
        Ok(TokioStream::Tls(stream))
    }

    #[cfg(all(feature = "tokio-rt", feature = "tokio-rust-tls"))]
    #[hotpath::measure]
    async fn tls_connection(host: &str, client_cert: &Option<ClientCert>) -> Result<TokioStream> {
        use rustls::pki_types::ServerName;

        let stream = { TcpStream::connect(host).await };

        if let Err(e) = stream {
            return Err(DeboaError::Connection(ConnectionError::Tcp {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let socket = stream.unwrap();
        let root_store = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let connector = TlsConnector::from(Arc::new(config));

        let hostname = if let Some((hostname, _)) = host.split_once(':') { hostname } else { host };

        let parsed_hostname = ServerName::try_from(hostname.to_string());

        if let Err(e) = parsed_hostname {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: hostname.to_string(),
                message: e.to_string(),
            }));
        }

        let stream = connector
            .connect(parsed_hostname.unwrap(), socket)
            .await;

        if let Err(e) = stream {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: hostname.to_string(),
                message: e.to_string(),
            }));
        }

        let stream = stream.unwrap();
        Ok(TokioStream::Tls(Box::new(stream)))
    }

    #[cfg(all(feature = "smol-rt", feature = "smol-native-tls"))]
    async fn tls_connection(host: &str, client_cert: &Option<ClientCert>) -> Result<SmolStream> {
        let stream = { TcpStream::connect(host).await };

        if let Err(e) = stream {
            return Err(DeboaError::Connection(ConnectionError::Tcp {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let socket = stream.unwrap();
        let root_store = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let connector = TlsConnector::from(Arc::new(config));

        let hostname = if let Some((hostname, _)) = host.split_once(':') { hostname } else { host };

        let parsed_hostname = ServerName::try_from(hostname.to_string());

        if let Err(e) = parsed_hostname {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: hostname,
                message: e.to_string(),
            }));
        }

        let stream = connector
            .connect(parsed_hostname.unwrap(), socket)
            .await;

        if let Err(e) = stream {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let stream = stream.unwrap();
        SmolStream::Tls(stream)
    }

    #[cfg(all(feature = "smol-rt", feature = "smol-rust-tls"))]
    async fn tls_connection(host: &str, client_cert: &Option<ClientCert>) -> Result<SmolStream> {
        let stream = { TcpStream::connect(host).await };

        if let Err(e) = stream {
            return Err(DeboaError::Connection(ConnectionError::Tcp {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let socket = stream.unwrap();
        let root_store = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        let connector = TlsConnector::from(Arc::new(config));

        let hostname = if let Some((hostname, _)) = host.split_once(':') {
            hostname
        };

        let hostname = ServerName::try_from(hostname.to_string());

        if let Err(e) = hostname {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: hostname,
                message: e.to_string(),
            }));
        }

        let stream = connector
            .connect(hostname.unwrap(), socket)
            .await;

        if let Err(e) = stream {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let stream = stream.unwrap();
        SmolStream::Tls(stream)
    }

    /// Create a new connection.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to connect.
    ///
    /// # Returns
    ///
    /// * `Result<BaseHttpConnection<Self::Sender>>` - The connection or error.
    ///
    async fn connect(
        is_secure: bool,
        host: &str,
        client_cert: &Option<ClientCert>,
    ) -> Result<BaseHttpConnection<Self::Sender>>;

    /// Get connection protocol.
    ///
    /// # Returns
    ///
    /// * `Version` - The connection protocol.
    ///
    fn protocol(&self) -> Version;

    /// Send a request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to send.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Incoming>>` - The response or error.
    ///
    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>>;

    /// Process a response.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to connect.
    /// * `method` - The method to use.
    /// * `response` - The response to process.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Incoming>>` - The response or error.
    ///
    #[hotpath::measure]
    async fn process_response(
        &self,
        method: &str,
        response: std::result::Result<Response<Incoming>, hyper::Error>,
    ) -> Result<Response<Incoming>> {
        if let Err(err) = response {
            return Err(DeboaError::Request(RequestError::Send {
                url: "".to_string(),
                method: method.to_string(),
                message: err.to_string(),
            }));
        }

        let response = response.unwrap();
        let status_code = response.status();
        if (!status_code.is_success()
            && !status_code.is_informational()
            && !status_code.is_redirection())
            || status_code == StatusCode::TOO_MANY_REQUESTS
        {
            let mut body = response.into_body();
            let mut error_message = Vec::new();
            let mut downloaded = 0;
            while let Some(chunk) = body.frame().await {
                if let Ok(frame) = chunk {
                    if let Some(data) = frame.data_ref() {
                        if downloaded + data.len() > MAX_ERROR_MESSAGE_SIZE {
                            break;
                        }
                        error_message.extend_from_slice(data);
                        downloaded += data.len();
                    }
                }
            }
            return Err(DeboaError::Response(ResponseError::Receive {
                status_code,
                message: format!(
                    "Could not process request ({}): {}",
                    status_code,
                    String::from_utf8_lossy(&error_message)
                ),
            }));
        }

        Ok(response)
    }
}

pub(crate) mod private {
    pub trait DeboaHttpConnectionSealed {}
}
