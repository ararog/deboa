use http::{StatusCode, header};
use httpmock::{Method::GET, MockServer};

use crate::{Deboa, HttpVersion, errors::DeboaError};

pub const JSONPLACEHOLDER: &str = "https://jsonplaceholder.typicode.com";

pub fn format_address(server: &MockServer) -> String {
    let server_address = *server.address();

    let ip = server_address.ip();
    let port = server_address.port();

    format!("http://{ip}:{port}")
}

pub fn setup_server(server: &MockServer, protocol: HttpVersion) -> Result<(httpmock::Mock<'_>, Deboa), DeboaError> {
    let http_mock = server.mock(|when, then| {
        when.method(GET).path("/posts");
        then.status::<u16>(StatusCode::OK.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let mut api = Deboa::new(&format_address(server))?;
    api.set_protocol(protocol);

    Ok((http_mock, api))
}
