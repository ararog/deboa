use crate::common::{
    data::Post,
    helpers::{create_client, start_mock_server},
};
use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::post;
use easyhttpmock_vetis_smol::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::test;
use std::error::Error;

#[apply(test!)]
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
    let client = create_client();
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

#[apply(test!)]
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
        .assert()
        .await?;
    Ok(())
}

#[apply(test!)]
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
        .assert()
        .await?;
    Ok(())
}

#[apply(test!)]
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
        .assert()
        .await?;
    Ok(())
}
