use crate::http::serde::msgpack::MsgPackBody;
use deboa::{Deboa, errors::DeboaError};

use http::header;

use httpmock::{Method::GET, MockServer};
use mime_typed::Msgpack;

use crate::tests::types::{JSONPLACEHOLDER, MSGPACK_POST, Post, format_address, sample_post};

#[test]
fn test_set_msgpack() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    let data = sample_post();

    let _ = api.set_body_as(MsgPackBody, data);

    assert_eq!(api.raw_body(), &MSGPACK_POST.to_vec());

    Ok(())
}

#[tokio::test]
async fn test_msgpack_response() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let data = sample_post();

    let http_mock = server.mock(|when, then| {
        when.method(GET).path("/posts/1");
        then.status(200)
            .header(http::header::CONTENT_TYPE.as_str(), Msgpack.to_string().as_str())
            .body(MSGPACK_POST);
    });

    let mut api = Deboa::new(&format_address(&server))?;
    api.edit_header(header::CONTENT_TYPE, Msgpack.to_string().as_str());
    api.edit_header(header::ACCEPT, Msgpack.to_string().as_str());

    let response: Post = api.get("/posts/1").await?.body_as(MsgPackBody)?;

    http_mock.assert();

    assert_eq!(response, data);
    Ok(())
}
