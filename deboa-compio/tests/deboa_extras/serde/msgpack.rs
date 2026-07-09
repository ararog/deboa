use crate::common::{
    data::{sample_post, Post, MSGPACK_POST},
    helpers::fake_url,
};
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};
use deboa_extras::serde::msgpack::MsgPackBody;
use http::header;
use http::StatusCode;
use http_body_util::BodyExt;
use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]
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

#[apply(test!)]
async fn test_msgpack_response() -> Result<()> {
    let data = sample_post();

    let response = DeboaResponse::builder(fake_url())
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(&MSGPACK_POST[..])
        .build();
    let response: Post = response
        .body_as(MsgPackBody)
        .await?;

    assert_eq!(response, data);
    Ok(())
}
