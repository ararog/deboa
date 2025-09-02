use crate::tests::types::format_address;
use deboa::{Deboa, errors::DeboaError};
use http::header;
use httpmock::{Method::GET, MockServer};

use crate::{
    http::serde::xml::XmlBody,
    tests::types::{Post, XML_POST, sample_post},
};

#[tokio::test]
async fn test_set_xml() -> Result<(), DeboaError> {
    use crate::tests::types::{XML_POST, sample_post};

    let mut api = Deboa::new("https://reqbin.com")?;

    let data = sample_post();

    let _ = api.set_body_as::<XmlBody, Post>(XmlBody, data)?;

    assert_eq!(api.raw_body(), &XML_POST.to_vec());

    Ok(())
}

#[tokio::test]
async fn test_xml_response() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let data = sample_post();

    let http_mock = server.mock(|when, then| {
        when.method(GET).path("/posts/1");
        then.status(200).header(header::CONTENT_TYPE.as_str(), "application/xml").body(XML_POST);
    });

    let mut api = Deboa::new(&format_address(&server))?;
    api.edit_header(header::CONTENT_TYPE, "application/xml".to_string());
    api.edit_header(header::ACCEPT, "application/xml".to_string());

    let response: Post = api.get("/posts/1").await?.body_as(XmlBody)?;

    http_mock.assert();

    assert_eq!(response, data);
    Ok(())
}
