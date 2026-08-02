use crate::{
    request::{DeboaRequest, IntoRequest, MethodExt},
    tests::{test_uri, test_url, TEST_URL},
};
use caramelo::{expect, matchers::eq};
use http::{header, HeaderValue, Method, Uri};
use std::{error::Error, str::FromStr};

#[test]
fn test_method_ext_from_url() -> Result<(), Box<dyn Error>> {
    let request = Method::GET
        .from_url(TEST_URL)?
        .build()?;
    expect(request.method()).to_be(eq(&Method::GET));
    expect(request.uri()).to_be(eq(&test_uri()));
    Ok(())
}

#[test]
fn test_method_ext_to_url() -> Result<(), Box<dyn Error>> {
    let request = Method::POST
        .to_url(TEST_URL)?
        .build()?;
    expect(request.method()).to_be(eq(&Method::POST));
    expect(request.uri()).to_be(eq(&test_uri()));
    Ok(())
}

#[test]
fn test_str_method_ext_from_url() -> Result<(), Box<dyn Error>> {
    let request = "GET"
        .from_url(TEST_URL)?
        .build()?;
    expect(request.method()).to_be(eq(&Method::GET));
    expect(request.uri()).to_be(eq(&test_uri()));
    Ok(())
}

#[test]
fn test_str_method_ext_to_url() -> Result<(), Box<dyn Error>> {
    let request = "POST"
        .to_url(TEST_URL)?
        .build()?;
    assert_eq!(request.method(), &Method::POST);
    assert_eq!(*request.uri(), test_uri());
    Ok(())
}

#[test]
fn test_into_url() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url)?.build()?;
    expect(request.uri()).to_be(eq(&test_uri()));
    Ok(())
}

#[test]
fn test_into_request_from_str() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = url
        .clone()
        .into_request()?;
    expect(request.uri()).to_be(eq(&test_uri()));
    Ok(())
}

#[test]
fn test_into_request_from_string() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let post_url = format!("{}posts/{}", &url, 1);
    let request = post_url
        .clone()
        .into_request()?;
    let uri = Uri::from_str(
        url.join("/posts/1")?
            .as_ref(),
    )?;
    expect(request.uri()).to_be(eq(&uri));
    Ok(())
}

#[test]
fn test_into_str() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url.clone())?.build()?;
    expect(request.uri()).to_be(eq(&test_uri()));
    Ok(())
}

#[test]
fn test_into_string() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url.clone())?.build()?;
    expect(request.uri()).to_be(eq(&test_uri()));
    Ok(())
}

#[test]
fn test_from_str_method_and_url() -> Result<(), Box<dyn Error>> {
    let request = DeboaRequest::from_str(
        r##"
    GET https://localhost:8000
    "##,
    )?;
    expect(request.method()).to_be(eq(&Method::GET));
    expect(request.uri()).to_be(eq(&Uri::from_static("https://localhost:8000")));
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
    expect(
        request
            .headers()
            .get(header::CONTENT_TYPE),
    )
    .to_be(eq(Some(&HeaderValue::from_str("application/json").unwrap())));
    Ok(())
}

#[test]
fn test_base_url() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let api = DeboaRequest::get(url.clone())?.build()?;
    assert_eq!(*api.uri(), test_uri());
    Ok(())
}

#[test]
fn test_set_headers() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url)?
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .build()?;

    expect(
        request
            .headers()
            .get(&header::CONTENT_TYPE),
    )
    .to_be(eq(Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())));

    Ok(())
}

#[test]
fn test_set_headers_as_tuple() -> Result<(), Box<dyn Error>> {
    let headers = vec![(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string())];
    let request = DeboaRequest::get(test_url())?
        .headers(headers)
        .build()?;

    expect(
        request
            .headers()
            .get(&header::CONTENT_TYPE),
    )
    .to_be(eq(Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())));

    Ok(())
}

#[test]
fn test_set_basic_auth() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url)?
        .basic_auth("username", "password")
        .build()?;

    expect(
        request
            .headers()
            .get(&header::AUTHORIZATION),
    )
    .to_be(eq(Some(&HeaderValue::from_str("Basic dXNlcm5hbWU6cGFzc3dvcmQ=").unwrap())));

    Ok(())
}

#[test]
fn test_set_bearer_auth() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url)?
        .bearer_auth("token")
        .build()?;

    expect(
        request
            .headers()
            .get(&header::AUTHORIZATION),
    )
    .to_be(eq(Some(&HeaderValue::from_str("Bearer token").unwrap())));

    Ok(())
}

#[test]
fn test_add_header() -> Result<(), Box<dyn Error>> {
    let url = test_url();
    let request = DeboaRequest::get(url)?
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .build()?;

    expect(
        request
            .headers()
            .get(&header::CONTENT_TYPE),
    )
    .to_be(eq(Some(&HeaderValue::from_str(mime::APPLICATION_JSON.as_ref()).unwrap())));

    Ok(())
}
