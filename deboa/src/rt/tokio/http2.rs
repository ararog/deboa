use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http2::handshake, Request, Response};
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio_native_tls::native_tls::TlsConnector;
use url::{Host, Url};

use crate::client::conn::http::DeboaHttpConnection;
use crate::rt::tokio::stream::TokioStream;
use crate::{
    client::conn::http::{BaseHttpConnection, Http2Request},
    errors::DeboaError,
    Result,
};

#[async_trait]
impl DeboaHttpConnection for BaseHttpConnection<Http2Request> {
    type Sender = Http2Request;

    #[inline]
    fn url(&self) -> &Url {
        &self.url
    }

    async fn connect(url: &Url) -> Result<BaseHttpConnection<Http2Request>> {
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
                    let cx = TlsConnector::builder().build().unwrap();
                    let cx = tokio_native_tls::TlsConnector::from(cx);

                    let stream = cx.connect(&host.to_string(), socket).await;

                    if let Err(e) = stream {
                        return Err(DeboaError::Connection {
                            host: host.to_string(),
                            message: e.to_string(),
                        });
                    }

                    TokioStream::Tls(stream.unwrap())
                }
                scheme => {
                    return Err(DeboaError::UnsupportedScheme {
                        message: format!("unsupported scheme: {scheme:?}"),
                    });
                }
            }
        };

        let result = handshake(TokioExecutor::new(), TokioIo::new(stream)).await;

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

        Ok(BaseHttpConnection::<Http2Request> { url: url.clone(), sender })
    }

    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>> {
        let method = request.method().to_string();
        let result = self.sender.send_request(request).await;

        self.process_response(&self.url, &method, result).await
    }
}
