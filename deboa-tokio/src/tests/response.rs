use caramelo::{expect, matchers::eq};
use deboa::{response::DeboaResponse, Result};
use std::fs::remove_file;

const SAMPLE_TEST: &[u8] = b"Hello, world!";

#[tokio::test]
async fn test_raw_body() -> Result<()> {
    let response = DeboaResponse::builder()
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    expect(
        response
            .bytes()
            .await,
    )
    .to_be(eq(SAMPLE_TEST.to_vec()));
    Ok(())
}

#[tokio::test]
async fn test_text_body() -> Result<()> {
    let response = DeboaResponse::builder()
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    expect(
        response
            .text()
            .await,
    )
    .to_be(eq(Ok(String::from_utf8_lossy(SAMPLE_TEST).to_string())));
    Ok(())
}

#[tokio::test]
async fn test_to_file() -> Result<()> {
    let output_file = "test.txt";
    let response = DeboaResponse::builder()
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    expect(
        response
            .to_file(output_file)
            .await,
    )
    .to_be(eq(Ok(())));
    remove_file(output_file).unwrap();
    Ok(())
}
