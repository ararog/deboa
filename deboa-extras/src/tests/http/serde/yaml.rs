use crate::http::serde::yaml::YamlBody;
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};
use deboa_tests::data::{sample_post, Post, YAML_POST};
use deboa_tests::utils::fake_url;

#[cfg(feature = "yaml")]
#[test]
fn test_set_yaml() -> Result<()> {
    let request = DeboaRequest::post(fake_url())?
        .body_as(YamlBody, sample_post())?
        .build()?;

    assert_eq!(*request.raw_body(), YAML_POST[..]);

    Ok(())
}

#[cfg(feature = "yaml")]
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
