use url::Url;

use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;

pub const TEST_HOST: &str = "http://localhost/";

pub fn fake_url() -> Url {
    Url::parse("http://test.com/get").unwrap()
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
