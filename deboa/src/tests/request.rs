use std::{str::FromStr, sync::Arc};

use crate::{
    cert::Certificate,
    request::{DeboaRequest, FetchWith, IntoRequest, MethodExt},
    Client, Result,
};

use deboa_tests::{
    server::ServerConfig,
    utils::{make_response, TEST_HOST},
};

#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::smol::HttpServer;

#[cfg(all(feature = "tokio-rt", feature = "http3-tokio"))]
use deboa_tests::server::udp::tokio::HttpServer;

use http::{header, HeaderValue, Method, StatusCode};
use url::Url;

#[test]
fn test_method_ext_from_url() -> Result<()> {
    let request = Method::GET
        .from_url(TEST_HOST)?
        .build()?;
    assert_eq!(request.method(), &Method::GET);
    assert_eq!(
        request
            .url()
            .to_string(),
        TEST_HOST
    );
    Ok(())
}

#[test]
fn test_method_ext_to_url() -> Result<()> {
    let request = Method::POST
        .to_url(TEST_HOST)?
        .build()?;
    assert_eq!(request.method(), &Method::POST);
    assert_eq!(
        request
            .url()
            .to_string(),
        TEST_HOST
    );
    Ok(())
}

#[test]
fn test_str_method_ext_from_url() -> Result<()> {
    let request = "GET"
        .from_url(TEST_HOST)?
        .build()?;
    assert_eq!(request.method(), &Method::GET);
    assert_eq!(
        request
            .url()
            .to_string(),
        TEST_HOST
    );
    Ok(())
}

#[test]
fn test_str_method_ext_to_url() -> Result<()> {
    let request = "POST"
        .to_url(TEST_HOST)?
        .build()?;
    assert_eq!(request.method(), &Method::POST);
    assert_eq!(
        request
            .url()
            .to_string(),
        TEST_HOST
    );
    Ok(())
}

#[test]
fn test_into_url() -> Result<()> {
    let url = Url::parse(TEST_HOST).unwrap();
    let request = DeboaRequest::get(url)?.build()?;
    assert_eq!(
        request
            .url()
            .to_string(),
        TEST_HOST
    );
    Ok(())
}

#[test]
fn test_into_request_from_str() -> Result<()> {
    let request = TEST_HOST.into_request()?;
    assert_eq!(
        request
            .url()
            .to_string(),
        TEST_HOST
    );
    Ok(())
}

#[test]
fn test_into_request_from_string() -> Result<()> {
    let request = format!("{}/posts/{}", TEST_HOST, 1).into_request()?;
    assert_eq!(
        request
            .url()
            .to_string(),
        format!("{}/posts/{}", TEST_HOST, 1)
    );
    Ok(())
}

#[test]
fn test_into_str() -> Result<()> {
    let request = DeboaRequest::get(TEST_HOST)?.build()?;
    assert_eq!(
        request
            .url()
            .to_string(),
        TEST_HOST
    );
    Ok(())
}

#[test]
fn test_into_string() -> Result<()> {
    let request = DeboaRequest::get(String::from(TEST_HOST))?.build()?;
    assert_eq!(
        request
            .url()
            .to_string(),
        TEST_HOST
    );
    Ok(())
}

#[tokio::test]
async fn test_try_into() -> Result<()> {
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
            if req.method() == "GET" && req.uri().path() == "/posts/1" {
                Ok(make_response(StatusCode::OK, b""))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let client = Client::builder()
        .certificate(Certificate::new("certs/ca.cert".into()))
        .build();
    let first_post = server.url("/posts/1");
    let response = client
        .execute(first_post.into_request()?)
        .await?;
    assert_eq!(response.status(), 200);

    server.stop().await;

    Ok(())
}

#[test]
fn test_from_str_method_and_url() -> Result<()> {
    let request = DeboaRequest::from_str(
        r##"
    GET https://localhost:8000
    "##,
    )?;
    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.url(), Arc::new(Url::parse("https://localhost:8000").unwrap()));
    Ok(())
}

#[test]
fn test_from_str_headers() -> Result<()> {
    let request = DeboaRequest::from_str(
        r##"
    GET https://localhost:8000
    Content-Type: application/json
    "##,
    )?;
    assert_eq!(
        request
            .headers()
            .get(header::CONTENT_TYPE),
        Some(&HeaderValue::from_str("application/json").unwrap())
    );
    Ok(())
}

#[test]
fn test_from_str_body() -> Result<()> {
    let request = DeboaRequest::from_str(
        r##"
    GET https://localhost:8000
    Content-Type: application/json
    
    {"title": "foo", "body": "bar", "userId": 1}
    "##,
    )?;
    assert_eq!(request.raw_body(), b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}");
    Ok(())
}

#[test]
fn test_set_retries() -> Result<()> {
    let api = DeboaRequest::get(TEST_HOST)?
        .retries(5)
        .build()?;
    assert_eq!(api.retries(), 5);
    Ok(())
}

#[test]
fn test_base_url() -> Result<()> {
    let api = DeboaRequest::get(String::from(TEST_HOST))?.build()?;
    assert_eq!(
        api.url()
            .to_string(),
        TEST_HOST
    );
    Ok(())
}

#[test]
fn test_set_headers() -> Result<()> {
    let request = DeboaRequest::get(TEST_HOST)?
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(&header::CONTENT_TYPE),
        Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())
    );

    Ok(())
}

#[test]
fn test_set_headers_as_tuple() -> Result<()> {
    let headers = vec![(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string())];
    let request = DeboaRequest::get(TEST_HOST)?
        .headers(headers)
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(&header::CONTENT_TYPE),
        Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())
    );

    Ok(())
}

#[test]
fn test_set_basic_auth() -> Result<()> {
    let request = DeboaRequest::get(TEST_HOST)?
        .basic_auth("username", "password")
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(&header::AUTHORIZATION),
        Some(&HeaderValue::from_str("Basic dXNlcm5hbWU6cGFzc3dvcmQ=").unwrap())
    );

    Ok(())
}

#[test]
fn test_set_bearer_auth() -> Result<()> {
    let request = DeboaRequest::get(TEST_HOST)?
        .bearer_auth("token")
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(&header::AUTHORIZATION),
        Some(&HeaderValue::from_str("Bearer token").unwrap())
    );

    Ok(())
}

#[test]
fn test_add_header() -> Result<()> {
    let request = DeboaRequest::get(TEST_HOST)?
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(&header::CONTENT_TYPE),
        Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())
    );

    Ok(())
}

#[test]
fn test_set_text_body() -> Result<()> {
    let request = DeboaRequest::post(TEST_HOST)?
        .text("test")
        .build()?;

    assert_eq!(*request.raw_body(), b"test"[..]);

    Ok(())
}

#[test]
fn test_raw_body() -> Result<()> {
    let request = DeboaRequest::post(TEST_HOST)?
        .raw_body(b"test")
        .build()?;

    assert_eq!(request.raw_body(), b"test");

    Ok(())
}

#[tokio::test]
async fn test_fetch_from_str() -> Result<()> {
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
            if req.method() == "GET" && req.uri().path() == "/posts/1" {
                Ok(make_response(StatusCode::OK, b""))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let client = Client::builder()
        .certificate(Certificate::new("certs/ca.cert".into()))
        .build();
    let first_post = server.url("/posts/1");
    let response = first_post
        .fetch_with(&client)
        .await?;
    assert_eq!(response.status(), 200);

    server.stop().await;

    Ok(())
}
