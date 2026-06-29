use crate::common::{
    data::{sample_post, Post, YAML_POST},
    helpers::fake_url,
};
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};
use deboa_extras::serde::yaml::YamlBody;
use http::header;
use http::StatusCode;
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
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/yaml")
        .body(&YAML_POST[..])
        .build();
    let response: Post = response
        .body_as(YamlBody)
        .await?;

    assert_eq!(response, data);

    Ok(())
}
