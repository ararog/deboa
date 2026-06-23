use crate::{
    request::{DeboaRequest, IntoRequest, MethodExt},
    tests::{test_url, TEST_URL},
};
use http::{header, HeaderValue, Method};
use std::{error::Error, str::FromStr, sync::Arc};
use url::Url;

#[test]
fn test_method_ext_from_url() -> Result<(), Box<dyn Error>> {
    let request = Method::GET
        .from_url(TEST_URL)?
        .build()?;
    assert_eq!(request.method(), &Method::GET);
    assert_eq!(*request.url(), test_url());
    Ok(())
}

#[test]
fn test_method_ext_to_url() -> Result<(), Box<dyn Error>> {
    let request = Method::POST
        .to_url(TEST_URL)?
        .build()?;
    assert_eq!(request.method(), &Method::POST);
    assert_eq!(*request.url(), test_url());
    Ok(())
}

#[test]
fn test_str_method_ext_from_url() -> Result<(), Box<dyn Error>> {
    let request = "GET"
        .from_url(TEST_URL)?
        .build()?;
    assert_eq!(request.method(), &Method::GET);
    assert_eq!(*request.url(), test_url());
    Ok(())
}

#[test]
fn test_str_method_ext_to_url() -> Result<(), Box<dyn Error>> {
    let request = "POST"
        .to_url(TEST_URL)?
        .build()?;
    assert_eq!(request.method(), &Method::POST);
    assert_eq!(*request.url(), test_url());
    Ok(())
}

#[test]
fn test_into_url() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url)?.build()?;
    assert_eq!(*request.url(), test_url());
    Ok(())
}

#[test]
fn test_into_request_from_str() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = url
        .clone()
        .into_request()?;
    assert_eq!(*request.url(), url);
    Ok(())
}

#[test]
fn test_into_request_from_string() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let post_url = format!("{}posts/{}", &url, 1);
    let request = post_url
        .clone()
        .into_request()?;
    assert_eq!(*request.url(), url.join("/posts/1")?);
    Ok(())
}

#[test]
fn test_into_str() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url.clone())?.build()?;
    assert_eq!(*request.url(), url);
    Ok(())
}

#[test]
fn test_into_string() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url.clone())?.build()?;
    assert_eq!(*request.url(), url);
    Ok(())
}

#[test]
fn test_from_str_method_and_url() -> Result<(), Box<dyn Error>> {
    let request = DeboaRequest::from_str(
        r##"
    GET https://localhost:8000
    "##,
    )?;
    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.url(), Arc::new(Url::parse("https://localhost:8000").unwrap()));
    Ok(())
}

#[test]
fn test_from_str_headers() -> Result<(), Box<dyn Error>> {
    let request = DeboaRequest::from_str(
        r##"
    GET https://localhost:8000
    Content-Type: application/json
    "##,
    )?;
    assert_eq!(
        request
            .headers()
            .get(header::CONTENT_TYPE),
        Some(&HeaderValue::from_str("application/json").unwrap())
    );
    Ok(())
}

#[test]
fn test_set_retries() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let api = DeboaRequest::get(url)?
        .retries(5)
        .build()?;
    assert_eq!(api.retries(), 5);
    Ok(())
}

#[test]
fn test_base_url() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let api = DeboaRequest::get(url.clone())?.build()?;
    assert_eq!(*api.url(), url);
    Ok(())
}

#[test]
fn test_set_headers() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url)?
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(&header::CONTENT_TYPE),
        Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())
    );

    Ok(())
}

#[test]
fn test_set_headers_as_tuple() -> Result<(), Box<dyn Error>> {
    let headers = vec![(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string())];
    let request = DeboaRequest::get(test_url())?
        .headers(headers)
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(&header::CONTENT_TYPE),
        Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())
    );

    Ok(())
}

#[test]
fn test_set_basic_auth() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url)?
        .basic_auth("username", "password")
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(&header::AUTHORIZATION),
        Some(&HeaderValue::from_str("Basic dXNlcm5hbWU6cGFzc3dvcmQ=").unwrap())
    );

    Ok(())
}

#[test]
fn test_set_bearer_auth() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url)?
        .bearer_auth("token")
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(&header::AUTHORIZATION),
        Some(&HeaderValue::from_str("Bearer token").unwrap())
    );

    Ok(())
}

#[test]
fn test_add_header() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url)?
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(&header::CONTENT_TYPE),
        Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())
    );

    Ok(())
}
