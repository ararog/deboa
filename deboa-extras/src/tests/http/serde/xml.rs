use deboa::{errors::DeboaError, request::DeboaRequest, response::DeboaResponse};

use crate::{
    http::serde::xml::XmlBody,
    tests::types::{Post, XML_POST, sample_post},
};

#[tokio::test]
async fn test_set_xml() -> Result<(), DeboaError> {
    let request = DeboaRequest::post("posts/1").body_as(XmlBody, sample_post())?.build()?;

    assert_eq!(*request.raw_body(), XML_POST.to_vec());

    Ok(())
}

#[tokio::test]
async fn test_xml_response() -> Result<(), DeboaError> {
    let data = sample_post();

    let response = DeboaResponse::new(http::StatusCode::OK, http::HeaderMap::new(), &XML_POST.to_vec());

    let response: Post = response.body_as(XmlBody)?;

    assert_eq!(response, data);

    Ok(())
}
