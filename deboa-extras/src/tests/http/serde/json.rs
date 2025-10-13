use crate::http::serde::json::JsonBody;
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};
use deboa_tests::data::{sample_post, Post, JSON_POST};
use deboa_tests::utils::fake_url;

#[cfg(feature = "json")]
#[test]
fn test_set_json() -> Result<()> {
    let request = DeboaRequest::post(fake_url())?
        .body_as(JsonBody, sample_post())?
        .build()?;

    assert_eq!(*request.raw_body(), JSON_POST[..]);

    Ok(())
}

#[cfg(feature = "json")]
#[tokio::test]
async fn test_response_json() -> Result<()> {
    let data = sample_post();

    let mut response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(&JSON_POST[..])
        .build();
    let response: Post = response.body_as(JsonBody).await?;

    assert_eq!(response, data);

    Ok(())
}
