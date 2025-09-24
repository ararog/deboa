use crate::{errors::DeboaError, response::DeboaResponse};
use deboa_tests::utils::fake_url;

const SAMPLE_TEST: &[u8] = b"Hello, world!";

#[test]
fn test_status() -> Result<(), DeboaError> {
    let response = DeboaResponse::new(fake_url(), http::StatusCode::OK, http::HeaderMap::new(), &Vec::new());
    assert_eq!(response.status(), http::StatusCode::OK);
    Ok(())
}

#[test]
fn test_headers() -> Result<(), DeboaError> {
    let response = DeboaResponse::new(fake_url(), http::StatusCode::OK, http::HeaderMap::new(), &Vec::new());
    assert_eq!(*response.headers(), http::HeaderMap::new());
    Ok(())
}

#[test]
fn test_set_raw_body() -> Result<(), DeboaError> {
    let mut response = DeboaResponse::new(fake_url(), http::StatusCode::OK, http::HeaderMap::new(), &Vec::new());
    response.set_raw_body(SAMPLE_TEST);
    assert_eq!(response.raw_body(), SAMPLE_TEST);
    Ok(())
}

#[test]
fn test_raw_body() -> Result<(), DeboaError> {
    let response = DeboaResponse::new(fake_url(), http::StatusCode::OK, http::HeaderMap::new(), SAMPLE_TEST);
    assert_eq!(response.raw_body(), SAMPLE_TEST);
    Ok(())
}

#[test]
fn test_text() -> Result<(), DeboaError> {
    let response = DeboaResponse::new(fake_url(), http::StatusCode::OK, http::HeaderMap::new(), SAMPLE_TEST);
    assert_eq!(response.text(), Ok(String::from_utf8_lossy(SAMPLE_TEST).to_string()));
    Ok(())
}

#[test]
fn test_to_file() -> Result<(), DeboaError> {
    let response = DeboaResponse::new(fake_url(), http::StatusCode::OK, http::HeaderMap::new(), SAMPLE_TEST);
    assert_eq!(response.to_file("test.txt"), Ok(()));
    Ok(())
}
