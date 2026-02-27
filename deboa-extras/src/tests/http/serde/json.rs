use crate::http::serde::json::JsonBody;
use deboa::{request::DeboaRequest, response::DeboaResponse};
use deboa_tests::data::{sample_post, Post, JSON_POST};
use deboa_tests::utils::fake_url;
use http_body_util::BodyExt;

#[tokio::test]
async fn test_set_json() -> Result<(), Box<dyn std::error::Error>> {
    let request = DeboaRequest::post(fake_url())?
        .body_as(JsonBody, sample_post())?
        .build()?;

    let bytes = request
        .body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    assert_eq!(bytes, JSON_POST[..]);

    Ok(())
}

#[tokio::test]
async fn test_response_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = sample_post();

    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(&JSON_POST[..])
        .build();
    let response: Post = response
        .body_as(JsonBody)
        .await?;

    assert_eq!(response, data);

    Ok(())
}
