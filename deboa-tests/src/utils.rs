use url::Url;

use http::{header, StatusCode};
use httpmock::{Method, MockServer};

pub const JSONPLACEHOLDER: &str = "https://jsonplaceholder.typicode.com/";

pub fn fake_url() -> Url {
    Url::parse("http://test.com/get").unwrap()
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
        when.method(method).path(path);
        then.status::<u16>(status.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    })
}
