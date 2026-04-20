#[cfg(feature = "http2")]
use crate::HttpVersion;
use crate::{
    cert::{ContentEncoding, Identity},
    tests::{
        helpers::{client_with_cert, start_mock_server, CA_CERT},
        TestResult,
    },
    Client,
};

#[cfg(feature = "rust-tls")]
use crate::tests::helpers::{CLIENT_CERT, CLIENT_KEY};
#[cfg(feature = "native-tls")]
use crate::tests::helpers::{CLIENT_CERT_PEM, CLIENT_KEY_PEM, CLIENT_P12};
use deboa::{
    errors::{ConnectionError, DeboaError, ResponseError},
    request::{DeboaRequest, FetchWith, IntoRequest},
    response::DeboaResponse,
    HttpClient,
};

use easyhttpmock_vetis_smol::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;

use macro_rules_attribute::apply;
use smol_macros::test;

//
// GET
//

#[apply(test!)]
async fn test_get_http() -> TestResult<()> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"Hello World!"),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = client_with_cert();

    let request = DeboaRequest::get(server.url("/posts/1"))?.build()?;

    let response: DeboaResponse = client
        .execute(request)
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response
            .status()
            .as_u16(),
        StatusCode::OK.as_u16()
    );

    server
        .assert()
        .await?;

    Ok(())
}

async fn skip_cert_verification_helper(skip: bool) -> TestResult<()> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"Hello World!"),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = Client::builder()
        .skip_cert_verification(skip)
        .build();

    let request = DeboaRequest::get(server.url("/posts/1"))?.build()?;

    let response = client
        .execute(request)
        .await;

    if skip {
        match client.protocol() {
            #[cfg(feature = "http1")]
            HttpVersion::Http1 => {
                let response = response?;
                assert_eq!(response.status(), StatusCode::OK);
            }
            #[cfg(feature = "http2")]
            HttpVersion::Http2 => {
                let response = response?;
                assert_eq!(response.status(), StatusCode::OK);
            }
            #[cfg(feature = "http3")]
            HttpVersion::Http3 => {
                let error = DeboaError::Connection(ConnectionError::Udp {
                    host: "localhost".to_string(),
                    message: "Could not connect to server: aborted by peer: the cryptographic handshake failed: error 120: peer doesn't support any known protocol".to_string(),
                });
                assert_eq!(response.unwrap_err(), error);
            }
        }
    } else {
        #[cfg(feature = "rust-tls")]
        {
            let error = match client.protocol() {
                #[cfg(feature = "http1")]
                HttpVersion::Http1 => {
                    DeboaError::Connection(ConnectionError::Tls {
                        host: "localhost".to_string(),
                        message:
                            "Could not connect to server: invalid peer certificate: UnknownIssuer"
                                .to_string(),
                    })
                }
                #[cfg(feature = "http2")]
                HttpVersion::Http2 => {
                    DeboaError::Connection(ConnectionError::Tls {
                        host: "localhost".to_string(),
                        message:
                            "Could not connect to server: invalid peer certificate: UnknownIssuer"
                                .to_string(),
                    })
                }
                #[cfg(feature = "http3")]
                HttpVersion::Http3 => {
                    DeboaError::Connection(ConnectionError::Tls {
                        host: "localhost".to_string(),
                        message: "Could not connect to server: the cryptographic handshake failed: error 48: invalid peer certificate: UnknownIssuer".to_string(),
                    })
                }
            };
            assert_eq!(response.unwrap_err(), error);
        }

        #[cfg(feature = "native-tls")]
        {
            let error = DeboaError::Connection(ConnectionError::Tls {
                host: "localhost".to_string(),
                message: "Could not connect to server: error:0A000086:SSL routines:tls_post_process_server_certificate:certificate verify failed:../ssl/statem/statem_clnt.c:1889: (self-signed certificate in certificate chain)".to_string(),
            });
            assert_eq!(response.unwrap_err(), error);
        }
    }

    server
        .assert()
        .await?;

    Ok(())
}

async fn do_get_http_skip_verification() -> TestResult<()> {
    skip_cert_verification_helper(true).await
}

#[apply(test!)]
async fn test_get_http_skip_verification() -> TestResult<()> {
    do_get_http_skip_verification().await
}

async fn do_get_http_verify() -> TestResult<()> {
    skip_cert_verification_helper(false).await
}

#[apply(test!)]
async fn test_get_http_verify() -> TestResult<()> {
    do_get_http_verify().await
}

