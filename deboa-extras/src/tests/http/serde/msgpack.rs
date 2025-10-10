use crate::http::serde::msgpack::MsgPackBody;
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};

use deboa_tests::data::{sample_post, Post, MSGPACK_POST};
use deboa_tests::utils::fake_url;

#[test]
fn test_set_msgpack() -> Result<()> {
    let request = DeboaRequest::post(fake_url())?
        .body_as(MsgPackBody, sample_post())?
        .build()?;

    assert_eq!(*request.raw_body(), MSGPACK_POST[..]);

    Ok(())
}

#[tokio::test]
async fn test_msgpack_response() -> Result<()> {
    let data = sample_post();

    let mut response = DeboaResponse::new(
        fake_url(),
        http::StatusCode::OK,
        http::HeaderMap::new(),
        &MSGPACK_POST[..],
    );
    let response: Post = response.body_as(MsgPackBody).await?;

    assert_eq!(response, data);
    Ok(())
}
