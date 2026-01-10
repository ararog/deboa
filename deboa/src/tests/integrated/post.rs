use crate::{
    cert::Certificate,
    form::{DeboaForm, EncodedForm, MultiPartForm},
    request::DeboaRequest,
    tests::SKIP_CERT_VERIFICATION,
    Client, Result,
};

#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "smol-rt", any(feature = "http1", feature = "http2")))]
use deboa_tests::server::tcp::smol::HttpServer;

#[cfg(all(feature = "tokio-rt", feature = "http3-tokio"))]
use deboa_tests::server::udp::tokio::HttpServer;

use deboa_tests::utils::{make_response, tls_server_config, CA_CERT};
use http::{header, StatusCode};

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// POST
//

async fn do_post() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "POST" && req.uri().path() == "/posts" {
                Ok(make_response(StatusCode::CREATED, b"{\n  \"id\": 101\n}"))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let client: Client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let request = DeboaRequest::post(server.url("/posts"))?
        .text("{ \"title\": \"foo\", \"body\": \"bar\", \"userId\": 1 }")
        .build()?;

    let mut response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        response
            .raw_body()
            .await,
        b"{\n  \"id\": 101\n}",
    );

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_post() -> Result<()> {
    do_post().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_post() -> Result<()> {
    do_post().await
}

/*
async fn do_post_encoded_form() -> Result<()> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/posts")
            .header(
                header::CONTENT_TYPE.as_str(),
                mime::APPLICATION_WWW_FORM_URLENCODED.to_string(),
            )
            .body("name=deboa&version=0.0.1");
        then.status::<u16>(StatusCode::CREATED.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let client = Client::default();

    let mut form = EncodedForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let request = DeboaRequest::post(
        format!("{}/posts", TEST_HOST).as_str(),
    )?
    .form(form.into())
    .build()?;

    let mut response = client
        .execute(request)
        .await?;

    http_mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        response
            .raw_body()
            .await,
        b"ping"
    );

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_post_encoded_form() -> Result<()> {
    do_post_encoded_form().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_post_encoded_form() -> Result<()> {
    do_post_encoded_form().await
}


async fn do_post_multipart_form() -> Result<()> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/posts")
            .header_prefix(header::CONTENT_TYPE.as_str(), mime::MULTIPART_FORM_DATA.to_string());
        then.status::<u16>(StatusCode::CREATED.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let client = Client::default();

    let mut form = MultiPartForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let request = DeboaRequest::post(
        format!("{}/posts", TEST_HOST).as_str(),
    )?
    .header(header::CONTENT_TYPE, mime::MULTIPART_FORM_DATA.essence_str())
    .form(form.into())
    .build()?;

    let mut response = client
        .execute(request)
        .await?;

    http_mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        response
            .raw_body()
            .await,
        b"ping"
    );

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_post_multipart_form() -> Result<()> {
    do_post_multipart_form().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_post_multipart_form() -> Result<()> {
    do_post_multipart_form().await
}
*/
