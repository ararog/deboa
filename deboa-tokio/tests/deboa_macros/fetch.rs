use crate::common::{data::Post, helpers::start_mock_server};
use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::fetch;
use deboa_tokio::Client;
use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;
use std::error::Error;

#[tokio::test]
async fn test_fetch_str_minimal() -> Result<(), Box<dyn Error>> {
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
    let response = fetch!(url => server.url("/posts"), client => &client);
    assert!(response
        .status()
        .is_success());
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_fetch_str_minimal_headers() -> Result<(), Box<dyn Error>> {
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
    let headers = vec![("User-Agent", "deboa")];
    let response = fetch!(
        url => server.url("/posts"),
        headers => headers,
        client => &client
    );
    assert!(response
        .status()
        .is_success());
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_fetch_str() -> Result<(), Box<dyn Error>> {
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
    let response = fetch!(
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
async fn test_fetch_ident() -> Result<(), Box<dyn Error>> {
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
    let url = server.url("/posts");
    let response = fetch!(
        url => url,
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
async fn test_fetch_ident_with_headers() -> Result<(), Box<dyn Error>> {
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
    let url = server.url("/posts");
    let headers = vec![("User-Agent", "deboa")];
    let response = fetch!(
        url => url,
        headers => headers,
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
