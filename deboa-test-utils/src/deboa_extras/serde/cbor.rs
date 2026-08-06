use crate::common::{
    data::{sample_post, Post},
    helpers::fake_url,
};
use caramelo::{
    expect,
    matchers::{eq, err},
};
use deboa::{request::DeboaRequest, response::DeboaResponse, Result, TestResult};
use deboa_extras::serde::cbor::CborBody;
use http::header;
use http::StatusCode;
use http_body_util::BodyExt;

fn build_sample_cbor_body() -> Vec<u8> {
    let mut buf = Vec::new();
    let body = sample_post();
    ciborium::ser::into_writer(&body, &mut buf).unwrap();
    buf
}

pub async fn test_set_cbor() -> TestResult<()> {
    let request = DeboaRequest::post(fake_url())?
        .body_as(CborBody, sample_post())?
        .build()?;

    let bytes = request
        .body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    assert_eq!(bytes, build_sample_cbor_body());

    Ok(())
}

pub fn test_set_cbor_register_headers() -> TestResult<()> {
    let request = DeboaRequest::post(fake_url())?
        .body_as(CborBody, sample_post())?
        .build()?;

    expect(
        request
            .headers()
            .get(header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .to_be(eq("application/cbor"));
    expect(
        request
            .headers()
            .get(header::ACCEPT)
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .to_be(eq("application/cbor"));

    Ok(())
}

pub async fn test_response_cbor() -> TestResult<()> {
    let data = sample_post();

    let response = DeboaResponse::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/cbor")
        .body(build_sample_cbor_body())
        .build();
    let response: Post = response
        .body_as(CborBody)
        .await?;

    expect(response).to_be(eq(data));

    Ok(())
}

pub async fn test_response_cbor_invalid_body() -> TestResult<()> {
    let response = DeboaResponse::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/cbor")
        .body(vec![0xff])
        .build();

    let result: Result<Post> = response
        .body_as(CborBody)
        .await;

    expect(result).to_be(err());

    Ok(())
}
