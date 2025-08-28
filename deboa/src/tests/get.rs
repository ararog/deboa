#[cfg(feature = "json")]
use crate::tests::types::Comment;
use crate::Deboa;
#[cfg(test)]
use crate::DeboaError;

use http::StatusCode;
use std::collections::HashMap;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// GET
//

async fn do_get() -> Result<(), DeboaError> {
    let api = Deboa::new("https://jsonplaceholder.typicode.com")?;

    let response = api.get("/posts").await?;

    assert_eq!(
        response.status,
        StatusCode::OK,
        "Status code is {} and should be {}",
        response.status.as_u16(),
        StatusCode::OK.as_u16()
    );

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get() -> Result<(), DeboaError> {
    do_get().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get() {
    let _ = do_get().await;
}

#[cfg(feature = "compio-rt")]
#[compio::test]
async fn test_get() {
    let _ = do_get().await;
}

//
// GET NOT FOUND
//

async fn do_get_not_found() -> Result<(), DeboaError> {
    let api = Deboa::new("https://jsonplaceholder.typicode.com/dsdsd")?;

    let response = api.get("/posts").await?;

    assert_eq!(
        response.status,
        StatusCode::NOT_FOUND,
        "Status code is {} and should be {}",
        response.status.as_u16(),
        StatusCode::NOT_FOUND.as_u16()
    );

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_not_found() -> Result<(), DeboaError> {
    do_get_not_found().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_not_found() {
    let _ = do_get_not_found().await;
}

#[cfg(feature = "compio-rt")]
#[compio::test]
async fn test_get_not_found() {
    let _ = do_get_not_found().await;
}

//
// GET INVALID SERVER
//

async fn do_get_invalid_server() -> Result<(), DeboaError> {
    let api = Deboa::new("https://invalid-server.com")?;

    let response = api.get("/posts").await;

    assert!(response.is_err());
    assert_eq!(
        response,
        Err(DeboaError::ConnectionError {
            host: "invalid-server.com".to_string(),
            message: "failed to lookup address information: Name or service not known".to_string(),
        })
    );

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_invalid_server() -> Result<(), DeboaError> {
    do_get_invalid_server().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_invalid_server() {
    let _ = do_get_invalid_server().await;
}

#[cfg(feature = "compio-rt")]
#[compio::test]
async fn test_get_invalid_server() {
    let _ = do_get_invalid_server().await;
}

//
// GET BY QUERY
//

async fn do_get_by_query() -> Result<(), DeboaError> {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;

    let query_map = HashMap::from([("id", "1")]);

    api.set_query_params(Some(query_map));

    #[cfg(feature = "json")]
    let mut response = api.get("/comments").await?;

    #[cfg(not(feature = "json"))]
    let response = api.get("/comments").await?;

    assert_eq!(
        response.status,
        StatusCode::OK,
        "Status code is {} and should be {}",
        response.status.as_u16(),
        StatusCode::OK.as_u16()
    );

    #[cfg(feature = "json")]
    let comments = response.json::<Vec<Comment>>().await?;

    #[cfg(feature = "json")]
    assert_eq!(comments.len(), 1, "Number of comments is {} and should be {}", comments.len(), 1);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_by_query() -> Result<(), DeboaError> {
    do_get_by_query().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_by_query() -> Result<(), DeboaError> {
    do_get_by_query().await
}

#[cfg(feature = "compio-rt")]
#[compio::test]
async fn test_get_by_query() -> Result<(), DeboaError> {
    do_get_by_query().await
}
