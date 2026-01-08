use url::Url;

use bytes::Bytes;
use http::{header, StatusCode};
use http_body_util::Full;
use httpmock::{Method, MockServer};

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

pub fn setup_server<'a>(
    server: &'a MockServer,
    path: &'a str,
    method: Method,
    status: StatusCode,
) -> httpmock::Mock<'a> {
    server.mock(|when, then| {
        when.method(method)
            .path(path);
        then.status::<u16>(status.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("pong");
    })
}

pub fn setup_server_with_body<'a>(
    server: &'a MockServer,
    path: &'a str,
    method: Method,
    status: StatusCode,
    body: &'a str,
) -> httpmock::Mock<'a> {
    server.mock(|when, then| {
        when.method(method)
            .path(path)
            .body(body);
        then.status::<u16>(status.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("pong");
    })
}
