use std::error::Error;

use deboa_macros::delete;
use deboa_smol::Client;

use easyhttpmock_vetis_smol::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::test;

use crate::common::helpers::start_mock_server;

#[apply(test!)]
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

#[apply(test!)]
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
    let response = delete!(
        url => server.url("/posts/1"),
        headers => vec![("User-Agent", "deboa")],
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
