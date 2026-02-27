use crate::http::serde::cbor::CborBody;
use deboa::errors::{ContentError, DeboaError};
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};
use deboa_tests::data::{sample_post, Post};
use deboa_tests::utils::fake_url;
use http_body_util::BodyExt;

fn build_sample_cbor_body() -> Vec<u8> {
    let mut buf = Vec::new();
    let body = sample_post();
    ciborium::ser::into_writer(&body, &mut buf).unwrap();
    buf
}

async fn do_test_set_cbor() -> Result<()> {
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

#[tokio::test]
async fn test_set_cbor() {
    do_test_set_cbor()
        .await
        .unwrap();
}

#[test]
fn test_set_cbor_registers_headers() -> Result<()> {
    let request = DeboaRequest::post(fake_url())?
        .body_as(CborBody, sample_post())?
        .build()?;

    assert_eq!(
        request
            .headers()
            .get(http::header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap(),
        "application/cbor"
    );
    assert_eq!(
        request
            .headers()
            .get(http::header::ACCEPT)
            .unwrap()
            .to_str()
            .unwrap(),
        "application/cbor"
    );

    Ok(())
}

#[tokio::test]
async fn test_response_cbor() -> Result<()> {
    let data = sample_post();

    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/cbor")
        .body(build_sample_cbor_body())
        .build();
    let response: Post = response
        .body_as(CborBody)
        .await?;

    assert_eq!(response, data);

    Ok(())
}

#[tokio::test]
async fn test_response_cbor_invalid_body() {
    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/cbor")
        .body(vec![0xff])
        .build();

    let result: Result<Post> = response
        .body_as(CborBody)
        .await;
    let err = result.unwrap_err();

    assert!(matches!(err, DeboaError::Content(ContentError::Deserialization { .. })));
}
