use crate::{cert::Certificate, request::DeboaRequest, Client, Result};

#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::smol::HttpServer;

#[cfg(all(feature = "tokio-rt", feature = "http3-tokio"))]
use deboa_tests::server::udp::tokio::HttpServer;

use deboa_tests::{server::ServerConfig, utils::make_response};
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// PATCH
//

async fn do_patch() -> Result<()> {
    #[cfg(all(
        any(feature = "tokio-rt", feature = "smol-rt"),
        any(feature = "http1", feature = "http2")
    ))]
    let config: Option<ServerConfig> = None;
    #[cfg(all(feature = "tokio-rt", feature = "http3-tokio"))]
    let config: Option<ServerConfig> = Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ));
    let mut server = HttpServer::new(config);
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "PATCH" && req.uri().path() == "/posts/1" {
                Ok(make_response(StatusCode::OK, b""))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let client: Client = Client::builder()
        .certificate(Certificate::new("certs/ca.cert".into()))
        .build();

    let request = DeboaRequest::patch(server.url("/posts/1"))?
        .text("")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_patch() -> Result<()> {
    do_patch().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_patch() -> Result<()> {
    do_patch().await
}
