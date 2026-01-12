use deboa_tests::utils::{make_response, tls_server_config, CA_CERT};

#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::smol::HttpServer;

#[cfg(all(feature = "tokio-rt", feature = "http3"))]
use deboa_tests::server::udp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", feature = "http3"))]
use deboa_tests::server::udp::smol::HttpServer;

use http::StatusCode;

use crate::{cert::Certificate, default_protocol, tests::SKIP_CERT_VERIFICATION, Client, Result};

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

#[test]
fn test_set_connection_timeout() -> Result<()> {
    let api = Client::builder()
        .connection_timeout(5)
        .build();

    assert_eq!(api.connection_timeout, 5);

    Ok(())
}

#[test]
fn test_set_request_timeout() -> Result<()> {
    let api = Client::builder()
        .request_timeout(5)
        .build();

    assert_eq!(api.request_timeout, 5);

    Ok(())
}

#[test]
fn test_set_protocol() -> Result<()> {
    let api = Client::builder()
        .protocol(default_protocol())
        .build();

    assert_eq!(api.protocol, default_protocol());

    Ok(())
}

#[test]
fn test_set_skip_cert_verification() -> Result<()> {
    let api = Client::builder()
        .skip_cert_verification(true)
        .build();

    assert!(api.skip_cert_verification);

    Ok(())
}

async fn shl() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "GET" && req.uri().path() == "/" {
                Ok(make_response(StatusCode::OK, b""))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();
    let request = &client << &server.url("/");
    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), 200);

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_shl() -> Result<()> {
    shl().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_shl() -> Result<()> {
    shl().await
}
