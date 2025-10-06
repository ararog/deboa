use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http1::handshake, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio_native_tls::native_tls::{Identity, TlsConnector};
use url::{Host, Url};

use crate::{
    cert::ClientCert,
    client::conn::http::{BaseHttpConnection, DeboaHttpConnection, Http1Request},
    errors::DeboaError,
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

    async fn connect(url: &Url, client_cert: &Option<ClientCert>) -> Result<BaseHttpConnection<Self::Sender>> {
        let host = url.host().unwrap_or(Host::Domain("localhost"));
        let stream = {
            match url.scheme() {
                "http" => {
                    let stream = {
                        let port = url.port().unwrap_or(80);
                        TcpStream::connect((host.to_string(), port)).await
                    };

                    if let Err(e) = stream {
                        return Err(DeboaError::Connection {
                            host: host.to_string(),
                            message: e.to_string(),
                        });
                    }

                    TokioStream::Plain(stream.unwrap())
                }
                "https" => {
                    let stream = {
                        let port = url.port().unwrap_or(443);
                        TcpStream::connect((host.to_string(), port)).await
                    };

                    if let Err(e) = stream {
                        return Err(DeboaError::Connection {
                            host: host.to_string(),
                            message: e.to_string(),
                        });
                    }

                    let socket = stream.unwrap();
                    let connector = if let Some(client_cert) = client_cert {
                        let file = std::fs::read(client_cert.cert());
                        if let Err(e) = file {
                            return Err(DeboaError::ClientCert { message: e.to_string() });
                        }
                        let identity = Identity::from_pkcs12(&file.unwrap(), client_cert.pw());
                        if let Err(e) = identity {
                            return Err(DeboaError::ClientCert { message: e.to_string() });
                        }
                        TlsConnector::builder().identity(identity.unwrap()).build().unwrap()
                    } else {
                        TlsConnector::builder().build().unwrap()
                    };
                    let connector = tokio_native_tls::TlsConnector::from(connector);
                    let stream = connector.connect(&host.to_string(), socket).await;

                    if let Err(e) = stream {
                        return Err(DeboaError::Connection {
                            host: host.to_string(),
                            message: e.to_string(),
                        });
                    }

                    let stream = stream.unwrap();
                    TokioStream::Tls(stream)
                }
                scheme => {
                    return Err(DeboaError::UnsupportedScheme {
                        message: format!("unsupported scheme: {scheme:?}"),
                    });
                }
            }
        };

        let result = handshake(TokioIo::new(stream)).await;

        if let Err(err) = result {
            return Err(DeboaError::Connection {
                host: host.to_string(),
                message: err.to_string(),
            });
        }

        let (sender, conn) = result.unwrap();

        tokio::spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(_err) => {}
            };
        });

        Ok(BaseHttpConnection::<Self::Sender> { url: url.clone(), sender })
    }

    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>> {
        let method = request.method().to_string();
        let result = self.sender.send_request(request).await;

        self.process_response(&self.url, &method, result).await
    }
}
