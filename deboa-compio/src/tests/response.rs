use deboa::{response::DeboaResponse, url::IntoUrl, Result};
use std::fs::remove_file;

const SAMPLE_TEST: &[u8] = b"Hello, world!";

async fn raw_body() -> Result<()> {
    let response = DeboaResponse::builder("https://example.com".into_url()?)
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    assert_eq!(
        response
            .bytes()
            .await,
        SAMPLE_TEST
    );
    Ok(())
}

#[compio::test]
async fn test_raw_body() -> Result<()> {
    raw_body().await
}

async fn text_body() -> Result<()> {
    let response = DeboaResponse::builder("https://example.com".into_url()?)
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

#[compio::test]
async fn test_text_body() -> Result<()> {
    text_body().await
}

async fn to_file() -> Result<()> {
    let output_file = "test.txt";
    let response = DeboaResponse::builder("https://example.com".into_url()?)
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

#[compio::test]
async fn test_to_file() -> Result<()> {
    to_file().await
}
