#[cfg(feature = "json")]
use crate::tests::types::Post;
use crate::Deboa;
#[cfg(test)]
use crate::DeboaError;
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// PUT
//

async fn do_put() -> Result<(), DeboaError> {
    #[cfg(feature = "json")]
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    #[cfg(not(feature = "json"))]
    let api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    #[cfg(feature = "json")]
    let post = Post {
        id: 1,
        title: "Test".to_string(),
        body: "Some test to do".to_string(),
    };

    #[cfg(feature = "json")]
    let response = api.set_json(post)?.put("/posts/1").await?;

    #[cfg(not(feature = "json"))]
    let response = api.put("/posts/1").await?;

    assert_eq!(response.status, StatusCode::OK);

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
async fn test_put() {
    let _ = do_put().await;
}

#[cfg(feature = "compio-rt")]
#[compio::test]
async fn test_put() {
    let _ = do_put().await;
}
