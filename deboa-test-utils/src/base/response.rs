use deboa::{response::DeboaResponse, TestResult};
use std::fs::remove_file;

const SAMPLE_TEST: &[u8] = b"Hello, world!";

pub async fn test_raw_body() -> TestResult<()> {
    let response = DeboaResponse::builder()
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    assert_eq!(
        response
            .bytes()
            .await?,
        SAMPLE_TEST
    );
    Ok(())
}

pub async fn test_text_body() -> TestResult<()> {
    let response = DeboaResponse::builder()
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    assert_eq!(
        response
            .text()
            .await?,
        String::from_utf8_lossy(SAMPLE_TEST).to_string()
    );
    Ok(())
}

pub async fn test_to_file() -> TestResult<()> {
    let output_file = "test.txt";
    let response = DeboaResponse::builder()
        .status(http::StatusCode::OK)
        .headers(http::HeaderMap::new())
        .body(SAMPLE_TEST)
        .build();
    assert_eq!(
        response
            .to_file(output_file)
            .await?,
        ()
    );
    remove_file(output_file).unwrap();
    Ok(())
}
