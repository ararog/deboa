#[cfg(test)]
use crate::errors::DeboaError;
use crate::response::DeboaResponse;
use crate::{tests::types::JSONPLACEHOLDER, Deboa};

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
    let api = Deboa::new(JSONPLACEHOLDER)?;

    let response: DeboaResponse = api.get("/posts").await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response.status().as_u16(),
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
    let api = Deboa::new(JSONPLACEHOLDER)?;

    let response: Result<DeboaResponse, DeboaError> = api.get("asasa/posts/1ddd").await;

    assert!(response.is_err());
    assert_eq!(
        response,
        Err(DeboaError::Request {
            host: "jsonplaceholder.typicode.com".to_string(),
            path: "/asasa/posts/1ddd".to_string(),
            method: "GET".to_string(),
            message: "Request failed with status code: 404 Not Found".to_string()
        })
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

    let response: Result<DeboaResponse, DeboaError> = api.get("/posts").await;

    assert!(response.is_err());
    assert_eq!(
        response,
        Err(DeboaError::Connection {
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
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    let query_map = HashMap::from([("id", "1")]);

    api.set_query_params(query_map);

    let response: DeboaResponse = api.get("/comments").await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response.status().as_u16(),
        StatusCode::OK.as_u16()
    );

    let comments = response.text();

    assert!(comments.is_ok());

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
