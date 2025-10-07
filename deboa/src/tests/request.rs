use crate::{request::DeboaRequest, request::Fetch, Deboa, Result};

use deboa_tests::utils::JSONPLACEHOLDER;
use http::{header, HeaderValue};
use url::Url;

#[test]
fn test_into_url() -> Result<()> {
    let url = Url::parse(JSONPLACEHOLDER).unwrap();
    let request = DeboaRequest::get(url)?.build()?;

    assert_eq!(request.url().to_string(), JSONPLACEHOLDER);

    Ok(())
}

#[test]
fn test_into_str() -> Result<()> {
    let request = DeboaRequest::get(JSONPLACEHOLDER)?.build()?;

    assert_eq!(request.url().to_string(), JSONPLACEHOLDER);

    Ok(())
}

#[test]
fn test_into_string() -> Result<()> {
    let request = DeboaRequest::get(String::from(JSONPLACEHOLDER))?.build()?;

    assert_eq!(request.url().to_string(), JSONPLACEHOLDER);

    Ok(())
}

#[tokio::test]
async fn test_try_into() -> Result<()> {
    let mut client = Deboa::new();

    let response = client.execute(JSONPLACEHOLDER).await?;

    assert_eq!(response.status(), 200);

    Ok(())
}

#[test]
fn test_set_retries() -> Result<()> {
    let api = DeboaRequest::get(JSONPLACEHOLDER)?.retries(5).build()?;

    assert_eq!(api.retries(), 5);

    Ok(())
}

#[test]
fn test_base_url() -> Result<()> {
    let api = DeboaRequest::get(String::from(JSONPLACEHOLDER))?.build()?;

    assert_eq!(api.url().to_string(), JSONPLACEHOLDER);

    Ok(())
}

#[test]
fn test_set_headers() -> Result<()> {
    let request = DeboaRequest::to(JSONPLACEHOLDER)?
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .build()?;

    assert_eq!(
        request.headers().get(&header::CONTENT_TYPE),
        Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())
    );

    Ok(())
}

#[test]
fn test_set_basic_auth() -> Result<()> {
    let request = DeboaRequest::get(JSONPLACEHOLDER)?
        .basic_auth("username", "password")
        .build()?;

    assert_eq!(
        request.headers().get(&header::AUTHORIZATION),
        Some(&HeaderValue::from_str("Basic dXNlcm5hbWU6cGFzc3dvcmQ=").unwrap())
    );

    Ok(())
}

#[test]
fn test_set_bearer_auth() -> Result<()> {
    let request = DeboaRequest::get(JSONPLACEHOLDER)?
        .bearer_auth("token")
        .build()?;

    assert_eq!(
        request.headers().get(&header::AUTHORIZATION),
        Some(&HeaderValue::from_str("Bearer token").unwrap())
    );

    Ok(())
}

#[test]
fn test_add_header() -> Result<()> {
    let request = DeboaRequest::get(JSONPLACEHOLDER)?
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .build()?;

    assert_eq!(
        request.headers().get(&header::CONTENT_TYPE),
        Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())
    );

    Ok(())
}

#[test]
fn test_set_text_body() -> Result<()> {
    let request = DeboaRequest::post(JSONPLACEHOLDER)?.text("test").build()?;

    assert_eq!(*request.raw_body(), b"test".to_vec());

    Ok(())
}

#[test]
fn test_raw_body() -> Result<()> {
    let request = DeboaRequest::post(JSONPLACEHOLDER)?
        .raw_body(b"test")
        .build()?;

    assert_eq!(request.raw_body(), b"test");

    Ok(())
}

#[tokio::test]
async fn test_fetch_from_string() -> Result<()> {
    let client = Deboa::new();

    let response = String::from(JSONPLACEHOLDER).fetch(client).await?;

    assert_eq!(response.status(), 200);

    Ok(())
}

#[tokio::test]
async fn test_fetch_from_url() -> Result<()> {
    let client = Deboa::new();
    let url = Url::parse(JSONPLACEHOLDER).unwrap();
    let response = url.fetch(client).await?;

    assert_eq!(response.status(), 200);

    Ok(())
}

#[tokio::test]
async fn test_fetch_from_str() -> Result<()> {
    let client = Deboa::new();
    let response = JSONPLACEHOLDER.fetch(client).await?;

    assert_eq!(response.status(), 200);

    Ok(())
}
