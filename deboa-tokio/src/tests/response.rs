use crate::tests::helpers::fake_url;
use deboa::{response::DeboaResponse, Result};
use std::fs::remove_file;

const SAMPLE_TEST: &[u8] = b"Hello, world!";

#[tokio::test]
async fn test_raw_body() -> Result<()> {
    let response = DeboaResponse::builder(fake_url())
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

#[tokio::test]
async fn test_text_body() -> Result<()> {
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

#[tokio::test]
async fn test_to_file() -> Result<()> {
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
