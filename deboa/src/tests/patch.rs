#[cfg(feature = "json")]
use crate::tests::types::{sample_post, JSONPLACEHOLDER};
use crate::{Deboa, DeboaError};
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// PATCH
//

async fn do_patch() -> Result<(), DeboaError> {
    #[cfg(feature = "json")]
    let mut api: Deboa = Deboa::new(JSONPLACEHOLDER)?;

    #[cfg(not(feature = "json"))]
    let api = Deboa::new(JSONPLACEHOLDER)?;

    #[cfg(feature = "json")]
    let data = sample_post();

    #[cfg(feature = "json")]
    let response = api.set_json(data)?.patch("/posts/1").await?;

    #[cfg(not(feature = "json"))]
    let response = api.patch("/posts/1").await?;

    assert_eq!(response.status, StatusCode::OK);

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
    do_patch().await?;
    Ok(())
}

#[cfg(feature = "compio-rt")]
#[compio::test]
async fn test_patch() -> Result<(), DeboaError> {
    do_patch().await?;
    Ok(())
}
