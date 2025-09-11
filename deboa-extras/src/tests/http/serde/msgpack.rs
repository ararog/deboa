use crate::http::serde::msgpack::MsgPackBody;
use deboa::{Deboa, errors::DeboaError};

use http::header;

use httpmock::{Method::GET, MockServer};
use mime_typed::Msgpack;

use crate::tests::types::{JSONPLACEHOLDER, MSGPACK_POST, Post, format_address, sample_post};

#[test]
fn test_set_msgpack() -> Result<(), DeboaError> {
    let request = DeboaRequest::post("posts/1").body_as(MsgPackBody, sample_post())?.build()?;

    assert_eq!(*request.raw_body(), MSGPACK_POST.to_vec());

    Ok(())
}

#[tokio::test]
async fn test_msgpack_response() -> Result<(), DeboaError> {
    let data = sample_post();

    let response = DeboaResponse::new(http::StatusCode::OK, http::HeaderMap::new(), &MSGPACK_POST.to_vec());
    let response: Post = response.body_as(MsgPackBody)?;

    assert_eq!(response, data);
    Ok(())
}
