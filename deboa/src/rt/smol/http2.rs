use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use futures_rustls::TlsConnector;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http2::handshake, Request, Response};
use rustls::pki_types::ServerName;
use smol::net::TcpStream;
use smol_hyper::rt::FuturesIo;
use url::Url;

use crate::{
    client::conn::http::{BaseHttpConnection, DeboaHttpConnection, Http2Request},
    errors::DeboaError,
    rt::smol::{executor::SmolExecutor, stream::SmolStream},
    Result,
};

#[async_trait]
impl DeboaHttpConnection for BaseHttpConnection<Http2Request> {
    type Sender = Http2Request;

    #[inline]
    fn url(&self) -> &Url {
        &self.url
    }

    async fn connect(url: &Url) -> Result<BaseHttpConnection<Self::Sender>> {
        let host = url.host().expect("uri has no host");
        let io = {
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

                    let stream = stream.unwrap();
                    SmolStream::Plain(stream)
                }
                "https" => {
                    // In case of HTTPS, establish a secure TLS connection first.
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

                    let stream = stream.unwrap();
                    let root_store = rustls::RootCertStore {
                        roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
                    };
                    let config = rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth();
                    let config = TlsConnector::from(Arc::new(config));
                    let stream = config.connect(ServerName::try_from(host.to_string()).unwrap(), stream).await;
                    
                    if let Err(e) = stream {
                        return Err(DeboaError::Connection {
                            host: host.to_string(),
                            message: e.to_string(),
                        });
                    }

                    let stream = stream.unwrap();
                    SmolStream::Tls(futures_rustls::TlsStream::Client(stream))
                }
                scheme => {
                    return Err(DeboaError::UnsupportedScheme {
                        message: format!("unsupported scheme: {scheme:?}"),
                    });
                }
            }
        };

        let result = handshake(SmolExecutor::new(), FuturesIo::new(io)).await;

        let (sender, conn) = result.unwrap();

        smol::spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(_err) => {}
            };
        })
        .detach();

        Ok(BaseHttpConnection::<Self::Sender> { url: url.clone(), sender })
    }

    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>> {
        let method = request.method().to_string();
        let result = self.sender.send_request(request).await;

        self.process_response(&self.url, &method, result).await
    }
}
