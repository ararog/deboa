use std::error::Error;

use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::get;
use deboa_tokio::Client;
use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;

use crate::common::{data::Post, helpers::start_mock_server};

#[tokio::test]
async fn test_get_minimal() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let response = get!(
        url => server.url("/posts"),
        client => &client
    );
    assert!(!response
        .text()
        .await?
        .is_empty());
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_get_minimal_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let response = get!(
        url => server.url("/posts"),
        headers => vec![("Content-Type", "application/json")],
        client => &client
    );
    assert!(!response
        .text()
        .await?
        .is_empty());
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_get() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let response = get!(
        url => server.url("/posts"),
        client => &client,
        res_body_ty => JsonBody,
        res_ty => Vec<Post>
    );
    assert_eq!(response.len(), 1);
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_get_with_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let response = get!(
        url => server.url("/posts"),
        headers => vec![("User-Agent", "deboa")],
        client => &client,
        res_body_ty => JsonBody,
        res_ty => Vec<Post>
    );
    assert_eq!(response.len(), 1);
    server
        .assert()
        .await?;
    Ok(())
}
