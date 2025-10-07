use crate::{cookie::DeboaCookie, response::DeboaResponse, Result};
use deboa_tests::utils::fake_url;
use http::header;

const SAMPLE_TEST: &[u8] = b"Hello, world!";

#[test]
fn test_status() -> Result<()> {
    let response = DeboaResponse::new(
        fake_url(),
        http::StatusCode::OK,
        http::HeaderMap::new(),
        &Vec::new(),
    );
    assert_eq!(response.status(), http::StatusCode::OK);
    Ok(())
}

#[test]
fn test_headers() -> Result<()> {
    let response = DeboaResponse::new(
        fake_url(),
        http::StatusCode::OK,
        http::HeaderMap::new(),
        &Vec::new(),
    );
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
    let response = DeboaResponse::new(fake_url(), http::StatusCode::OK, headers, &Vec::new());
    assert_eq!(
        response.cookies(),
        Ok(Some(vec![DeboaCookie::new("test", "test")]))
    );
    Ok(())
}

#[test]
fn test_set_raw_body() -> Result<()> {
    let mut response = DeboaResponse::new(
        fake_url(),
        http::StatusCode::OK,
        http::HeaderMap::new(),
        &Vec::new(),
    );
    response.set_raw_body(SAMPLE_TEST);
    assert_eq!(response.raw_body(), SAMPLE_TEST);
    Ok(())
}

#[test]
fn test_raw_body() -> Result<()> {
    let response = DeboaResponse::new(
        fake_url(),
        http::StatusCode::OK,
        http::HeaderMap::new(),
        SAMPLE_TEST,
    );
    assert_eq!(response.raw_body(), SAMPLE_TEST);
    Ok(())
}

#[test]
fn test_text() -> Result<()> {
    let response = DeboaResponse::new(
        fake_url(),
        http::StatusCode::OK,
        http::HeaderMap::new(),
        SAMPLE_TEST,
    );
    assert_eq!(
        response.text(),
        Ok(String::from_utf8_lossy(SAMPLE_TEST).to_string())
    );
    Ok(())
}

#[test]
fn test_to_file() -> Result<()> {
    let response = DeboaResponse::new(
        fake_url(),
        http::StatusCode::OK,
        http::HeaderMap::new(),
        SAMPLE_TEST,
    );
    assert_eq!(response.to_file("test.txt"), Ok(()));
    Ok(())
}
