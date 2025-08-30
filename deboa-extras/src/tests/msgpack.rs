#[test]
fn test_set_msgpack() -> Result<(), DeboaError> {
    use crate::tests::types::RAW_POST;

    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    let data = sample_post();

    let _ = api.set_msgpack(data);

    assert_eq!(api.body, Some(RAW_POST.to_vec()));

    Ok(())
}

#[tokio::test]
async fn test_msgpack_response() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let data = sample_post();

    let http_mock = server.mock(|when, then| {
        when.method(GET).path("/posts/1");
        then.status(200)
            .header(header::CONTENT_TYPE.as_str(), crate::APPLICATION_MSGPACK)
            .body(RAW_POST);
    });

    let server_address = *server.address();

    let ip = server_address.ip();
    let port = server_address.port();

    let mut api = Deboa::new(&format!("http://{ip}:{port}"))?;
    api.edit_header(header::CONTENT_TYPE, crate::APPLICATION_MSGPACK.to_string());
    api.edit_header(header::ACCEPT, crate::APPLICATION_MSGPACK.to_string());

    let response = api.get("/posts/1").await?.msgpack::<Post>().await?;

    http_mock.assert();

    assert_eq!(response, data);
    Ok(())
}
