use crate::{
    cookie::DeboaCookie,
    response::{DeboaResponse, IntoBody},
    tests::{test_url, TestResult},
};
use http::{header, Response};

const SAMPLE_TEST: &[u8] = b"Hello, world!";

#[test]
fn test_status() -> Result<(), Box<dyn std::error::Error>> {
    let response = Response::builder()
        .status(http::StatusCode::OK)
        .body(SAMPLE_TEST.into_body())
        .unwrap();

    let response = DeboaResponse::new(test_url().into(), response);
    assert_eq!(response.status(), http::StatusCode::OK);
    Ok(())
}

#[test]
fn test_headers() -> Result<(), Box<dyn std::error::Error>> {
    let response = DeboaResponse::builder(test_url())
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .build();
    assert_eq!(*response.headers(), http::HeaderMap::new());
    Ok(())
}

#[test]
fn test_cookies() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = http::HeaderMap::new();
    headers.insert(header::SET_COOKIE, http::HeaderValue::from_static("test=test"));
    let response = DeboaResponse::builder(test_url())
        .status(http::StatusCode::OK)
        .headers(headers)
        .build();
    assert_eq!(response.cookies(), Ok(Some(vec![DeboaCookie::new("test", "test")])));
    Ok(())
}

#[test]
fn test_header() -> Result<(), Box<dyn std::error::Error>> {
    let response = DeboaResponse::builder(test_url())
        .status(http::StatusCode::OK)
        .header(header::ACCEPT_LANGUAGE, "pt-BR")
        .build();
    assert_eq!(
        response
            .headers()
            .get(header::ACCEPT_LANGUAGE),
        Some(&http::HeaderValue::from_static("pt-BR"))
    );
    Ok(())
}

#[test]
fn test_response_url() -> Result<(), Box<dyn std::error::Error>> {
    let response = DeboaResponse::builder(test_url())
        .status(http::StatusCode::OK)
        .header(header::ACCEPT_LANGUAGE, "pt-BR")
        .build();
    assert_eq!(
        response
            .url()
            .as_str(),
        test_url().as_str()
    );
    Ok(())
}

#[test]
fn test_content_type() -> Result<(), Box<dyn std::error::Error>> {
    let response = DeboaResponse::builder(test_url())
        .status(http::StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .build();
    assert_eq!(response.content_type(), Ok("text/html".to_string()));
    Ok(())
}

#[test]
fn test_content_length() -> Result<(), Box<dyn std::error::Error>> {
    let response = DeboaResponse::builder(test_url())
        .status(http::StatusCode::OK)
        .header(header::CONTENT_LENGTH, "9")
        .build();
    assert_eq!(response.content_length(), Ok(9));
    Ok(())
}
