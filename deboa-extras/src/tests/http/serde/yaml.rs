use crate::http::serde::yaml::YamlBody;
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};
use deboa_tests::data::{sample_post, Post, YAML_POST};
use deboa_tests::utils::fake_url;
use http_body_util::BodyExt;

#[tokio::test]
async fn test_set_yaml() -> Result<()> {
    let request = DeboaRequest::post(fake_url())?
        .body_as(YamlBody, sample_post())?
        .build()?;

    let bytes = request
        .body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    assert_eq!(bytes, YAML_POST[..]);

    Ok(())
}

#[tokio::test]
async fn test_response_yaml() -> Result<()> {
    let data = sample_post();

    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/yaml")
        .body(&YAML_POST[..])
        .build();
    let response: Post = response
        .body_as(YamlBody)
        .await?;

    assert_eq!(response, data);

    Ok(())
}
