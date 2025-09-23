use crate::http::serde::json::JsonBody;
use deboa::errors::DeboaError;
use deboa::{request::DeboaRequest, response::DeboaResponse};

use crate::tests::types::{JSON_POST, Post, sample_post, url};

#[cfg(feature = "json")]
#[test]
fn test_set_json() -> Result<(), DeboaError> {
    let request = DeboaRequest::post(url())?.body_as(JsonBody, sample_post())?.build()?;

    assert_eq!(*request.raw_body(), JSON_POST.to_vec());

    Ok(())
}

#[cfg(feature = "json")]
#[tokio::test]
async fn test_response_json() -> Result<(), DeboaError> {
    let data = sample_post();

    let response = DeboaResponse::new(url(), http::StatusCode::OK, http::HeaderMap::new(), JSON_POST.as_ref());
    let response: Post = response.body_as(JsonBody)?;

    assert_eq!(response, data);

    Ok(())
}
