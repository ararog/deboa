use crate::common::helpers::{create_client, create_server, default_protocol_version};
#[cfg(feature = "rust-tls")]
use crate::common::helpers::{CA_CERT, CLIENT_CERT, CLIENT_KEY};
#[cfg(feature = "native-tls")]
use crate::common::helpers::{CA_CERT, CLIENT_CERT_PEM, CLIENT_KEY_PEM, CLIENT_P12};
use caramelo::{
    expect,
    matchers::{eq, err, truthy},
};
#[cfg(feature = "native-tls")]
use deboa::cert::IdentityNativeExt as _;
#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
use deboa::cert::{CertificateExt as _, ContentEncoding, IdentityExt as _};
use deboa::{
    request::{DeboaRequest, FetchWith, IntoRequest},
    response::DeboaResponse,
    HttpClient, TestResult,
};
use deboa_tokio::{
    cert::{DeboaCertificate, DeboaIdentity},
    Client,
};
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::{StatusCode, Version};

//
// GET
//
#[tokio::test]
async fn test_get_http() -> TestResult<()> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"Hello World!"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let request = DeboaRequest::get(server.url("/posts/1"))?
        .version(default_protocol_version())
        .build()?;
    let response: DeboaResponse = client
        .execute(request)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));

    server
        .stop()
        .await?;

    Ok(())
}

async fn skip_cert_verification_helper(skip: bool) -> TestResult<()> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"Hello World!"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = Client::builder()
        .skip_cert_verification(skip)
        .build();

    let request = DeboaRequest::get(server.url("/posts/1"))?
        .version(default_protocol_version())
        .build()?;
    let http_version = request.version();
    let response = client
        .execute(request)
        .await;

    if skip {
        match http_version {
            #[cfg(feature = "http1")]
            Version::HTTP_11 => {
                let response = response?;
                expect(response.status()).to_be(eq(StatusCode::OK));
            }
            #[cfg(feature = "http2")]
            Version::HTTP_2 => {
                let response = response?;
                expect(response.status()).to_be(eq(StatusCode::OK));
            }
            #[cfg(feature = "http3")]
            Version::HTTP_3 => {
                let error = DeboaError::Connection(ConnectionError::Udp {
                    host: "localhost".to_string(),
                    message: "Could not connect to server: aborted by peer: the cryptographic handshake failed: error 120: peer doesn't support any known protocol".to_string(),
                });
                expect(response.unwrap_err()).to_be(eq(error));
            }
            _ => unreachable!(),
        }
    } else {
        expect(response).to_be(err());
    }

    server
        .stop()
        .await?;

    Ok(())
}

async fn do_get_http_skip_verification() -> TestResult<()> {
    skip_cert_verification_helper(true).await
}

#[tokio::test]
async fn test_get_http_skip_verification() -> TestResult<()> {
    do_get_http_skip_verification().await?;
    Ok(())
}
async fn do_get_http_verify() -> TestResult<()> {
    skip_cert_verification_helper(false).await
}

#[tokio::test]
async fn test_get_http_verify() -> TestResult<()> {
    do_get_http_verify().await
}

#[tokio::test]
async fn test_get_http_mutual_authentication() -> TestResult<()> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"Hello World!"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;

    #[cfg(feature = "rust-tls")]
    let identity = DeboaIdentity::from_pkcs8(CLIENT_CERT, CLIENT_KEY, ContentEncoding::DER);

    #[cfg(feature = "native-tls")]
    let identity = DeboaIdentity::from_pkcs8(CLIENT_CERT_PEM, CLIENT_KEY_PEM, ContentEncoding::PEM);

    #[cfg(any(feature = "rust-tls", feature = "native-tls"))]
    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .identity(identity)
        .build();

    #[cfg(not(any(feature = "rust-tls", feature = "native-tls")))]
    let client = Client::default();

    let request = DeboaRequest::get(server.url("/posts/1"))?
        .version(default_protocol_version())
        .build()?;
    let response = client
        .execute(request)
        .await;

    expect(response?.status()).to_be(eq(StatusCode::OK));

    server
        .stop()
        .await?;

    Ok(())
}

