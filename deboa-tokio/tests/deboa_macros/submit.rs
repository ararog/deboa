#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server};
use deboa_macros::submit;
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::{Method, StatusCode};
use std::error::Error;

#[tokio::test]
async fn test_submit_str_minimal() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("POST").and(path("/posts"))).will_return(
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

    let response = submit!(
        method => Method::POST,
        data => "user=deboa",
        url => server.url("/posts"),
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
async fn test_submit_str_method() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("POST").and(path("/posts"))).will_return(
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

    let headers = vec![("Content-Type", "application/x-www-form-urlencoded")];
    let response = submit!(
        method => Method::POST,
        data => "user=deboa",
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
