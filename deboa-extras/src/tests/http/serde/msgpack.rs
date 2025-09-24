use crate::http::serde::msgpack::MsgPackBody;
use deboa::{errors::DeboaError, request::DeboaRequest, response::DeboaResponse};

use deboa_tests::data::{MSGPACK_POST, Post, sample_post};
use deboa_tests::utils::fake_url;

#[test]
fn test_set_msgpack() -> Result<(), DeboaError> {
    let request = DeboaRequest::post(fake_url())?.body_as(MsgPackBody, sample_post())?.build()?;

    assert_eq!(*request.raw_body(), MSGPACK_POST.to_vec());

    Ok(())
}

#[tokio::test]
async fn test_msgpack_response() -> Result<(), DeboaError> {
    let data = sample_post();

    let response = DeboaResponse::new(fake_url(), http::StatusCode::OK, http::HeaderMap::new(), &MSGPACK_POST.to_vec());
    let response: Post = response.body_as(MsgPackBody)?;

    assert_eq!(response, data);
    Ok(())
}