async fn do_get_http_mutual_authentication() -> TestResult<()> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"Hello World!"),
            ),
    );

    let mut server = start_mock_server(mock).await;

    #[cfg(feature = "rust-tls")]
    let identity = Identity::from_pkcs8(CLIENT_CERT, CLIENT_KEY, ContentEncoding::DER);

    #[cfg(feature = "native-tls")]
    let identity = Identity::from_pkcs8(CLIENT_CERT_PEM, CLIENT_KEY_PEM, ContentEncoding::PEM);

    let client = Client::builder()
        .certificate(crate::cert::Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .identity(identity)
        .build();

    let request = DeboaRequest::get(server.url("/posts/1"))?.build()?;

    let response = client
        .execute(request)
        .await;

    assert_eq!(response?.status(), StatusCode::OK);

    server
        .assert()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_get_http_mutual_authentication() -> TestResult<()> {
    do_get_http_mutual_authentication().await
}

#[cfg(feature = "native-tls")]
async fn do_get_http_mutual_authentication_with_password() -> TestResult<()> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"Hello World!"),
            ),
    );
    let mut server = start_mock_server(mock).await;

    let identity = Identity::from_pkcs12(CLIENT_P12, Some("test".to_string()));

    let client = Client::builder()
        .certificate(crate::cert::Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .identity(identity)
        .build();

    let request = DeboaRequest::get(server.url("/posts/1"))?.build()?;

    let response = client
        .execute(request)
        .await;

    assert_eq!(response?.status(), StatusCode::OK);

    server
        .assert()
        .await?;

    Ok(())
}

#[cfg(feature = "native-tls")]
#[apply(test!)]
async fn test_get_http_mutual_authentication_with_password() -> TestResult<()> {
    do_get_http_mutual_authentication_with_password().await
}

//
// GET NOT FOUND
//

#[apply(test!)]
async fn test_get_not_found() -> TestResult<()> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::NOT_FOUND
                    .respond()
                    .with_body(b"Not found"),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = client_with_cert();

    let response: crate::Result<DeboaResponse> =
        DeboaRequest::get(server.url("/asasa/posts/1ddd"))?
            .send_with(&client)
            .await;

    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err(),
        DeboaError::Response(ResponseError::Receive {
            status_code: StatusCode::NOT_FOUND,
            message: "Could not process request (404 Not Found): Not found".to_string()
        })
    );

    server
        .assert()
        .await?;

    Ok(())
}

//
// GET INVALID SERVER
//

#[apply(test!)]
async fn test_get_invalid_server() -> TestResult<()> {
    let client = Client::default();

    let request = DeboaRequest::get("https://invalid-server.com/posts")?
        .text("test")
        .build()?;

    let response: crate::Result<DeboaResponse> = client
        .execute(request)
        .await;

    let error = match client.protocol() {
        #[cfg(feature = "http1")]
        HttpVersion::Http1 => DeboaError::Connection(ConnectionError::Tcp {
            host: "invalid-server.com".to_string(),
            message: "Could not resolve host: invalid-server.com.".to_string(),
        }),
        #[cfg(feature = "http2")]
        HttpVersion::Http2 => DeboaError::Connection(ConnectionError::Tcp {
            host: "invalid-server.com".to_string(),
            message: "Could not resolve host: invalid-server.com.".to_string(),
        }),
        #[cfg(feature = "http3")]
        HttpVersion::Http3 => DeboaError::Connection(ConnectionError::Udp {
            host: "invalid-server.com".to_string(),
            message: "Could not resolve host: invalid-server.com.".to_string(),
        }),
    };

    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), error);

    Ok(())
}

//
// GET BY QUERY
//

#[apply(test!)]
async fn test_get_by_query() -> TestResult<()> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/comments/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"My comment"),
            ),
    );

    let mut server = start_mock_server(mock).await;
    let client = client_with_cert();

    let response = DeboaRequest::get(server.url("/comments/1"))?
        .send_with(&client)
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response
            .status()
            .as_u16(),
        StatusCode::OK.as_u16()
    );

    let comments = response
        .text()
        .await;

    assert!(comments.is_ok());
    assert_eq!(comments.unwrap(), "My comment");

    server
        .assert()
        .await?;

    Ok(())
}

/*
async fn do_get_by_query_with_retries() -> Result<()> {
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

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_by_query_with_retries() -> TestResult<()> {
    do_get_by_query_with_retries().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_by_query_with_retries() {
    let _ = do_get_by_query_with_retries().await;
}
*/

/*
async fn do_get_with_redirect() -> Result<()> {
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

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_with_redirect() -> TestResult<()> {
    do_get_with_redirect().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_with_redirect() {
    let _ = do_get_with_redirect().await;
}
*/

#[apply(test!)]
async fn test_try_into() -> TestResult<()> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = client_with_cert();
    let first_post = server.url("/posts/1");
    let response = client
        .execute(first_post.into_request()?)
        .await?;
    assert_eq!(response.status(), 200);

    server
        .assert()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_fetch_from_str() -> TestResult<()> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = client_with_cert();
    let first_post = server.url("/posts/1");
    let response = first_post
        .fetch_with(client)
        .await?;
    assert_eq!(response.status(), 200);

    server
        .assert()
        .await?;

    Ok(())
}
