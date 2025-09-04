#[cfg(test)]
use crate::errors::DeboaError;
use crate::{tests::types::JSONPLACEHOLDER, Deboa};
use http::StatusCode;

//
// PUT
//

async fn do_put() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    let response = api.set_text("".to_string()).put("/posts/1").await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_put() -> Result<(), DeboaError> {
    do_put().await?;
    Ok(())
}
