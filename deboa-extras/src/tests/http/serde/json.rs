use deboa::errors::DeboaError;
use deboa::{request::DeboaRequest, response::DeboaResponse};

use crate::http::serde::json::JsonBody;
use deboa_tests::data::{sample_post, Post, JSON_POST};
use deboa_tests::utils::fake_url;

#[cfg(feature = "json")]
#[test]
fn test_set_json() -> Result<(), DeboaError> {
    let request = DeboaRequest::post(fake_url())?.body_as(JsonBody, sample_post())?.build()?;

    assert_eq!(*request.raw_body(), JSON_POST.to_vec());

    Ok(())
}

#[cfg(feature = "json")]
#[tokio::test]
async fn test_response_json() -> Result<(), DeboaError> {
    let data = sample_post();

    let response = DeboaResponse::new(fake_url(), http::StatusCode::OK, http::HeaderMap::new(), JSON_POST.as_ref());
    let response: Post = response.body_as(JsonBody)?;

    assert_eq!(response, data);

    Ok(())
}
