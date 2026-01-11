use url::Url;

use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;

use crate::server::ServerConfig;

pub const CA_CERT: &[u8] = include_bytes!("../certs/ca.pem");
pub const SERVER_CERT: &[u8] = include_bytes!("../certs/server.pem");
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
