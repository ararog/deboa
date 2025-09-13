use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};

use crate::{
    Deboa,
    catcher::{DeboaCatcher, MockDeboaCatcher},
    request::DeboaRequest,
    response::DeboaResponse,
};

#[tokio::test]
async fn test_catcher_request() {
    let mut mock = MockDeboaCatcher::new();
    let mut request = DeboaRequest::get("https://httpbin.org/get").build().unwrap();
    mock.expect_on_request().returning(move |req| {
        req.headers_mut().insert(HeaderName::from_static("test"), "test".into());
        Ok(None)
    });

    let _ = mock.on_request(&mut request);
    assert_eq!(request.headers().get(&HeaderName::from_static("test")), Some(&"test".into()));
}

#[tokio::test]
async fn test_catcher_response() {
    let mut mock = MockDeboaCatcher::new();
    mock.expect_on_request().times(1).returning(move |_| Ok(None));
    mock.expect_on_response().times(1).returning(move |res| {
        res.set_raw_body(b"test");
    });

    let mut client = Deboa::builder().catch(mock).build();
    let response = DeboaRequest::get("https://httpbin.org/get").send_with(&mut client).await.unwrap();
    assert_eq!(response.raw_body(), b"test");
}

#[tokio::test]
async fn test_catcher_early_response() {
    let mut mock = MockDeboaCatcher::new();

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("test"), HeaderValue::from_static("test"));

    mock.expect_on_request()
        .times(1)
        .returning(move |_| Ok(Some(DeboaResponse::new(StatusCode::OK, headers.clone(), b"test"))));

    mock.expect_on_response().times(1).return_const(());

    let mut client = Deboa::builder().catch(mock).build();
    let response = DeboaRequest::get("https://httpbin.org/get").send_with(&mut client).await.unwrap();

    assert_eq!(response.headers().get("test").unwrap(), "test");
}
