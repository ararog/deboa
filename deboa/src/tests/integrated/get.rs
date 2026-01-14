use crate::{
    cert::Identity,
    errors::{ConnectionError, ResponseError},
    request::{FetchWith, IntoRequest},
    tests::helpers::client_with_cert,
};
#[cfg(test)]
use crate::{errors::DeboaError, request::DeboaRequest, response::DeboaResponse, Client, Result};

use deboa_tests::utils::{make_response, start_mock_server, CLIENT_CERT, CLIENT_KEY, CLIENT_P12};
#[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
use deboa_tests::utils::{CLIENT_CERT_PEM, CLIENT_KEY_PEM};

use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// GET
//

async fn do_get_http() -> Result<()> {
    let mut server = start_mock_server(|req| {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(make_response(StatusCode::OK, b"Hello World!"))
        } else {
            Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
        }
    })
    .await;

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

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_http() -> Result<()> {
    do_get_http().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_http() {
    let _ = do_get_http().await;
}

async fn skip_cert_verification_helper(skip: bool) -> Result<()> {
    let mut server = start_mock_server(|req| {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(make_response(StatusCode::OK, b"Hello World!"))
        } else {
            Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
        }
    })
    .await;

    let client = Client::builder()
        .skip_cert_verification(skip)
        .build();

    let request = DeboaRequest::get(server.url("/posts/1"))?.build()?;

    let response = client
        .execute(request)
        .await;

    if skip {
        #[cfg(any(feature = "http1", feature = "http2"))]
        {
            let response = response?;
            assert_eq!(response.status(), StatusCode::OK);
        }
        #[cfg(feature = "http3")]
        {
            let error = DeboaError::Connection(ConnectionError::Udp {
                host: "localhost".to_string(),
                message: "Could not connect to server: aborted by peer: the cryptographic handshake failed: error 120: peer doesn't support any known protocol".to_string(),
            });
            assert_eq!(response.unwrap_err(), error);
        }
    } else {
        #[cfg(all(
            any(feature = "http1", feature = "http2"),
            any(feature = "tokio-rust-tls", feature = "smol-rust-tls")
        ))]
        let error = DeboaError::Connection(ConnectionError::Tls {
            host: "localhost".to_string(),
            message: "Could not connect to server: invalid peer certificate: UnknownIssuer"
                .to_string(),
        });

        #[cfg(all(feature = "http3", any(feature = "tokio-rust-tls", feature = "smol-rust-tls")))]
        let error = DeboaError::Connection(ConnectionError::Udp {
            host: "localhost".to_string(),
            message: "Could not connect to server: the cryptographic handshake failed: error 48: invalid peer certificate: UnknownIssuer".to_string(),
        });

        #[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
        let error = DeboaError::Connection(ConnectionError::Tls {
            host: "localhost".to_string(),
            message: "Could not connect to server: error:0A000086:SSL routines:tls_post_process_server_certificate:certificate verify failed:../ssl/statem/statem_clnt.c:1889: (unable to get local issuer certificate)".to_string(),
        });
        assert_eq!(response.unwrap_err(), error);
    }

    server.stop().await;

    Ok(())
}

