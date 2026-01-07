use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};

use crate::{
    catcher::{DeboaCatcher, MockDeboaCatcher},
    request::DeboaRequest,
    response::DeboaResponse,
    Client,
};

use deboa_tests::utils::{url_from_string, JSONPLACEHOLDER};

#[tokio::test]
async fn test_catcher_request() {
    let mut mock = MockDeboaCatcher::new();
    let mut request = DeboaRequest::get("https://httpbin.org/get")
        .unwrap()
        .build()
        .unwrap();
    mock.expect_on_request()
        .returning(move |req| {
            req.headers_mut()
                .insert("test", HeaderValue::from_str("test").unwrap());
            Ok(None)
        });

    let _ = mock
        .on_request(&mut request)
        .await;
    assert_eq!(
        request
            .headers()
            .get("test"),
        Some(&HeaderValue::from_str("test").unwrap())
    );
}

#[tokio::test]
async fn test_catcher_response() {
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

    let client = Client::builder()
        .catch(catcher_mock)
        .build();
    let mut response = DeboaRequest::get(format!("{}/posts/1", JSONPLACEHOLDER))
        .unwrap()
        .send_with(client)
        .await
        .unwrap();

    assert_eq!(
        response
            .raw_body()
            .await,
        b"test"
    );
}

#[tokio::test]
async fn test_catcher_early_response() {
    let mut catcher_mock = MockDeboaCatcher::new();

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("test"), HeaderValue::from_static("test"));

    let url = url_from_string(format!("{}/posts/1", JSONPLACEHOLDER));

    catcher_mock
        .expect_on_request()
        .times(1)
        .returning(move |_| {
            Ok(Some(
                DeboaResponse::builder(url.clone())
                    .status(StatusCode::OK)
                    .headers(headers.clone())
                    .body(&b"test"[..])
                    .build(),
            ))
        });

    catcher_mock
        .expect_on_response()
        .times(1)
        .return_const(Ok(()));

    let client = Client::builder()
        .catch(catcher_mock)
        .build();
    let response = DeboaRequest::get(format!("{}/posts/1", JSONPLACEHOLDER))
        .unwrap()
        .send_with(client)
        .await
        .unwrap();

    assert_eq!(
        response
            .headers()
            .get("test")
            .unwrap(),
        "test"
    );
}
