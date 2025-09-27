use crate::{errors::DeboaError, request::DeboaRequest, Deboa};
use http::{header, StatusCode};

use httpmock::Method::PATCH;
use httpmock::MockServer;
#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// PATCH
//

async fn do_patch() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(PATCH).path("/posts/1");
        then.status::<u16>(StatusCode::OK.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let mut client: Deboa = Deboa::new();

    let request = DeboaRequest::patch(server.url("/posts/1").as_str())?.text("").build()?;

    let response = client.execute(request).await?;

    http_mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_patch() -> Result<(), DeboaError> {
    do_patch().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_patch() -> Result<(), DeboaError> {
    do_patch().await
}
