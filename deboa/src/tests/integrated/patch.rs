use crate::Deboa;
use crate::errors::DeboaError;
use crate::tests::utils::JSONPLACEHOLDER;
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

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

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_patch() -> Result<(), DeboaError> {
    do_patch().await
}

#[cfg(feature = "compio-rt")]
#[compio::test]
async fn test_patch() -> Result<(), DeboaError> {
    let _ = do_patch().await;
    Ok(())
}
