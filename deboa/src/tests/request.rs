use std::{str::FromStr, sync::Arc};

use crate::{
    request::{DeboaRequest, IntoRequest, MethodExt},
    Result,
};

use deboa_tests::utils::{test_url, url_from_string};

use http::{header, HeaderValue, Method};

use url::Url;

#[test]
fn test_method_ext_from_url() -> Result<()> {
    let test_url = test_url(None);
    let request = Method::GET
        .from_url(&test_url)?
        .build()?;
    assert_eq!(request.method(), &Method::GET);
    assert_eq!(*request.url(), url_from_string(test_url));
    Ok(())
}

#[test]
fn test_method_ext_to_url() -> Result<()> {
    let test_url = test_url(None);
    let request = Method::POST
        .to_url(&test_url)?
        .build()?;
    assert_eq!(request.method(), &Method::POST);
    assert_eq!(*request.url(), url_from_string(test_url));
    Ok(())
}

#[test]
fn test_str_method_ext_from_url() -> Result<()> {
    let test_url = test_url(None);
    let request = "GET"
        .from_url(&test_url)?
        .build()?;
    assert_eq!(request.method(), &Method::GET);
    assert_eq!(*request.url(), url_from_string(test_url));
    Ok(())
}

#[test]
fn test_str_method_ext_to_url() -> Result<()> {
    let test_url = test_url(None);
    let request = "POST"
        .to_url(&test_url)?
        .build()?;
    assert_eq!(request.method(), &Method::POST);
    assert_eq!(*request.url(), url_from_string(test_url));
    Ok(())
}

#[test]
fn test_into_url() -> Result<()> {
    let test_url = test_url(None);
    let url = Url::parse(&test_url).unwrap();
    let request = DeboaRequest::get(url)?.build()?;
    assert_eq!(*request.url(), url_from_string(test_url));
    Ok(())
}

#[test]
fn test_into_request_from_str() -> Result<()> {
    let test_url = test_url(None);
    let request = test_url
        .clone()
        .into_request()?;
    assert_eq!(*request.url(), url_from_string(test_url));
    Ok(())
}

#[test]
fn test_into_request_from_string() -> Result<()> {
    let test_url = test_url(None);
    let post_url = format!("{}posts/{}", &test_url, 1);
    let request = post_url
        .clone()
        .into_request()?;
    assert_eq!(*request.url(), url_from_string(post_url));
    Ok(())
}

#[test]
fn test_into_str() -> Result<()> {
    let test_url = test_url(None);
    let request = DeboaRequest::get(&test_url)?.build()?;
    assert_eq!(*request.url(), url_from_string(test_url));
    Ok(())
}

#[test]
fn test_into_string() -> Result<()> {
    let test_url = test_url(None);
    let request = DeboaRequest::get(&test_url)?.build()?;
    assert_eq!(*request.url(), url_from_string(test_url));
    Ok(())
}

#[test]
fn test_from_str_method_and_url() -> Result<()> {
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
fn test_from_str_headers() -> Result<()> {
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
fn test_from_str_body() -> Result<()> {
    let request = DeboaRequest::from_str(
        r##"
    GET https://localhost:8000
    Content-Type: application/json
    
    {"title": "foo", "body": "bar", "userId": 1}
    "##,
    )?;
    assert_eq!(request.raw_body(), b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}");
    Ok(())
}

#[test]
fn test_set_retries() -> Result<()> {
    let api = DeboaRequest::get(test_url(None))?
        .retries(5)
        .build()?;
    assert_eq!(api.retries(), 5);
    Ok(())
}

#[test]
fn test_base_url() -> Result<()> {
    let test_url = test_url(None);
    let api = DeboaRequest::get(&test_url)?.build()?;
    assert_eq!(*api.url(), url_from_string(test_url));
    Ok(())
}

#[test]
fn test_set_headers() -> Result<()> {
    let test_url = test_url(None);
    let request = DeboaRequest::get(&test_url)?
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
fn test_set_headers_as_tuple() -> Result<()> {
    let headers = vec![(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string())];
    let request = DeboaRequest::get(test_url(None))?
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
fn test_set_basic_auth() -> Result<()> {
    let test_url = test_url(None);
    let request = DeboaRequest::get(&test_url)?
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
fn test_set_bearer_auth() -> Result<()> {
    let test_url = test_url(None);
    let request = DeboaRequest::get(&test_url)?
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
fn test_add_header() -> Result<()> {
    let test_url = test_url(None);
    let request = DeboaRequest::get(&test_url)?
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
fn test_set_text_body() -> Result<()> {
    let test_url = test_url(None);
    let request = DeboaRequest::post(&test_url)?
        .text("test")
        .build()?;

    assert_eq!(*request.raw_body(), b"test"[..]);

    Ok(())
}

#[test]
fn test_raw_body() -> Result<()> {
    let test_url = test_url(None);
    let request = DeboaRequest::post(&test_url)?
        .raw_body(b"test")
        .build()?;

    assert_eq!(request.raw_body(), b"test");

    Ok(())
}
