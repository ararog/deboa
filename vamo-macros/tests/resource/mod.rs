use deboa::{
    cert::{Certificate, ContentEncoding},
    client::serde::RequestBody,
    Client as DeboaClient,
};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tests::{
    mock_response,
    utils::{start_mock_server, CA_CERT},
};

use http::StatusCode;
use serde::Serialize;
use vamo::{resource::ResourceMethod, Vamo};
use vamo_macros::Resource;

use crate::SKIP_CERT_VERIFICATION;

#[derive(Resource, Serialize)]
#[name("users")]
#[body_type(JsonBody)]
pub struct User {
    #[rid]
    id: i32,
    name: String,
}

async fn do_post_resource() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "POST" && req.uri().path() == "/api/users" {
            Ok(mock_response(StatusCode::CREATED, ""))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let mut user = User { id: 32, name: "User 1".to_string() };

    let mut url = server.base_url();
    url.push_str("/api");

    let client = DeboaClient::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(url.to_string())?;
    vamo.client(client);
    let response = vamo
        .create(&mut user)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    server
        .stop()
        .await?;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_post_resource() -> Result<(), Box<dyn std::error::Error>> {
    do_post_resource().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_post_resource() -> Result<(), Box<dyn std::error::Error>> {
    do_post_resource().await
}
