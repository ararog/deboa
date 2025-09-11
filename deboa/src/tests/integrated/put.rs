#[cfg(test)]
use crate::errors::DeboaError;
use crate::{Deboa, request::DeboaRequest, tests::utils::JSONPLACEHOLDER};
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// PUT
//

async fn do_put() -> Result<(), DeboaError> {
    let mut client = Deboa::new();

    let request = DeboaRequest::put(format!("{JSONPLACEHOLDER}/posts/1").as_str()).text("").build()?;

    let response = client.execute(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_put() -> Result<(), DeboaError> {
    do_put().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_put() -> Result<(), DeboaError> {
    do_put().await
}
