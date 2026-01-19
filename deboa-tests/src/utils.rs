use std::future::Future;

#[cfg(any(feature = "http1", feature = "http2"))]
use hyper::body::Incoming;
use url::Url;

use crate::server::errors::EasyHttpMockError;
use crate::server::{Server, ServerConfig};
use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::Full;

#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
use crate::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", any(feature = "http1", feature = "http2")))]
use crate::server::tcp::smol::HttpServer;

#[cfg(all(feature = "tokio-rt", feature = "http3"))]
use crate::server::udp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", feature = "http3"))]
use crate::server::udp::smol::HttpServer;

pub const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub const CA_CERT_PEM: &[u8] = include_bytes!("../certs/ca.crt");

pub const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

pub const SERVER_CERT_PEM: &[u8] = include_bytes!("../certs/server.crt");
pub const SERVER_KEY_PEM: &[u8] = include_bytes!("../certs/server.key");

pub const CLIENT_CERT: &[u8] = include_bytes!("../certs/client.der");
pub const CLIENT_KEY: &[u8] = include_bytes!("../certs/client.key.der");

pub const CLIENT_CERT_PEM: &[u8] = include_bytes!("../certs/client.crt");
pub const CLIENT_KEY_PEM: &[u8] = include_bytes!("../certs/client.key");

pub const CLIENT_P12: &[u8] = include_bytes!("../certs/client.p12");

const TEST_URL: &str = "https://localhost";

pub fn test_url(port: Option<u16>) -> String {
    if let Some(port) = port {
        format!("{}:{}", TEST_URL, port)
    } else {
        TEST_URL.to_string()
    }
}

pub fn fake_url() -> Url {
    Url::parse("http://test.com/get").unwrap()
}

pub fn generate_port() -> u16 {
    rand::random_range(9000..65535)
}

pub fn tls_server_config() -> Option<ServerConfig> {
    Some(ServerConfig::new(Some(SERVER_CERT.to_vec()), Some(SERVER_KEY.to_vec())))
}

pub fn make_response(status: StatusCode, body: &[u8]) -> http::Response<Full<Bytes>> {
    http::Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(body.to_vec())))
        .unwrap()
}

pub fn url_from_string(url: String) -> Url {
    url.parse().unwrap()
}

#[cfg(any(feature = "http1", feature = "http2"))]
type RequestType = Request<Incoming>;

#[cfg(feature = "http3")]
type RequestType = Request<Full<Bytes>>;

#[cfg(any(feature = "http1", feature = "http2"))]
type ResponseType = Response<Full<Bytes>>;

#[cfg(feature = "http3")]
type ResponseType = Response<Full<Bytes>>;

pub async fn start_mock_server<H, Fut>(handler: H) -> HttpServer
where
    H: Fn(RequestType) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<ResponseType, EasyHttpMockError>> + Send + 'static,
{
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    let result = server
        .start(handler)
        .await;

    result.unwrap_or_else(|err| {
        panic!("Failed to start mock server: {}", err);
    });

    server
}
