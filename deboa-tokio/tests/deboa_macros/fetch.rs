use crate::common::{
    data::Post,
    helpers::{create_client, create_server},
};
use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::fetch;
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::StatusCode;
use std::error::Error;

#[tokio::test]
async fn test_fetch_str_minimal() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let response = fetch!(url => server.url("/posts"), client => &client);
    assert!(response
        .status()
        .is_success());

    server
        .stop()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_fetch_str_minimal_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

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
        .stop()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_fetch_str() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let response = fetch!(
        url => server.url("/posts"),
        client => &client,
        res_body_ty => JsonBody,
        res_ty => Vec<Post>
    );
    assert_eq!(response.len(), 1);

    server
        .stop()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_fetch_ident() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let url = server.url("/posts");
    let response = fetch!(
        url => url,
        client => &client,
        res_body_ty => JsonBody,
        res_ty => Vec<Post>
    );
    assert_eq!(response.len(), 1);

    server
        .stop()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_fetch_ident_with_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

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
        .stop()
        .await?;
    Ok(())
}
