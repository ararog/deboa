use crate::{request::DeboaRequest, Client, Result};
use deboa_tests::utils::JSONPLACEHOLDER;
use http::StatusCode;
#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// PUT
//

async fn do_put() -> Result<()> {
    let client = Client::default();

    let request = DeboaRequest::put(format!("{}/posts/1", JSONPLACEHOLDER).as_str())?
        .text("ping")
        .build()?;

    let response = client
        .execute(request)
        .await?;

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
