use crate::common::helpers::{create_client, create_server};
use deboa_macros::delete;
use easyhttpmock_vetis_smol::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::test;
use std::error::Error;

#[apply(test!)]
async fn delete() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("DELETE").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await;
    let client = create_client();
    let response = delete!(
        url => server.url("/posts/1"),
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

#[apply(test!)]
async fn delete_with_headers() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("DELETE").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await;
    let client = create_client();
    let response = delete!(
        url => server.url("/posts/1"),
        headers => vec![("User-Agent", "deboa")],
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
