use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};

use deboa::{
    catcher::{DeboaCatcher, MockDeboaCatcher},
    cert::{Certificate, ContentEncoding},
    request::DeboaRequest,
    response::DeboaResponse,
    tests::SKIP_CERT_VERIFICATION,
    Client, Result,
};

use deboa_tests::{
    server::Server,
    utils::{make_response, test_url, tls_server_config, url_from_string, CA_CERT},
};

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

async fn catcher_request() -> Result<()> {
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

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_catcher_request() -> Result<()> {
    catcher_request().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_catcher_request() -> Result<()> {
    catcher_request().await
}

async fn catcher_response() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| async move {
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
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
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

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_catcher_response() -> Result<()> {
    catcher_response().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_catcher_response() -> Result<()> {
    catcher_response().await
}

async fn catcher_early_response() -> Result<()> {
    let mut catcher_mock = MockDeboaCatcher::new();

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("test"), HeaderValue::from_static("test"));

    let url = url_from_string(format!("{}posts/1", test_url(None)));

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
    let response = DeboaRequest::get(format!("{}posts/1", test_url(None)))
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

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_catcher_early_response() -> Result<()> {
    catcher_early_response().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_catcher_early_response() -> Result<()> {
    catcher_early_response().await
}
