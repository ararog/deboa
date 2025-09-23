use url::Url;

use http::{StatusCode, header};
use httpmock::{Method::GET, MockServer};

use crate::errors::DeboaError;

pub const JSONPLACEHOLDER: &str = "https://jsonplaceholder.typicode.com/";

pub fn url() -> Url {
    Url::parse("http://test.com/get").unwrap()
}

pub fn url_from_string(url: String) -> Url {
    url.parse().unwrap()
}

pub fn setup_server<'a>(server: &'a MockServer, path: &'a str, status: StatusCode) -> Result<httpmock::Mock<'a>, DeboaError> {
    let http_mock = server.mock(|when, then| {
        when.method(GET).path(path);
        then.status::<u16>(status.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    Ok(http_mock)
}
