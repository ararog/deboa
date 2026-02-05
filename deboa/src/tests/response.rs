use std::fs::remove_file;

use crate::{
    cookie::DeboaCookie,
    response::{DeboaResponse, IntoBody},
    Result,
};
use deboa_tests::utils::fake_url;
use http::{header, Response};

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

const SAMPLE_TEST: &[u8] = b"Hello, world!";

#[test]
fn test_status() -> Result<()> {
    let response = Response::builder()
        .status(http::StatusCode::OK)
        .body(SAMPLE_TEST.into_body())
        .unwrap();

    let response = DeboaResponse::new(fake_url().into(), response);
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
    headers.insert(header::SET_COOKIE, http::HeaderValue::from_static("test=test"));
    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .headers(headers)
        .build();
    assert_eq!(response.cookies(), Ok(Some(vec![DeboaCookie::new("test", "test")])));
    Ok(())
}

async fn raw_body() -> Result<()> {
    let mut response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    assert_eq!(
        response
            .raw_body()
            .await,
        SAMPLE_TEST
    );
    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_raw_body() -> Result<()> {
    raw_body().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_raw_body() -> Result<()> {
    raw_body().await
}

async fn text_body() -> Result<()> {
    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    assert_eq!(
        response
            .text()
            .await,
        Ok(String::from_utf8_lossy(SAMPLE_TEST).to_string())
    );
    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_text_body() -> Result<()> {
    text_body().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_text_body() -> Result<()> {
    text_body().await
}

async fn to_file() -> Result<()> {
    let output_file = "test.txt";
    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    assert_eq!(
        response
            .to_file(output_file)
            .await,
        Ok(())
    );
    remove_file(output_file).unwrap();
    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_to_file() -> Result<()> {
    to_file().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_to_file() -> Result<()> {
    to_file().await
}