async fn do_get_http_skip_verification() -> Result<()> {
    skip_cert_verification_helper(true).await
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_http_skip_verification() -> Result<()> {
    do_get_http_skip_verification().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_http_skip_verification() {
    let _ = do_get_http_skip_verification().await;
}

async fn test_get_http_verify() -> Result<()> {
    skip_cert_verification_helper(false).await
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn do_get_http_verify() -> Result<()> {
    test_get_http_verify().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn do_get_http_verify() {
    let _ = test_get_http_verify().await;
}

async fn do_get_http_mutual_authentication() -> Result<()> {
    let mut server = start_mock_server(|req| {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(make_response(StatusCode::OK, b"Hello World!"))
        } else {
            Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
        }
    })
    .await;

    #[cfg(any(feature = "tokio-rust-tls", feature = "smol-rust-tls"))]
    let identity = Identity::from_pkcs8(CLIENT_CERT, CLIENT_KEY);

    #[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
    let identity = Identity::from_pkcs8(CLIENT_CERT_PEM, CLIENT_KEY_PEM);

    let client = Client::builder()
        .identity(identity)
        .build();

    let request = DeboaRequest::get(server.url("/posts/1"))?.build()?;

    let response = client
        .execute(request)
        .await;

    #[cfg(all(
        any(feature = "http1", feature = "http2"),
        any(feature = "tokio-rust-tls", feature = "smol-rust-tls")
    ))]
    let error = DeboaError::Connection(ConnectionError::Tls {
        host: "localhost".to_string(),
        message: "Could not connect to server: invalid peer certificate: UnknownIssuer".to_string(),
    });

    #[cfg(all(feature = "http3", any(feature = "tokio-rust-tls", feature = "smol-rust-tls")))]
    let error = DeboaError::Connection(ConnectionError::Udp {
        host: "localhost".to_string(),
        message: "Could not connect to server: the cryptographic handshake failed: error 48: invalid peer certificate: UnknownIssuer".to_string(),
    });

    #[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
    let error = DeboaError::Connection(ConnectionError::Tls {
        host: "localhost".to_string(),
        message: "Could not connect to server: error:0A000086:SSL routines:tls_post_process_server_certificate:certificate verify failed:../ssl/statem/statem_clnt.c:1889: (unable to get local issuer certificate)".to_string(),
    });
    assert_eq!(response.unwrap_err(), error);

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_http_mutual_authentication() -> Result<()> {
    do_get_http_mutual_authentication().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_http_mutual_authentication() -> Result<()> {
    do_get_http_mutual_authentication().await?;
    Ok(())
}

#[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
async fn do_get_http_mutual_authentication_with_password() -> Result<()> {
    let mut server = start_mock_server(|req| {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(make_response(StatusCode::OK, b"Hello World!"))
        } else {
            Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
        }
    })
    .await;

    let identity = Identity::from_pkcs12(CLIENT_P12, Some("test".to_string()));

    let client = Client::builder()
        .identity(identity)
        .build();

    let request = DeboaRequest::get(server.url("/posts/1"))?.build()?;

    let response = client
        .execute(request)
        .await;

    #[cfg(all(
        any(feature = "http1", feature = "http2"),
        any(feature = "tokio-rust-tls", feature = "smol-rust-tls")
    ))]
    let error = DeboaError::Connection(ConnectionError::Tls {
        host: "localhost".to_string(),
        message: "Could not connect to server: invalid peer certificate: UnknownIssuer".to_string(),
    });

    #[cfg(all(feature = "http3", any(feature = "tokio-rust-tls", feature = "smol-rust-tls")))]
    let error = DeboaError::Connection(ConnectionError::Udp {
        host: "localhost".to_string(),
        message: "Could not connect to server: the cryptographic handshake failed: error 48: invalid peer certificate: UnknownIssuer".to_string(),
    });

    #[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
    let error = DeboaError::Connection(ConnectionError::Tls {
        host: "localhost".to_string(),
        message: "Could not connect to server: error:0A000086:SSL routines:tls_post_process_server_certificate:certificate verify failed:../ssl/statem/statem_clnt.c:1889: (unable to get local issuer certificate)".to_string(),
    });
    assert_eq!(response.unwrap_err(), error);

    server.stop().await;

    Ok(())
}

#[cfg(all(feature = "tokio-rt", any(feature = "tokio-native-tls", feature = "smol-native-tls")))]
#[tokio::test]
async fn test_get_http_mutual_authentication_with_password() -> Result<()> {
    do_get_http_mutual_authentication_with_password().await?;
    Ok(())
}

#[cfg(all(feature = "smol-rt", any(feature = "tokio-native-tls", feature = "smol-native-tls")))]
#[apply(test!)]
async fn test_get_http_mutual_authentication_with_password() {
    let _ = do_get_http_mutual_authentication_with_password().await;
}

//
// GET NOT FOUND
//

