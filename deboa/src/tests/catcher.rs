use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};

use crate::{
    catcher::{DeboaCatcher, MockDeboaCatcher},
    cert::Certificate,
    request::DeboaRequest,
    response::DeboaResponse,
    Client,
};

use deboa_tests::utils::{make_response, tls_server_config, url_from_string, TEST_HOST};

#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::smol::HttpServer;

#[cfg(all(feature = "tokio-rt", feature = "http3-tokio"))]
use deboa_tests::server::udp::tokio::HttpServer;

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
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "GET" && req.uri().path() == "/posts/1" {
                Ok(make_response(StatusCode::OK, b"Hello World!"))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

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
        .certificate(Certificate::new("certs/ca.cert".into()))
        .catch(catcher_mock)
        .build();
    let mut response = DeboaRequest::get(server.url("/posts/1"))
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

    server.stop().await;
}

#[tokio::test]
async fn test_catcher_early_response() {
    let mut catcher_mock = MockDeboaCatcher::new();

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("test"), HeaderValue::from_static("test"));

    let url = url_from_string(format!("{}/posts/1", TEST_HOST));

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
    let response = DeboaRequest::get(format!("{}/posts/1", TEST_HOST))
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
