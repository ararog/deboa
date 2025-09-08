#[cfg(test)]
use crate::errors::DeboaError;
use crate::{Deboa, tests::utils::JSONPLACEHOLDER};
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// DELETE
//

async fn do_delete() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    let response = api.delete("/posts/1").await?;

    assert_eq!(response.status(), StatusCode::OK);

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
