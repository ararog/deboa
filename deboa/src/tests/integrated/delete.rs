#[cfg(test)]
use crate::{request::DeboaRequest, Deboa, Result};
use http::StatusCode;

use httpmock::{Method::DELETE, MockServer};
#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// DELETE
//

async fn do_delete() -> Result<()> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(DELETE).path("/posts/1");
        then.status::<u16>(StatusCode::NO_CONTENT.into());
    });

    let client = Deboa::new();

    let response = DeboaRequest::delete(server.url("/posts/1").as_str())?
        .with(client)
        .await?;

    http_mock.assert();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_delete() -> Result<()> {
    do_delete().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_delete() -> Result<()> {
    do_delete().await
}
