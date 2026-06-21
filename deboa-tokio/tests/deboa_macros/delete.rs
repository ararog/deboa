use crate::common::helpers::start_mock_server;
use deboa_macros::delete;
use deboa_tokio::Client;
use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;
use std::error::Error;

#[tokio::test]
async fn delete() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "DELETE"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let response = delete!(
        url => server.url("/posts/1"),
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
async fn delete_with_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "DELETE"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = Client::default();
    let headers = vec![("User-Agent", "deboa")];
    let response = delete!(
        url => server.url("/posts/1"),
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
