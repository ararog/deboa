use crate::{
    cert::Certificate,
    errors::{ConnectionError, ResponseError},
    tests::SKIP_CERT_VERIFICATION,
};
#[cfg(test)]
use crate::{errors::DeboaError, request::DeboaRequest, response::DeboaResponse, Client, Result};

use deboa_tests::utils::{make_response, tls_server_config, CA_CERT};

#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::smol::HttpServer;

#[cfg(all(feature = "tokio-rt", feature = "http3-tokio"))]
use deboa_tests::server::udp::tokio::HttpServer;

use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// GET
//

async fn do_get_http() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    let result = server
        .start(|req| {
            if req.method() == "GET" && req.uri().path() == "/posts/1" {
                Ok(make_response(StatusCode::OK, b"Hello World!"))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    if let Err(err) = result {
        return Err(DeboaError::Connection(ConnectionError::Tcp {
            host: "localhost".to_string(),
            message: err.to_string(),
        }));
    }

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let request = DeboaRequest::get(server.url("/posts/1"))?.build()?;

    let response: DeboaResponse = client
        .execute(request)
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response
            .status()
            .as_u16(),
        StatusCode::OK.as_u16()
    );

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_http() -> Result<()> {
    do_get_http().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_http() {
    let _ = do_get_http().await;
}

//
// Test TLS cert verification
//

/*
async fn test_tls_verification_helper(skip: bool) -> Result<()> {
    // This function would contain the common logic for both skip and non-skip verification tests
    let ca_path = "src/tests/integrated/certs/cert.pem";
    let key_path = "src/tests/integrated/certs/key.pem";

    let server = HttpMockServerBuilder::new()
      .port(8698)
      .https_ca_key_pair_option(Some(ca_path), Some(key_path))
      .build().unwrap();

    let server = MockServer::connect_async("localhost:8698").await;

    let http_mock = setup_server(&server, "/comments/1", httpmock::Method::GET, StatusCode::OK);

    let client = Client::builder()
        .skip_cert_verification(skip)
        .build();

    let response = DeboaRequest::get(
        server
            .url("/comments/1")
            .as_str(),
    )?
    .send_with(client)
    .await?;

    http_mock.assert();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response
            .status()
            .as_u16(),
        StatusCode::OK.as_u16()
    );

    let comments = response
        .text()
        .await;

    assert!(comments.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_tls_skip_cert_verification() -> Result<()> {
    test_tls_verification_helper(true).await
}

#[tokio::test]
async fn test_tls_cert_verification() -> Result<()> {
    test_tls_verification_helper(false).await
}
*/

//
// GET NOT FOUND
//

async fn do_get_not_found() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|_| Ok(make_response(StatusCode::NOT_FOUND, b"Not found")))
        .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let response: Result<DeboaResponse> = DeboaRequest::get(server.url("/asasa/posts/1ddd"))?
        .send_with(client)
        .await;

    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err(),
        DeboaError::Response(ResponseError::Receive {
            status_code: StatusCode::NOT_FOUND,
            message: "Could not process request (404 Not Found): Not found".to_string()
        })
    );

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_not_found() -> Result<()> {
    do_get_not_found().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_not_found() {
    let _ = do_get_not_found().await;
}

//
// GET INVALID SERVER
//

async fn do_get_invalid_server() -> Result<()> {
    let api = Client::default();

    let request = DeboaRequest::get("https://invalid-server.com/posts")?
        .text("test")
        .build()?;

    let response: Result<DeboaResponse> = api
        .execute(request)
        .await;

    #[cfg(any(feature = "http1", feature = "http2"))]
    let error = DeboaError::Connection(ConnectionError::Tcp {
        host: "invalid-server.com".to_string(),
        #[cfg(target_os = "windows")]
        message: "Could not connect to server: No such host is known. (os error 11001)".to_string(),
        #[cfg(target_os = "linux")]
        message: "Could not connect to server: failed to lookup address information: Name or service not known".to_string(),
        #[cfg(target_os = "macos")]
        message:
            "Could not connect to server: failed to lookup address information: nodename nor servname provided, or not known"
                .to_string(),
    });

    #[cfg(feature = "http3-tokio")]
    let error = DeboaError::Connection(ConnectionError::Udp {
        host: "invalid-server.com".to_string(),
        #[cfg(target_os = "windows")]
        message: "Could not connect to server: No such host is known. (os error 11001)".to_string(),
        #[cfg(target_os = "linux")]
        message: "Could not connect to server: failed to lookup address information: Name or service not known".to_string(),
        #[cfg(target_os = "macos")]
        message: "Could not connect to server: nodename nor servname provided, or not known"
            .to_string(),
    });

    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), error);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_invalid_server() -> Result<()> {
    do_get_invalid_server().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_invalid_server() {
    let _ = do_get_invalid_server().await;
}

//
// GET BY QUERY
//

async fn do_get_by_query() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "GET" && req.uri().path() == "/comments/1" {
                Ok(make_response(StatusCode::OK, b"My comment"))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let response = DeboaRequest::get(server.url("/comments/1"))?
        .send_with(client)
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response
            .status()
            .as_u16(),
        StatusCode::OK.as_u16()
    );

    let comments = response
        .text()
        .await;

    assert!(comments.is_ok());
    assert_eq!(comments.unwrap(), "My comment");

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_by_query() -> Result<()> {
    do_get_by_query().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_by_query() {
    let _ = do_get_by_query().await;
}

async fn do_get_by_query_with_retries() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|_req| Ok(make_response(StatusCode::BAD_GATEWAY, b"pong")))
        .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let response = DeboaRequest::get(server.url("/comments/1"))?
        .retries(2)
        .send_with(client)
        .await;

    if let Err(err) = response {
        assert_eq!(
            err,
            DeboaError::Response(ResponseError::Receive {
                status_code: StatusCode::BAD_GATEWAY,
                message: "Could not process request (502 Bad Gateway): pong".to_string(),
            }),
        );
    }

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_by_query_with_retries() -> Result<()> {
    do_get_by_query_with_retries().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_by_query_with_retries() {
    let _ = do_get_by_query_with_retries().await;
}

/*
async fn do_get_with_redirect() -> Result<()> {
    let client = Client::default();

    let url = if cfg!(feature = "http3-tokio") {
        "https://tinyurl.com/bccjpjd7"
    } else {
        "https://tinyurl.com/bp6e548b"
    };

    let response = DeboaRequest::get(url)?
        .send_with(client)
        .await?;

    let server = if cfg!(feature = "http3-tokio") { "facebook.com" } else { "github.com" };

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("server")
            .unwrap()
            .to_str()
            .unwrap(),
        server
    );

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_with_redirect() -> Result<()> {
    do_get_with_redirect().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_with_redirect() {
    let _ = do_get_with_redirect().await;
}
*/
