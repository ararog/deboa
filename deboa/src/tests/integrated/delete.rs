#[cfg(test)]
use crate::errors::DeboaError;
use crate::{Deboa, request::DeboaRequest, tests::utils::JSONPLACEHOLDER};
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// DELETE
//

async fn do_delete() -> Result<(), DeboaError> {
    let mut client = Deboa::new();

    let response = DeboaRequest::delete(&format!("{}/posts/1", JSONPLACEHOLDER))
        .send_with(&mut client)
        .await?;

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
