use crate::{ForyRequestBuilder, ForyResponse};
use deboa::{Client, cert::Certificate, errors::DeboaError, request::post};
use deboa_tests::utils::{make_response, tls_server_config, CA_CERT};

#[cfg(all(feature = "_tokio-rt", any(feature = "_http1", feature = "_http2")))]
use deboa_tests::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "_smol-rt", any(feature = "_http1", feature = "_http2")))]
use deboa_tests::server::tcp::smol::HttpServer;

#[cfg(all(feature = "_tokio-rt", feature = "_http3"))]
use deboa_tests::server::udp::tokio::HttpServer;

use fory::{Fory, ForyObject};
use http::{header, Method, StatusCode};

pub(crate) const SKIP_CERT_VERIFICATION: bool =
    cfg!(any(feature = "_tokio-native-tls", feature = "_smol-native-tls"));

const FORY_PERSON: [u8; 15] = [2, 255, 143, 2, 30, 255, 34, 74, 111, 104, 110, 32, 68, 111, 101];

#[derive(ForyObject)]
struct Person {
    name: String,
    age: u8,
}

async fn do_fory_post_request() -> Result<(), DeboaError> {
    let method = Method::POST;
    let path = "/posts";
    let status = 200;

    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "POST" && req.uri().path() == "/posts" {
                Ok(make_response(StatusCode::OK, b"Hello World!"))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut fory = Fory::default();
    let result = fory.register::<Person>(1);
    assert!(result.is_ok());

    let person = Person { name: "John Doe".to_string(), age: 30 };

    let request = post(server.url(path))?.body_as_fory(&fory, person)?;

    let response: Person = request
        .send_with(client)
        .await?
        .body_as_fory(&fory)
        .await?;

    assert_eq!(response.name, "John Doe");
    assert_eq!(response.age, 30);

    server.stop().await;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_fory_post_request() -> Result<(), DeboaError> {
    do_fory_post_request().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_fory_post_request() -> Result<(), DeboaError> {
    do_fory_post_request().await
}
