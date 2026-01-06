#[cfg(test)]
use crate::{request::DeboaRequest, Client, Result};
#[cfg(feature = "http3-tokio")]
use crate::{response::DeboaResponse, HttpVersion};
use deboa_tests::utils::JSONPLACEHOLDER;
use http::StatusCode;

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
    let client = Client::builder()
        .protocol(HttpVersion::Http3)
        .build();

    let request = DeboaRequest::delete(format!("{}/posts/1", JSONPLACEHOLDER))?.build()?;

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
    let client = Client::default();

    let response = DeboaRequest::delete(format!("{}/posts/1", JSONPLACEHOLDER))?
        .send_with(client)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

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
