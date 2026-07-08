use deboa::{request::DeboaRequest, url::IntoUrl};
use http_body_util::BodyExt;
use std::{error::Error, str::FromStr};

async fn do_test_from_str_body() -> Result<(), Box<dyn Error>> {
    let request = DeboaRequest::from_str(
        r##"
    GET https://localhost:8000
    Content-Type: application/json

    {"title": "foo", "body": "bar", "userId": 1}
    "##,
    )?;

    let bytes = request
        .body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    assert_eq!(bytes, b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}"[..]);
    Ok(())
}

#[compio::test]
async fn test_from_str_body() {
    do_test_from_str_body()
        .await
        .unwrap();
}

async fn do_test_set_text_body() -> Result<(), Box<dyn Error>> {
    let test_url = "https://example.com".into_url()?;
    let request = DeboaRequest::post(test_url)?
        .text("test")
        .build()?;

    let bytes = request
        .body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    assert_eq!(bytes, b"test"[..]);

    Ok(())
}

#[compio::test]
async fn test_set_text_body() -> Result<(), Box<dyn Error>> {
    do_test_set_text_body().await
}

async fn do_test_raw_body() -> Result<(), Box<dyn Error>> {
    let test_url = "https://example.com".into_url()?;
    let request = DeboaRequest::post(test_url)?
        .text("test")
        .build()?;

    let bytes = request
        .body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    assert_eq!(bytes, b"test"[..]);

    Ok(())
}

#[compio::test]
async fn test_raw_body() -> Result<(), Box<dyn Error>> {
    do_test_raw_body().await
}
