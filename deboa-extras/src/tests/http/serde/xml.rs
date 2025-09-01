#[tokio::test]
async fn test_set_xml() -> Result<(), DeboaError> {
    use crate::tests::types::{XML_POST, sample_post};

    let mut api = Deboa::new("https://reqbin.com")?;

    let data = sample_post();

    let _ = api.set_xml(data)?;

    assert_eq!(api.body, Some(XML_POST.to_vec()));

    Ok(())
}

#[tokio::test]
async fn test_xml_response() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let data = sample_post();

    let http_mock = server.mock(|when, then| {
        when.method(GET).path("/posts/1");
        then.status(200)
            .header(header::CONTENT_TYPE.as_str(), crate::APPLICATION_XML)
            .body(XML_POST);
    });

    let server_address = *server.address();

    let ip = server_address.ip();
    let port = server_address.port();

    let mut api = Deboa::new(&format!("http://{ip}:{port}"))?;
    api.edit_header(header::CONTENT_TYPE, crate::APPLICATION_XML.to_string());
    api.edit_header(header::ACCEPT, crate::APPLICATION_XML.to_string());

    let response = api.get("/posts/1").await?.xml::<Post>().await?;

    http_mock.assert();

    assert_eq!(response, data);
    Ok(())
}
