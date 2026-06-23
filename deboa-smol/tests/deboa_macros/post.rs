use crate::common::{
    data::Post,
    helpers::{create_client, create_server},
};
use deboa_extras::serde::json::JsonBody;
use deboa_macros::post;
use easyhttpmock_vetis_smol::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::test;
use std::error::Error;

#[apply(test!)]
async fn test_only_post_minimal() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("POST").and(path("/posts"))).will_return(
            StatusCode::CREATED
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
    let response = post!(
        data => data,
        url => server.url("/posts"),
        client => &client
    );
    assert_eq!(response.status(), 201);
    server
        .stop()
        .await?;
    Ok(())
}

#[apply(test!)]
async fn test_only_post_minimal_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("POST").and(path("/posts"))).will_return(
            StatusCode::CREATED
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
    let response = post!(
        data => data,
        url => server.url("/posts"),
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 201);
    server
        .stop()
        .await?;
    Ok(())
}

#[apply(test!)]
async fn test_only_post() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("POST").and(path("/posts"))).will_return(
            StatusCode::CREATED
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
    let response = post!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts"),
        client => &client
    );
    assert_eq!(response.status(), 201);
    server
        .stop()
        .await?;
    Ok(())
}

#[apply(test!)]
async fn test_post_with_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("POST").and(path("/posts"))).will_return(
            StatusCode::CREATED
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
    let response = post!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts"),
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 201);
    server
        .stop()
        .await?;
    Ok(())
}
