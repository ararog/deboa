use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http1::handshake, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio_native_tls::native_tls::{Certificate, Identity, TlsConnector};
use url::{Host, Url};

use crate::{
    cert::ClientCert,
    client::conn::http::{BaseHttpConnection, DeboaHttpConnection, Http1Request},
    errors::{ConnectionError, DeboaError},
    rt::tokio::stream::TokioStream,
    Result,
};

#[async_trait]
impl DeboaHttpConnection for BaseHttpConnection<Http1Request> {
    type Sender = Http1Request;

    #[inline]
    fn url(&self) -> &Url {
        &self.url
    }

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_11
    }

    async fn connect(
        url: Arc<Url>,
        client_cert: &Option<ClientCert>,
    ) -> Result<BaseHttpConnection<Self::Sender>> {
        let host = url.host().unwrap_or(Host::Domain("localhost"));
        let stream = {
            match url.scheme() {
                "ws" | "http" => {
                    let stream = {
                        let port = url.port().unwrap_or(80);
                        TcpStream::connect((host.to_string(), port)).await
                    };

                    if let Err(e) = stream {
                        return Err(DeboaError::Connection(ConnectionError::Tcp {
                            host: host.to_string(),
                            message: e.to_string(),
                        }));
                    }

                    TokioStream::Plain(stream.unwrap())
                }
                "wss" | "https" => {
                    let stream = {
                        let port = url.port().unwrap_or(443);
                        TcpStream::connect((host.to_string(), port)).await
                    };

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
                            return Err(DeboaError::ClientCert {
                                message: e.to_string(),
                            });
                        }
                        let identity = Identity::from_pkcs12(&file.unwrap(), client_cert.pw());
                        if let Err(e) = identity {
                            return Err(DeboaError::ClientCert {
                                message: e.to_string(),
                            });
                        }
                        builder.identity(identity.unwrap());

                        if let Some(ca) = client_cert.ca() {
                            let pem = std::fs::read(ca);
                            if let Err(e) = pem {
                                return Err(DeboaError::ClientCert {
                                    message: e.to_string(),
                                });
                            }
                            let cert = Certificate::from_pem(&pem.unwrap());
                            builder.add_root_certificate(cert.unwrap());
                        }
                    }

                    let connector = builder.build().unwrap();
                    let connector = tokio_native_tls::TlsConnector::from(connector);
                    let stream = connector.connect(&host.to_string(), socket).await;

                    if let Err(e) = stream {
                        return Err(DeboaError::Connection(ConnectionError::Tls {
                            host: host.to_string(),
                            message: e.to_string(),
                        }));
                    }

                    let stream = stream.unwrap();
                    TokioStream::Tls(stream)
                }
                scheme => {
                    return Err(DeboaError::Connection(ConnectionError::UnsupportedScheme {
                        message: format!("unsupported scheme: {scheme:?}"),
                    }));
                }
            }
        };

        let result = handshake(TokioIo::new(stream)).await;

        if let Err(err) = result {
            return Err(DeboaError::Connection(ConnectionError::Handshake {
                host: host.to_string(),
                message: err.to_string(),
            }));
        }

        let (sender, conn) = result.unwrap();

        tokio::spawn(async move {
            match conn.with_upgrades().await {
                Ok(_) => (),
                Err(_err) => {}
            };
        });

        Ok(BaseHttpConnection::<Self::Sender> {
            url: Arc::clone(&url),
            sender,
        })
    }

    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>> {
        let method = request.method().to_string();
        let result = self.sender.send_request(request).await;

        self.process_response(&self.url, &method, result).await
    }
}
