use crate::http::serde::xml::XmlBody;
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};

use deboa_tests::{
    data::{sample_post, Post, XML_POST},
    utils::fake_url,
};

#[tokio::test]
async fn test_set_xml() -> Result<()> {
    let request = DeboaRequest::post(fake_url())?
        .body_as(XmlBody, sample_post())?
        .build()?;

    assert_eq!(*request.raw_body(), XML_POST.to_vec());

    Ok(())
}

#[tokio::test]
async fn test_xml_response() -> Result<()> {
    let data = sample_post();

    let response = DeboaResponse::new(
        fake_url(),
        http::StatusCode::OK,
        http::HeaderMap::new(),
        &XML_POST.to_vec(),
    );

    let response: Post = response.body_as(XmlBody)?;

    assert_eq!(response, data);

    Ok(())
}
