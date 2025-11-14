use crate::{request::DeboaRequest, Deboa, Result};
use http::{header, StatusCode};
use httpmock::{Method::PUT, MockServer};
#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// PUT
//

async fn do_put() -> Result<()> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(PUT)
            .path("/posts/1");
        then.status::<u16>(StatusCode::OK.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let mut client = Deboa::new();

    let request = DeboaRequest::put(
        server
            .url("/posts/1")
            .as_str(),
    )?
    .text("ping")
    .build()?;

    let response = client
        .execute(request)
        .await?;

    http_mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_put() -> Result<()> {
    do_put().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_put() -> Result<()> {
    do_put().await
}
