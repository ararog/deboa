use crate::tests::helpers::fake_url;
use caramelo::{expect, matchers::eq};
use deboa::{request::DeboaRequest, TestResult};
use http_body_util::BodyExt;
use std::str::FromStr;

#[tokio::test]
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

    expect(bytes).to_be(eq(&b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}"[..]));
    Ok(())
}

#[tokio::test]
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

    expect(bytes).to_be(eq(&b"test"[..]));

    Ok(())
}

#[tokio::test]
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

    expect(bytes).to_be(eq(&b"test"[..]));

    Ok(())
}
