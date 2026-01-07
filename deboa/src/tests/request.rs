use std::{str::FromStr, sync::Arc};

use crate::{
    request::{DeboaRequest, FetchWith, IntoRequest, MethodExt},
    Client, Result,
};

use deboa_tests::utils::JSONPLACEHOLDER;
use http::{header, HeaderValue, Method};
use url::Url;

#[test]
fn test_method_ext_from_url() -> Result<()> {
    let request = Method::GET
        .from_url(JSONPLACEHOLDER)?
        .build()?;
    assert_eq!(request.method(), &Method::GET);
    assert_eq!(
        request
            .url()
            .to_string(),
        JSONPLACEHOLDER
    );
    Ok(())
}

#[test]
fn test_method_ext_to_url() -> Result<()> {
    let request = Method::POST
        .to_url(JSONPLACEHOLDER)?
        .build()?;
    assert_eq!(request.method(), &Method::POST);
    assert_eq!(
        request
            .url()
            .to_string(),
        JSONPLACEHOLDER
    );
    Ok(())
}

#[test]
fn test_str_method_ext_from_url() -> Result<()> {
    let request = "GET"
        .from_url(JSONPLACEHOLDER)?
        .build()?;
    assert_eq!(request.method(), &Method::GET);
    assert_eq!(
        request
            .url()
            .to_string(),
        JSONPLACEHOLDER
    );
    Ok(())
}

#[test]
fn test_str_method_ext_to_url() -> Result<()> {
    let request = "POST"
        .to_url(JSONPLACEHOLDER)?
        .build()?;
    assert_eq!(request.method(), &Method::POST);
    assert_eq!(
        request
            .url()
            .to_string(),
        JSONPLACEHOLDER
    );
    Ok(())
}

#[test]
fn test_into_url() -> Result<()> {
    let url = Url::parse(JSONPLACEHOLDER).unwrap();
    let request = DeboaRequest::get(url)?.build()?;
    assert_eq!(
        request
            .url()
            .to_string(),
        JSONPLACEHOLDER
    );
    Ok(())
}

#[test]
fn test_into_request_from_str() -> Result<()> {
    let request = JSONPLACEHOLDER.into_request()?;
    assert_eq!(
        request
            .url()
            .to_string(),
        JSONPLACEHOLDER
    );
    Ok(())
}

#[test]
fn test_into_request_from_string() -> Result<()> {
    let request = format!("{}/posts/{}", JSONPLACEHOLDER, 1).into_request()?;
    assert_eq!(
        request
            .url()
            .to_string(),
        format!("{}/posts/{}", JSONPLACEHOLDER, 1)
    );
    Ok(())
}

#[test]
fn test_into_str() -> Result<()> {
    let request = DeboaRequest::get(JSONPLACEHOLDER)?.build()?;
    assert_eq!(
        request
            .url()
            .to_string(),
        JSONPLACEHOLDER
    );
    Ok(())
}

#[test]
fn test_into_string() -> Result<()> {
    let request = DeboaRequest::get(String::from(JSONPLACEHOLDER))?.build()?;
    assert_eq!(
        request
            .url()
            .to_string(),
        JSONPLACEHOLDER
    );
    Ok(())
}

#[tokio::test]
async fn test_try_into() -> Result<()> {
    let client = Client::default();
    let first_post = "https://jsonplaceholder.typicode.com/posts/1";
    let response = client
        .execute(first_post.into_request()?)
        .await?;
    assert_eq!(response.status(), 200);
    Ok(())
}

#[test]
fn test_from_str_method_and_url() -> Result<()> {
    let request = DeboaRequest::from_str(
        r##"
    GET https://jsonplaceholder.typicode.com
    "##,
    )?;
    assert_eq!(request.method(), Method::GET);
    assert_eq!(
        request.url(),
        Arc::new(Url::parse("https://jsonplaceholder.typicode.com").unwrap())
    );
    Ok(())
}

#[test]
fn test_from_str_headers() -> Result<()> {
    let request = DeboaRequest::from_str(
        r##"
    GET https://jsonplaceholder.typicode.com
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
    GET https://jsonplaceholder.typicode.com
    Content-Type: application/json
    
    {"title": "foo", "body": "bar", "userId": 1}
    "##,
    )?;
    assert_eq!(request.raw_body(), b"{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}");
    Ok(())
}

#[test]
fn test_set_retries() -> Result<()> {
    let api = DeboaRequest::get(JSONPLACEHOLDER)?
        .retries(5)
        .build()?;
    assert_eq!(api.retries(), 5);
    Ok(())
}

#[test]
fn test_base_url() -> Result<()> {
    let api = DeboaRequest::get(String::from(JSONPLACEHOLDER))?.build()?;
    assert_eq!(
        api.url()
            .to_string(),
        JSONPLACEHOLDER
    );
    Ok(())
}

#[test]
fn test_set_headers() -> Result<()> {
    let request = DeboaRequest::get(JSONPLACEHOLDER)?
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
    let request = DeboaRequest::get(JSONPLACEHOLDER)?
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
    let request = DeboaRequest::get(JSONPLACEHOLDER)?
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
    let request = DeboaRequest::get(JSONPLACEHOLDER)?
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
    let request = DeboaRequest::get(JSONPLACEHOLDER)?
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
    let request = DeboaRequest::post(JSONPLACEHOLDER)?
        .text("test")
        .build()?;

    assert_eq!(*request.raw_body(), b"test"[..]);

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
async fn test_fetch_from_str() -> Result<()> {
    let client = Client::default();

    let first_post = "https://jsonplaceholder.typicode.com/posts/1";

    let response = first_post
        .fetch_with(&client)
        .await?;
    assert_eq!(response.status(), 200);

    Ok(())
}
