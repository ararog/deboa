use http::{StatusCode, header};
use httpmock::{Method::GET, MockServer};

use crate::errors::DeboaError;

pub const JSONPLACEHOLDER: &str = "https://jsonplaceholder.typicode.com";

pub fn format_address(server: &MockServer) -> String {
    let server_address = *server.address();

    let ip = server_address.ip();
    let port = server_address.port();

    format!("http://{ip}:{port}")
}

pub fn setup_server(server: &MockServer) -> Result<httpmock::Mock<'_>, DeboaError> {
    let http_mock = server.mock(|when, then| {
        when.method(GET).path("/posts");
        then.status::<u16>(StatusCode::OK.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    Ok(http_mock)
}
