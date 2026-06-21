use crate::common::data::Post;
use crate::common::helpers::start_mock_server;
use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::post;
use deboa_tokio::Client;
use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;
use std::error::Error;

#[tokio::test]
async fn test_only_post_minimal() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "POST"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::CREATED
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = post!(
        data => data,
        url => server.url("/posts"),
        client => &client
    );
    assert_eq!(response.status(), 201);
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_only_post_minimal_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "POST"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::CREATED
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = post!(
        data => data,
        url => server.url("/posts"),
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 201);
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_only_post() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "POST"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::CREATED
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = post!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts"),
        client => &client
    );
    assert_eq!(response.status(), 201);
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_post_with_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "POST"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::CREATED
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = post!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts"),
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 201);
    server
        .assert()
        .await?;
    Ok(())
}
