use crate::http::serde::msgpack::MsgPackBody;
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};
use deboa_tests::data::{sample_post, Post, MSGPACK_POST};
use deboa_tests::utils::fake_url;
use http_body_util::BodyExt;

#[tokio::test]
async fn test_set_msgpack() -> Result<()> {
    let request = DeboaRequest::post(fake_url())?
        .body_as(MsgPackBody, sample_post())?
        .build()?;

    let bytes = request
        .body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    assert_eq!(bytes, MSGPACK_POST[..]);

    Ok(())
}

#[tokio::test]
async fn test_msgpack_response() -> Result<()> {
    let data = sample_post();

    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/msgpack")
        .body(&MSGPACK_POST[..])
        .build();
    let response: Post = response
        .body_as(MsgPackBody)
        .await?;

    assert_eq!(response, data);
    Ok(())
}
