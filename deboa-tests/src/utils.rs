use url::Url;

use crate::server::ServerConfig;
use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;

#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
use crate::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", any(feature = "http1", feature = "http2")))]
use crate::server::tcp::smol::HttpServer;

#[cfg(all(feature = "tokio-rt", feature = "http3"))]
use crate::server::udp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", feature = "http3"))]
use crate::server::udp::smol::HttpServer;

use http::{Request, Response};
use hyper::body::Incoming;

pub const CA_CERT: &[u8] = include_bytes!("../certs/ca.cert");
pub const SERVER_CERT: &[u8] = include_bytes!("../certs/server.cert");
pub const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key");

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
    rand::random_range(20000..65535)
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

pub async fn start_mock_server(
    #[cfg(not(feature = "http3"))] handler: fn(
        Request<Incoming>,
    ) -> std::result::Result<
        Response<Full<Bytes>>,
        hyper::Error,
    >,
    #[cfg(feature = "http3")] handler: fn(
        Request<Full<Bytes>>,
    ) -> std::result::Result<
        Response<Full<Bytes>>,
        hyper::Error,
    >,
) -> HttpServer {
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
