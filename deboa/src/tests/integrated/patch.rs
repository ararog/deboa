use crate::errors::DeboaError;
use crate::tests::utils::JSONPLACEHOLDER;
use crate::Deboa;
use http::StatusCode;

//
// PATCH
//

async fn do_patch() -> Result<(), DeboaError> {
    let mut api: Deboa = Deboa::new(JSONPLACEHOLDER)?;

    let response = api.set_text("".to_string()).patch("/posts/1").await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_patch() -> Result<(), DeboaError> {
    do_patch().await?;
    Ok(())
}
