use url::Url;

use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;

use crate::server::ServerConfig;

pub const TEST_HOST: &str = "http://localhost/";

pub fn fake_url() -> Url {
    Url::parse("http://test.com/get").unwrap()
}

pub fn generate_port() -> u16 {
    rand::random_range(20000..65535)
}

pub fn tls_server_config() -> Option<ServerConfig> {
    Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ))
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
