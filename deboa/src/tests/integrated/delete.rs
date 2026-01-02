#[cfg(test)]
use crate::{request::DeboaRequest, Client, Result};
#[cfg(feature = "http3")]
use crate::{response::DeboaResponse, HttpVersion};
use http::StatusCode;

use httpmock::{Method::DELETE, MockServer};
#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// DELETE
//
#[cfg(feature = "http3-tokio")]
#[tokio::test]

async fn delete_http3() -> Result<()> {
    let mut client = Client::builder()
        .protocol(HttpVersion::Http3)
        .build();

    let request = DeboaRequest::delete("https://jsonplaceholder.typicode.com/posts/1")?.build()?;

    let response: DeboaResponse = client
        .execute(request)
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response
            .status()
            .as_u16(),
        StatusCode::OK.as_u16()
    );

    Ok(())
}

async fn do_delete() -> Result<()> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(DELETE)
            .path("/posts/1");
        then.status::<u16>(StatusCode::NO_CONTENT.into());
    });

    let client = Client::default();

    let response = DeboaRequest::delete(
        server
            .url("/posts/1")
            .as_str(),
    )?
    .send_with(client)
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
