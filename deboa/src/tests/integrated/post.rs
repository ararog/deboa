use crate::Deboa;
use crate::{errors::DeboaError, tests::utils::format_address};
use http::{StatusCode, header};
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
        then.status::<u16>(StatusCode::CREATED.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let mut api = Deboa::new(&format_address(&server))?;

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
    do_post().await
}

#[cfg(feature = "compio-rt")]
#[compio::test]
async fn test_post() -> Result<(), DeboaError> {
    let _ = do_post().await;
    Ok(())
}
