use deboa::{
    cert::{Certificate, ContentEncoding},
    client::serde::RequestBody,
    Client as DeboaClient, Result,
};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tests::utils::{make_response, tls_server_config, CA_CERT};

#[cfg(all(feature = "_tokio-rt", any(feature = "_http1", feature = "_http2")))]
use deboa_tests::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "_smol-rt", any(feature = "_http1", feature = "_http2")))]
use deboa_tests::server::tcp::smol::HttpServer;

#[cfg(all(feature = "_tokio-rt", feature = "_http3"))]
use deboa_tests::server::udp::tokio::HttpServer;

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

async fn do_post_resource() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "POST" && req.uri().path() == "/api/users" {
                Ok(make_response(StatusCode::CREATED, b""))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
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

    server.stop().await;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_post_resource() -> Result<()> {
    do_post_resource().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_post_resource() -> Result<()> {
    do_post_resource().await
}
