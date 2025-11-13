use crate::errors::{ConnectionError, ResponseError};
#[cfg(test)]
use crate::{
    errors::DeboaError, request::DeboaRequest, response::DeboaResponse, Deboa, HttpVersion, Result,
};

use deboa_tests::utils::setup_server;

use http::{header, StatusCode};
use httpmock::MockServer;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// GET
//

async fn do_get_http1() -> Result<()> {
    let server = MockServer::start();

    let http_mock = setup_server(&server, "/posts", httpmock::Method::GET, StatusCode::OK);

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
    let server = MockServer::start();

    let http_mock = setup_server(&server, "/posts", httpmock::Method::GET, StatusCode::OK);

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
async fn test_get_http2() -> Result<()> {
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

async fn do_get_not_found() -> Result<()> {
    let server = MockServer::start();

    let http_mock = setup_server(
        &server,
        "/asasa/posts/1ddd",
        httpmock::Method::GET,
        StatusCode::NOT_FOUND,
    );

    let client = Deboa::new();

    let response: Result<DeboaResponse> =
        DeboaRequest::get(server.url("/asasa/posts/1ddd").as_str())?
            .with(client)
            .await;

    http_mock.assert();

    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err(),
        DeboaError::Response(ResponseError::Receive {
            status_code: StatusCode::NOT_FOUND,
            message: "Could not process request (404 Not Found): ping".to_string()
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
    let mut api = Deboa::new();

    let request = DeboaRequest::get("https://invalid-server.com/posts")?
        .text("test")
        .build()?;

    let response: Result<DeboaResponse> = api.execute(request).await;

    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err(),
        DeboaError::Connection(ConnectionError::Tcp {
            host: "invalid-server.com".to_string(),
            #[cfg(target_os = "linux")]
            message: "failed to lookup address information: Name or service not known".to_string(),
            #[cfg(target_os = "macos")]
            message:
                "failed to lookup address information: nodename nor servname provided, or not known"
                    .to_string(),
        })
    );

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
    let server = MockServer::start();

    let http_mock = setup_server(
        &server,
        "/comments/1",
        httpmock::Method::GET,
        StatusCode::OK,
    );

    let client = Deboa::new();

    let response = DeboaRequest::get(server.url("/comments/1").as_str())?
        .with(client)
        .await?;

    http_mock.assert();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response.status().as_u16(),
        StatusCode::OK.as_u16()
    );

    let comments = response.text().await;

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
    let server = MockServer::start();

    let http_mock = setup_server(
        &server,
        "/comments/1",
        httpmock::Method::GET,
        StatusCode::BAD_GATEWAY,
    );

    let client = Deboa::new();

    let response = DeboaRequest::get(server.url("/comments/1").as_str())?
        .retries(2)
        .with(client)
        .await;

    http_mock.assert_calls(3);

    if let Err(err) = response {
        assert_eq!(
            err,
            DeboaError::Response(ResponseError::Receive {
                status_code: StatusCode::BAD_GATEWAY,
                message: "Could not process request (502 Bad Gateway): ping".to_string(),
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
    let server = MockServer::start();

    let http_mock_red = server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/comments/one");
        then.status::<u16>(StatusCode::MOVED_PERMANENTLY.into())
            .header(header::LOCATION.as_str(), server.url("/comments/1"));
    });

    let http_mock_tgt = server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/comments/1");
        then.status::<u16>(StatusCode::OK.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let client = Deboa::new();

    let response = DeboaRequest::get(server.url("/comments/one").as_str())?
        .with(client)
        .await?;

    http_mock_red.assert();
    http_mock_tgt.assert();

    assert_eq!(response.status(), StatusCode::OK);

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
