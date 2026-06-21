use crate::common::{
    data::{Post, PostWithId},
    helpers::start_mock_server,
};
use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::patch;
use deboa_tokio::Client;
use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;
use std::error::Error;

#[tokio::test]
async fn test_only_patch_minimal() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PATCH"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let data: PostWithId = PostWithId { id: 1 };
    let response = patch!(
        data => data,
        url => server.url("/posts/1"),
        client => &client
    );
    assert_eq!(response.status(), 200);
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_only_patch_minimal_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PATCH"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let response = patch!(
        data => Post { id: 1, title: "title".to_string(), body: "body".to_string() },
        url => server.url("/posts/1"),
        headers => vec![("Content-Type", "application/json")],
        client => &client
    );
    assert_eq!(response.status(), 200);
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_patch() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PATCH"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(
        data => data,
        url => server.url("/posts/1"),
        client => &client
    );
    assert_eq!(response.status(), 200);
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_patch_with_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PATCH"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(
        data => data,
        url => server.url("/posts/1"),
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 200);
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_patch_with_json_body_request() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PATCH"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts/1"),
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 200);
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_patch_with_json_body_no_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PATCH"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"{\"id\": 1, \"title\": \"Teste\", \"body\": \"Teste\"}"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts/1"),
        client => &client,
        res_body_ty => JsonBody,
        res_ty => PostWithId
    );
    assert_eq!(response.id, 1);
    server
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_patch_with_json_body_response() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PATCH"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"{\"id\": 1, \"title\": \"Teste\", \"body\": \"Teste\"}"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts/1"),
        headers => headers,
        client => &client,
        res_body_ty => JsonBody,
        res_ty => Post
    );
    assert_eq!(response.id, 1);
    server
        .assert()
        .await?;
    Ok(())
}
