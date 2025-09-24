use crate::Deboa;
use crate::HttpVersion;
#[cfg(test)]
use crate::errors::DeboaError;
use crate::request::DeboaRequest;
use crate::response::DeboaResponse;

use deboa_tests::utils::setup_server;

use http::StatusCode;
use httpmock::MockServer;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// GET
//

async fn do_get_http1() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = setup_server(&server, "/posts", StatusCode::OK);

    let mut client = Deboa::builder().protocol(HttpVersion::Http1).build();

    let request = DeboaRequest::get(server.url("/posts").as_str())?.build()?;

    let response: DeboaResponse = client.execute(request).await?;

    http_mock.assert();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response.status().as_u16(),
        StatusCode::OK.as_u16()
    );

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_http1() -> Result<(), DeboaError> {
    do_get_http1().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_http1() {
    let _ = do_get_http1().await;
}

#[cfg(feature = "http2")]
async fn do_get_http2() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = setup_server(&server, "/posts", StatusCode::OK);

    let mut client = Deboa::builder().protocol(HttpVersion::Http2).build();

    let request = DeboaRequest::get(server.url("/posts").as_str())?.build()?;

    let response: DeboaResponse = client.execute(request).await?;

    http_mock.assert();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response.status().as_u16(),
        StatusCode::OK.as_u16()
    );

    Ok(())
}

#[cfg(all(feature = "http2", feature = "tokio-rt"))]
#[tokio::test]
async fn test_get_http2() -> Result<(), DeboaError> {
    do_get_http2().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_http2() {
    let _ = do_get_http2().await;
}

//
// GET NOT FOUND
//

async fn do_get_not_found() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = setup_server(&server, "/asasa/posts/1ddd", StatusCode::NOT_FOUND);

    let client = Deboa::new();

    let response: Result<DeboaResponse, DeboaError> = DeboaRequest::get(server.url("/asasa/posts/1ddd").as_str())?.go(client).await;

    http_mock.assert();

    assert!(response.is_err());
    assert_eq!(
        response,
        Err(DeboaError::Response {
            status_code: StatusCode::NOT_FOUND,
            message: "404 Not Found".to_string()
        })
    );

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_not_found() -> Result<(), DeboaError> {
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

async fn do_get_invalid_server() -> Result<(), DeboaError> {
    let mut api = Deboa::new();

    let request = DeboaRequest::get("https://invalid-server.com/posts")?.text("test").build()?;

    let response: Result<DeboaResponse, DeboaError> = api.execute(request).await;

    assert!(response.is_err());
    assert_eq!(
        response,
        Err(DeboaError::Connection {
            host: "invalid-server.com".to_string(),
            #[cfg(target_os = "linux")]
            message: "failed to lookup address information: Name or service not known".to_string(),
            #[cfg(target_os = "macos")]
            message: "failed to lookup address information: nodename nor servname provided, or not known".to_string(),
        })
    );

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_invalid_server() -> Result<(), DeboaError> {
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

async fn do_get_by_query() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = setup_server(&server, "/comments/1", StatusCode::OK);

    let client = Deboa::new();

    let response = DeboaRequest::get(server.url("/comments/1").as_str())?.go(client).await?;

    http_mock.assert();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response.status().as_u16(),
        StatusCode::OK.as_u16()
    );

    let comments = response.text();

    assert!(comments.is_ok());

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_by_query() -> Result<(), DeboaError> {
    do_get_by_query().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_by_query() {
    let _ = do_get_by_query().await;
}

async fn do_get_by_query_with_retries() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = setup_server(&server, "/comments/1", StatusCode::BAD_GATEWAY);

    let client = Deboa::new();

    let response = DeboaRequest::get(server.url("/comments/1").as_str())?.retries(2).go(client).await;

    http_mock.assert_calls(3);

    if let Err(err) = response {
        assert_eq!(
            err,
            DeboaError::Response {
                status_code: StatusCode::BAD_GATEWAY,
                message: "502 Bad Gateway".to_string(),
            },
        );
    }

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_by_query_with_retries() -> Result<(), DeboaError> {
    do_get_by_query_with_retries().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_by_query_with_retries() {
    let _ = do_get_by_query_with_retries().await;
}
