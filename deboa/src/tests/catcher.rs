use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use httpmock::MockServer;

use crate::{
    catcher::{DeboaCatcher, MockDeboaCatcher},
    request::DeboaRequest,
    response::DeboaResponse,
    Deboa,
};

use deboa_tests::utils::{setup_server, url_from_string};

#[tokio::test]
async fn test_catcher_request() {
    let mut mock = MockDeboaCatcher::new();
    let mut request = DeboaRequest::get("https://httpbin.org/get")
        .expect("REASON")
        .build()
        .unwrap();
    mock.expect_on_request().returning(move |req| {
        req.headers_mut()
            .insert("test", HeaderValue::from_str("test").unwrap());
        Ok(None)
    });

    let _ = mock.on_request(&mut request).await;
    assert_eq!(
        request.headers().get("test"),
        Some(&HeaderValue::from_str("test").unwrap())
    );
}

#[tokio::test]
async fn test_catcher_response() {
    let server = MockServer::start();

    let http_mock = setup_server(&server, "/get", httpmock::Method::GET, StatusCode::OK);

    let mut catcher_mock = MockDeboaCatcher::new();
    catcher_mock
        .expect_on_request()
        .times(1)
        .returning(move |_| Ok(None));
    catcher_mock
        .expect_on_response()
        .times(1)
        .returning(move |res| {
            res.set_raw_body(Bytes::from("test"));
            Ok(())
        });

    let client = Deboa::builder().catch(catcher_mock).build();
    let mut response = DeboaRequest::get(server.url("/get").as_str())
        .expect("Invalid URL")
        .go(client)
        .await
        .unwrap();

    http_mock.assert();

    assert_eq!(response.raw_body().await, b"test");
}

#[tokio::test]
async fn test_catcher_early_response() {
    let server = MockServer::start();

    let http_mock = setup_server(&server, "/get", httpmock::Method::GET, StatusCode::OK);

    let mut catcher_mock = MockDeboaCatcher::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("test"),
        HeaderValue::from_static("test"),
    );

    let url = url_from_string(server.url("/get").to_string());

    catcher_mock
        .expect_on_request()
        .times(1)
        .returning(move |_| {
            Ok(Some(DeboaResponse::builder(url.clone())
                .status(StatusCode::OK)
                .headers(headers.clone())
                .body(&b"test"[..])
                .build()))
        });

    catcher_mock
        .expect_on_response()
        .times(1)
        .return_const(Ok(()));

    let client = Deboa::builder().catch(catcher_mock).build();
    let response = DeboaRequest::get(server.url("/get").as_str())
        .expect("Invalid URL")
        .go(client)
        .await
        .unwrap();

    http_mock.assert_calls(0);

    assert_eq!(response.headers().get("test").unwrap(), "test");
}
