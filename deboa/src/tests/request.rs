use crate::errors::DeboaError;
#[cfg(feature = "middlewares")]
use crate::Deboa;

use crate::tests::types::JSONPLACEHOLDER;
use http::header;
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

#[test]
fn test_base_url() -> Result<(), DeboaError> {
    let api = Deboa::new(JSONPLACEHOLDER)?;

    assert_eq!(api.base_url, Url::parse(JSONPLACEHOLDER).unwrap());

    Ok(())
}

#[test]
fn test_invalid_url() -> Result<(), DeboaError> {
    let api = Deboa::new("invalid_url");

    assert!(api.is_err());

    Ok(())
}

#[test]
fn test_set_query_params() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    let query_map = HashMap::from([("id".to_string(), "1".to_string())]);

    api.set_query_params(query_map.clone());

    assert_eq!(api.query_params, Some(query_map));

    Ok(())
}

#[test]
fn test_set_headers() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    let headers = HashMap::from([(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string())]);

    api.headers = Some(headers);

    assert_eq!(
        api.headers,
        Some(HashMap::from([(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string())]))
    );

    Ok(())
}

#[test]
fn test_set_basic_auth() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.add_basic_auth("username", "password");

    assert_eq!(
        api.get_mut_header(&header::AUTHORIZATION),
        Some(&mut "Basic dXNlcm5hbWU6cGFzc3dvcmQ=".to_string())
    );

    Ok(())
}

#[test]
fn test_set_bearer_auth() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.add_bearer_auth("token");

    assert_eq!(api.get_mut_header(&header::AUTHORIZATION), Some(&mut "Bearer token".to_string()));

    Ok(())
}

#[test]
fn test_set_retries() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.set_retries(5);

    assert_eq!(api.retries, 5);

    Ok(())
}

#[test]
fn test_set_connection_timeout() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.set_connection_timeout(5);

    assert_eq!(api.connection_timeout, 5);

    Ok(())
}

#[test]
fn test_set_request_timeout() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.set_request_timeout(5);

    assert_eq!(api.request_timeout, 5);

    Ok(())
}

#[test]
fn test_edit_header() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.edit_header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref());

    assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), Some(&mut mime::APPLICATION_JSON.to_string()));

    Ok(())
}

#[test]
fn test_add_header() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.add_header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref());

    assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), Some(&mut mime::APPLICATION_JSON.to_string()));

    Ok(())
}

#[test]
fn test_remove_header() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.remove_header(header::CONTENT_TYPE);

    assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), None);

    Ok(())
}

#[test]
fn test_set_text_body() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.set_text("test".to_string());

    assert_eq!(api.body, b"test".to_vec().into());

    Ok(())
}

#[test]
fn test_raw_body() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.set_raw_body(b"test");

    assert_eq!(api.raw_body(), b"test");

    Ok(())
}

#[test]
fn test_get_mut_header() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    api.add_header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref());

    assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), Some(&mut mime::APPLICATION_JSON.to_string()));

    Ok(())
}
