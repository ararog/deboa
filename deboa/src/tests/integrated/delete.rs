#[cfg(test)]
use crate::errors::DeboaError;
use crate::{tests::types::JSONPLACEHOLDER, Deboa};
use http::StatusCode;

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
