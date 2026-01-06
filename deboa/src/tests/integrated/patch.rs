use crate::{request::DeboaRequest, Client, Result};
use deboa_tests::utils::JSONPLACEHOLDER;
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// PATCH
//

async fn do_patch() -> Result<()> {
    let client: Client = Client::default();

    let request = DeboaRequest::patch(format!("{}/posts/1", JSONPLACEHOLDER))?
        .text("")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_patch() -> Result<()> {
    do_patch().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_patch() -> Result<()> {
    do_patch().await
}
