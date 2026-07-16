use crate::tests::{helpers::fake_url, TestResult};
use deboa::request::DeboaRequest;
use http_body_util::BodyExt;
use macro_rules_attribute::apply;
use smol_macros::test;
use std::str::FromStr;

#[apply(test!)]
async fn test_from_str_body() -> TestResult<()> {
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

#[apply(test!)]
async fn test_set_text_body() -> TestResult<()> {
    let request = DeboaRequest::post(fake_url())?
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

#[apply(test!)]
async fn test_raw_body() -> TestResult<()> {
    let request = DeboaRequest::post(fake_url())?
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
