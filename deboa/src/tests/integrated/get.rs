use crate::errors::{ConnectionError, ResponseError};
#[cfg(test)]
use crate::{
    errors::DeboaError, request::DeboaRequest, response::DeboaResponse, Client, HttpVersion, Result,
};

use deboa_tests::utils::JSONPLACEHOLDER;

use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// GET
//

async fn do_get_http1() -> Result<()> {
    let client = Client::default();

    let request = DeboaRequest::get(format!("{}/posts/1", JSONPLACEHOLDER))?.build()?;

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

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_http1() -> Result<()> {
    do_get_http1().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_http1() {
    let _ = do_get_http1().await;
}

#[cfg(feature = "http2")]
async fn do_get_http2() -> Result<()> {
    let client = Client::builder()
        .protocol(HttpVersion::Http2)
        .build();

    let request = DeboaRequest::get(format!("{}/posts/1", JSONPLACEHOLDER))?.build()?;

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

    Ok(())
}

#[cfg(all(feature = "http2", feature = "tokio-rt"))]
#[tokio::test]
async fn test_get_http2() -> Result<()> {
    do_get_http2().await?;
    Ok(())
}

#[cfg(all(feature = "http2", feature = "smol-rt"))]
#[apply(test!)]
async fn test_get_http2() {
    let _ = do_get_http2().await;
}

#[cfg(feature = "http3-tokio")]
#[tokio::test]
async fn get_http3() -> Result<()> {
    let client = Client::builder()
        .protocol(HttpVersion::Http3)
        .build();

    let request = DeboaRequest::get(format!("{}/posts/1", JSONPLACEHOLDER))?.build()?;

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

    Ok(())
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
    let client = Client::default();

    let response: Result<DeboaResponse> =
        DeboaRequest::get(format!("{}/asasa/posts/1ddd", JSONPLACEHOLDER))?
            .send_with(client)
            .await;

    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err(),
        DeboaError::Response(ResponseError::Receive {
            status_code: StatusCode::NOT_FOUND,
            message: "Could not process request (404 Not Found): {}".to_string()
        })
    );

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
        message: "No such host is known. (os error 11001)".to_string(),
        #[cfg(target_os = "linux")]
        message: "failed to lookup address information: Name or service not known".to_string(),
        #[cfg(target_os = "macos")]
        message:
            "failed to lookup address information: nodename nor servname provided, or not known"
                .to_string(),
    });

    #[cfg(feature = "http3-tokio")]
    let error = DeboaError::Connection(ConnectionError::Udp {
        host: "invalid-server.com".to_string(),
        #[cfg(target_os = "windows")]
        message: "No such host is known. (os error 11001)".to_string(),
        #[cfg(target_os = "linux")]
        message: "failed to lookup address information: Name or service not known".to_string(),
        #[cfg(target_os = "macos")]
        message:
            "failed to lookup address information: nodename nor servname provided, or not known"
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
    let client = Client::default();

    let response = DeboaRequest::get(format!("{}/comments/1", JSONPLACEHOLDER))?
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
    let client = Client::default();

    let response = DeboaRequest::get(format!("{}/comments/1", JSONPLACEHOLDER))?
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
