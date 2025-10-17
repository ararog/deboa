use crate::{
    cookie::DeboaCookie,
    response::{DeboaBody, DeboaResponse},
    Result,
};
use bytes::Bytes;
use deboa_tests::utils::fake_url;
use http::{header, Response};
use http_body_util::Full;

const SAMPLE_TEST: &[u8] = b"Hello, world!";

#[test]
fn test_status() -> Result<()> {
    let response = Response::builder()
        .status(http::StatusCode::OK)
        .body(DeboaBody::Right(Full::<Bytes>::from(SAMPLE_TEST.to_vec())))
        .unwrap();

    let response = DeboaResponse::new(fake_url(), response);
    assert_eq!(response.status(), http::StatusCode::OK);
    Ok(())
}

#[test]
fn test_headers() -> Result<()> {
    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .build();
    assert_eq!(*response.headers(), http::HeaderMap::new());
    Ok(())
}

#[test]
fn test_cookies() -> Result<()> {
    let mut headers = http::HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        http::HeaderValue::from_static("test=test"),
    );
    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .headers(headers)
        .build();
    assert_eq!(
        response.cookies(),
        Ok(Some(vec![DeboaCookie::new("test", "test")]))
    );
    Ok(())
}

#[tokio::test]
async fn raw_body() -> Result<()> {
    let mut response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    assert_eq!(response.raw_body().await, SAMPLE_TEST);
    Ok(())
}

#[tokio::test]
async fn test_text() -> Result<()> {
    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    assert_eq!(
        response.text().await,
        Ok(String::from_utf8_lossy(SAMPLE_TEST).to_string())
    );
    Ok(())
}

#[tokio::test]
async fn test_to_file() -> Result<()> {
    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    assert_eq!(response.to_file("test.txt").await, Ok(()));
    Ok(())
}
