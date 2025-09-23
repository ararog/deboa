use crate::request::DeboaRequest;
use crate::{errors::DeboaError, request::Fetch};

use crate::tests::utils::JSONPLACEHOLDER;
use http::{HeaderValue, header};
use url::Url;

#[test]
fn test_base_url() -> Result<(), DeboaError> {
    let api = DeboaRequest::get(String::from(JSONPLACEHOLDER))?.build()?;

    assert_eq!(api.url(), JSONPLACEHOLDER);

    Ok(())
}

#[test]
fn test_set_headers() -> Result<(), DeboaError> {
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
fn test_set_basic_auth() -> Result<(), DeboaError> {
    let request = DeboaRequest::get(JSONPLACEHOLDER)?.basic_auth("username", "password").build()?;

    assert_eq!(
        request.headers().get(&header::AUTHORIZATION),
        Some(&HeaderValue::from_str("Basic dXNlcm5hbWU6cGFzc3dvcmQ=").unwrap())
    );

    Ok(())
}

#[test]
fn test_set_bearer_auth() -> Result<(), DeboaError> {
    let request = DeboaRequest::get(JSONPLACEHOLDER)?.bearer_auth("token").build()?;

    assert_eq!(
        request.headers().get(&header::AUTHORIZATION),
        Some(&HeaderValue::from_str("Bearer token").unwrap())
    );

    Ok(())
}

#[test]
fn test_add_header() -> Result<(), DeboaError> {
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
fn test_set_text_body() -> Result<(), DeboaError> {
    let request = DeboaRequest::post(JSONPLACEHOLDER)?.text("test").build()?;

    assert_eq!(*request.raw_body(), b"test".to_vec());

    Ok(())
}

#[test]
fn test_raw_body() -> Result<(), DeboaError> {
    let request = DeboaRequest::post(JSONPLACEHOLDER)?.raw_body(b"test").build()?;

    assert_eq!(request.raw_body(), b"test");

    Ok(())
}

#[test]
fn test_fetch_from_string() -> Result<(), DeboaError> {
    let request = String::from(JSONPLACEHOLDER).fetch()?.build()?;

    assert_eq!(request.url(), JSONPLACEHOLDER);

    Ok(())
}

#[test]
fn test_fetch_from_url() -> Result<(), DeboaError> {
    let url = Url::parse(JSONPLACEHOLDER).unwrap();
    let request = url.fetch()?.build()?;

    assert_eq!(request.url(), JSONPLACEHOLDER);

    Ok(())
}

#[test]
fn test_fetch_from_str() -> Result<(), DeboaError> {
    let request = JSONPLACEHOLDER.fetch()?.build()?;

    assert_eq!(request.url(), JSONPLACEHOLDER);

    Ok(())
}
