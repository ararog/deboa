use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, body::Incoming, client::conn::http2::handshake};
use smol::net::TcpStream;
use smol_hyper::rt::FuturesIo;
use url::Url;

use crate::client::conn::http::DeboaHttpConnection;
use crate::rt::smol::executor::SmolExecutor;
use crate::rt::smol::stream::SmolStream;
use crate::{
    client::conn::http::{BaseHttpConnection, Http2Request},
    errors::DeboaError,
};

#[async_trait]
impl DeboaHttpConnection<Http2Request> for BaseHttpConnection<Http2Request> {
    fn url(&self) -> &Url {
        &self.url
    }

    async fn connect(url: Url) -> Result<BaseHttpConnection<Http2Request>, DeboaError> {
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
                    let stream = async_native_tls::connect(host.to_string(), stream).await;

                    if let Err(e) = stream {
                        return Err(DeboaError::Connection {
                            host: host.to_string(),
                            message: e.to_string(),
                        });
                    }

                    let stream = stream.unwrap();
                    SmolStream::Tls(stream)
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

        Ok(BaseHttpConnection::<Http2Request> { url, sender })
    }

    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>, DeboaError> {
        let method = request.method().to_string();
        let result = self.sender.send_request(request).await;

        self.process_response(self.url.clone(), &method, result)
    }
}
