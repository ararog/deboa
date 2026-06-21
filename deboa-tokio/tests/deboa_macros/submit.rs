use crate::common::helpers::{create_client, start_mock_server};
use deboa_macros::submit;
use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::{Method, StatusCode};
use std::error::Error;

#[tokio::test]
async fn test_submit_str_minimal() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "POST"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;
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
        .assert()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_submit_str_method() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "POST"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;
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
        .assert()
        .await?;
    Ok(())
}
