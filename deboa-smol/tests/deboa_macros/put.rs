#![allow(unused_variables)]
use crate::common::{
    data::{Post, PostWithId},
    helpers::{create_client, create_server},
};
use deboa_extras::serde::json::JsonBody;
use deboa_macros::put;
use easyhttpmock_vetis_smol::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::test;
use std::error::Error;

#[apply(test!)]
async fn test_only_put_minimal() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = put!(
        data => data,
        url => server.url("/posts/1"),
        client => &client
    );
    assert_eq!(response.status(), 200);
    server
        .stop()
        .await?;
    Ok(())
}

#[apply(test!)]
async fn test_only_put_minimal_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
        data => data,
        url => server.url("/posts/1"),
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 200);
    server
        .stop()
        .await?;
    Ok(())
}

#[apply(test!)]
async fn test_put() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = put!(
        data => data,
        url => server.url("/posts/1"),
        client => &client
    );
    assert_eq!(response.status(), 200);
    server
        .stop()
        .await?;
    Ok(())
}

#[apply(test!)]
async fn test_put_with_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
        data => data,
        url => server.url("/posts/1"),
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 200);
    server
        .stop()
        .await?;
    Ok(())
}

#[apply(test!)]
async fn test_put_with_json_body_request() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts/1"),
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 200);
    server
        .stop()
        .await?;
    Ok(())
}

#[apply(test!)]
async fn test_put_with_json_body_no_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{\"id\": 1, \"title\": \"Teste\", \"body\": \"Teste\"}"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = put!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts/1"),
        client => &client,
        res_body_ty => JsonBody,
        res_ty => PostWithId
    );
    assert_eq!(response.id, 1);
    server
        .stop()
        .await?;
    Ok(())
}

#[apply(test!)]
async fn test_put_with_json_body_response() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{\"id\": 1, \"title\": \"Teste\", \"body\": \"Teste\"}"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
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
        .stop()
        .await?;
    Ok(())
}
