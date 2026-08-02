use crate::common::{
    data::{sample_post, Post, XML_POST},
    helpers::{default_protocol_version, fake_url},
};
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};
use deboa_extras::serde::xml::XmlBody;
use http::header;
use http::StatusCode;
use http_body_util::BodyExt;

#[compio::test]
async fn test_set_xml() -> Result<()> {
    let request = DeboaRequest::post(fake_url())?
        .version(default_protocol_version())
        .body_as(XmlBody, sample_post())?
        .build()?;

    let bytes = request
        .body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    assert_eq!(bytes, XML_POST[..]);

    Ok(())
}

#[compio::test]
async fn test_xml_response() -> Result<()> {
    let data = sample_post();

    let response = DeboaResponse::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml")
        .body(&XML_POST[..])
        .build();

    let response: Post = response
        .body_as(XmlBody)
        .await?;

    assert_eq!(response, data);

    Ok(())
}
