use crate::Deboa;
use crate::errors::DeboaError;
use crate::request::DeboaRequest;
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
    let mut client: Deboa = Deboa::new();

    let request = DeboaRequest::patch(format!("{JSONPLACEHOLDER}/posts/1").as_str()).text("").build()?;

    let response = client.execute(request).await?;

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
