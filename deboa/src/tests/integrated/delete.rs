#[cfg(test)]
use crate::errors::DeboaError;
use crate::{Deboa, request::DeboaRequest};
use http::StatusCode;

use httpmock::{Method::DELETE, MockServer};
#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// DELETE
//

async fn do_delete() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(DELETE).path("/posts/1");
        then.status::<u16>(StatusCode::NO_CONTENT.into());
    });

    let client = Deboa::new();

    let response = DeboaRequest::delete(server.url("/posts/1").as_str())?.go(client).await?;

    http_mock.assert();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_delete() -> Result<(), DeboaError> {
    do_delete().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_delete() -> Result<(), DeboaError> {
    do_delete().await
}