#[cfg(feature = "native-tls")]
#[tokio::test]
async fn test_get_http_mutual_authentication_with_password() -> TestResult<()> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"Hello World!"),
        ),
    );
    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;

    let identity = DeboaIdentity::from_pkcs12(CLIENT_P12, Some("test".to_string()));
    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .identity(identity)
        .build();

    let request = DeboaRequest::get(server.url("/posts/1"))?
        .version(default_protocol_version())
        .build()?;

    let response = client
        .execute(request)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));
    expect(
        response
            .text()
            .await?,
    )
    .to_be(eq("Hello World!"));

    server
        .stop()
        .await?;

    Ok(())
}

//
// GET NOT FOUND
//
#[tokio::test]
async fn test_get_not_found() -> TestResult<()> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::NOT_FOUND
                .respond()
                .with_body(b"Not found"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let response = DeboaRequest::get(server.url("/asasa/posts/1ddd"))?
        .version(default_protocol_version())
        .send_with(&client)
        .await?;
    expect(response.status()).to_be(eq(StatusCode::NOT_FOUND));

    server
        .stop()
        .await?;

    Ok(())
}

//
// GET INVALID SERVER
//
#[tokio::test]
async fn test_get_invalid_server() -> TestResult<()> {
    let client = Client::default();
    let request = DeboaRequest::get("https://invalid-server.com/posts")?
        .text("test")
        .build()?;
    let response: deboa::Result<DeboaResponse> = client
        .execute(request)
        .await;

    expect(response.is_err()).to_be(truthy());
    expect(response).to_be(err());

    Ok(())
}

//
// GET BY QUERY
//
#[tokio::test]
async fn test_get_by_query() -> TestResult<()> {
    let mock = Mock::of(
        given(method("GET").and(path("/comments/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"My comment"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;

    let client = create_client();
    let response = DeboaRequest::get(server.url("/comments/1"))?
        .version(default_protocol_version())
        .send_with(&client)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));

    let comments = response
        .text()
        .await;

    expect(comments.is_ok()).to_be(truthy());
    expect(comments.unwrap()).to_be(eq("My comment"));

    server
        .stop()
        .await?;

    Ok(())
}

/*
#[tokio::test]
async fn test_get_by_query_with_retries() {
    let mut server = start_mock_server(|_req| async move {
        Ok(make_response(StatusCode::BAD_GATEWAY, "pong"))
    })
    .await;

    let client = client_with_cert();

    let response = DeboaRequest::get(server.url("/comments/1"))?
        .retries(2)
        .send_with(client)
        .await;

    if let Err(err) = response {
        assert_eq!(
            err,
            DeboaError::Response(ResponseError::Receive {
                status_code: StatusCode::BAD_GATEWAY,
                message: "Could not process request (502 Bad Gateway): pong".to_string(),
            }),
        );
    }

    server.stop().await;

    Ok(())
}
*/

/*
#[tokio::test]
async fn test_get_with_redirect() -> TestResult<()> {
    let client = Client::default();

    let url = if cfg!(feature = "http3-tokio") {
        "https://tinyurl.com/bccjpjd7"
    } else {
        "https://tinyurl.com/bp6e548"
    };

    let response = DeboaRequest::get(url)?
        .send_with(client)
        .await?;

    let server = if cfg!(feature = "http3-tokio") { "facebook.com" } else { "github.com" };

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("server")
            .unwrap()
            .to_str()
            .unwrap(),
        server
    );

    Ok(())
}
*/

#[tokio::test]
async fn test_try_into() -> TestResult<()> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let first_post = server.url("/posts/1");
    let response = client
        .execute(first_post.into_request()?)
        .await?;
    expect(response.status()).to_be(eq(200));

    server
        .stop()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_fetch_from_str() -> TestResult<()> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;

    let client = create_client();
    let first_post = server.url("/posts/1");
    let response = first_post
        .fetch_with(client)
        .await?;
    expect(response.status()).to_be(eq(200));

    server
        .stop()
        .await?;

    Ok(())
}
