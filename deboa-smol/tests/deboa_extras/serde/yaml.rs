use crate::common::{
    data::{sample_post, Post, YAML_POST},
    helpers::{default_protocol_version, fake_url},
};
use deboa::{request::DeboaRequest, response::DeboaResponse, TestResult};
use deboa_extras::serde::yaml::YamlBody;
use http::header;
use http::StatusCode;
use http_body_util::BodyExt;
use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]
async fn test_set_yaml() -> TestResult<()> {
    let request = DeboaRequest::post(fake_url())?
        .version(default_protocol_version())
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

#[apply(test!)]
async fn test_response_yaml() -> TestResult<()> {
    let data = sample_post();

    let response = DeboaResponse::builder()
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
