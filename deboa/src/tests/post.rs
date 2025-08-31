use crate::errors::DeboaError;
use crate::Deboa;
use http::{header, StatusCode};

use httpmock::{Method::POST, MockServer};
#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// POST
//

async fn do_post() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(POST).path("/posts");
        then.status(StatusCode::CREATED.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let server_address = *server.address();

    let ip = server_address.ip();
    let port = server_address.port();

    let mut api = Deboa::new(&format!("http://{ip}:{port}"))?;

    let data = "ping".to_string();
    let response = api.set_text(data).post("/posts").await?;

    http_mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(response.raw_body(), b"ping");

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_post() -> Result<(), DeboaError> {
    do_post().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_post() -> Result<(), DeboaError> {
    do_post().await?;
    Ok(())
}

#[cfg(feature = "compio-rt")]
#[compio::test]
async fn test_post() -> Result<(), DeboaError> {
    do_post().await?;
    Ok(())
}