async fn do_get_not_found() -> Result<()> {
    let mut server =
        start_mock_server(|_| Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))).await;

    let client = client_with_cert();

    let response: Result<DeboaResponse> = DeboaRequest::get(server.url("/asasa/posts/1ddd"))?
        .send_with(client)
        .await;

    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err(),
        DeboaError::Response(ResponseError::Receive {
            status_code: StatusCode::NOT_FOUND,
            message: "Could not process request (404 Not Found): Not found".to_string()
        })
    );

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_not_found() -> Result<()> {
    do_get_not_found().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_not_found() {
    let _ = do_get_not_found().await;
}

//
// GET INVALID SERVER
//

async fn do_get_invalid_server() -> Result<()> {
    let api = Client::default();

    let request = DeboaRequest::get("https://invalid-server.com/posts")?
        .text("test")
        .build()?;

    let response: Result<DeboaResponse> = api
        .execute(request)
        .await;

    #[cfg(any(feature = "http1", feature = "http2"))]
    let error = DeboaError::Connection(ConnectionError::Tcp {
        host: "invalid-server.com".to_string(),
        #[cfg(target_os = "windows")]
        message: "Could not connect to server: No such host is known. (os error 11001)".to_string(),
        #[cfg(target_os = "linux")]
        message: "Could not connect to server: failed to lookup address information: Name or service not known".to_string(),
        #[cfg(target_os = "macos")]
        message:
            "Could not connect to server: failed to lookup address information: nodename nor servname provided, or not known"
                .to_string(),
    });

    #[cfg(feature = "http3")]
    let error = DeboaError::Connection(ConnectionError::Udp {
        host: "invalid-server.com".to_string(),
        message: "Could not connect to server: no record found for Query { name: Name(\"invalid-server.com.\"), query_type: AAAA, query_class: IN }"
            .to_string(),
    });

    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), error);

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_invalid_server() -> Result<()> {
    do_get_invalid_server().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_invalid_server() {
    let _ = do_get_invalid_server().await;
}

//
// GET BY QUERY
//

async fn do_get_by_query() -> Result<()> {
    let mut server = start_mock_server(|req| {
        if req.method() == "GET" && req.uri().path() == "/comments/1" {
            Ok(make_response(StatusCode::OK, b"My comment"))
        } else {
            Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
        }
    })
    .await;

    let client = client_with_cert();

    let response = DeboaRequest::get(server.url("/comments/1"))?
        .send_with(client)
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

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_by_query() -> Result<()> {
    do_get_by_query().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_by_query() {
    let _ = do_get_by_query().await;
}

async fn do_get_by_query_with_retries() -> Result<()> {
    let mut server =
        start_mock_server(|_req| Ok(make_response(StatusCode::BAD_GATEWAY, b"pong"))).await;

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
async fn test_get_by_query_with_retries() -> Result<()> {
    do_get_by_query_with_retries().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_by_query_with_retries() {
    let _ = do_get_by_query_with_retries().await;
}

/*
async fn do_get_with_redirect() -> Result<()> {
    let client = Client::default();

    let url = if cfg!(feature = "http3-tokio") {
        "https://tinyurl.com/bccjpjd7"
    } else {
        "https://tinyurl.com/bp6e548b"
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
async fn test_get_with_redirect() -> Result<()> {
    do_get_with_redirect().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_with_redirect() {
    let _ = do_get_with_redirect().await;
}
*/

async fn try_intro() -> Result<()> {
    let mut server = start_mock_server(|req| {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(make_response(StatusCode::OK, b""))
        } else {
            Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
        }
    })
    .await;

    let client = client_with_cert();
    let first_post = server.url("/posts/1");
    let response = client
        .execute(first_post.into_request()?)
        .await?;
    assert_eq!(response.status(), 200);

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_try_into() -> Result<()> {
    try_intro().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_try_into() -> Result<()> {
    try_intro().await
}

async fn fetch_from_str() -> Result<()> {
    let mut server = start_mock_server(|req| {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(make_response(StatusCode::OK, b""))
        } else {
            Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
        }
    })
    .await;

    let client = client_with_cert();
    let first_post = server.url("/posts/1");
    let response = first_post
        .fetch_with(&client)
        .await?;
    assert_eq!(response.status(), 200);

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_fetch_from_str() -> Result<()> {
    fetch_from_str().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_fetch_from_str() -> Result<()> {
    fetch_from_str().await
}
