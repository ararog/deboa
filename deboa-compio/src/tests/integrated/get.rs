use crate::Client;
use deboa::{
    cert::ContentEncoding,
    errors::{ConnectionError, DeboaError, ResponseError},
    request::DeboaRequest,
    response::DeboaResponse,
};
use http::StatusCode;

//
// GET
//

#[compio::test]
async fn test_get_http() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, "Hello World!"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
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

    server
        .stop()
        .await?;

    Ok(())
}

async fn skip_cert_verification_helper(skip: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, "Hello World!"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
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
            message: "Could not connect to server: error:0A000086:SSL routines:tls_post_process_server_certificate:certificate verify failed:../ssl/statem/statem_clnt.c:1889: (self-signed certificate in certificate chain)".to_string(),
        });
        assert_eq!(response.unwrap_err(), error);
    }

    server
        .stop()
        .await?;

    Ok(())
}

async fn do_get_http_skip_verification() -> Result<(), Box<dyn std::error::Error>> {
    skip_cert_verification_helper(true).await
}

#[compio::test]
async fn test_get_http_skip_verification() -> Result<(), Box<dyn std::error::Error>> {
    do_get_http_skip_verification().await?;
    Ok(())
}

async fn do_get_http_verify() -> Result<(), Box<dyn std::error::Error>> {
    skip_cert_verification_helper(false).await
}

#[compio::test]
async fn test_get_http_verify() -> Result<(), Box<dyn std::error::Error>> {
    do_get_http_verify().await
}

#[compio::test]
async fn test_get_http_mutual_authentication() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, "Hello World!"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    #[cfg(any(feature = "tokio-rust-tls", feature = "smol-rust-tls"))]
    let identity = Identity::from_pkcs8(CLIENT_CERT, CLIENT_KEY, ContentEncoding::DER);

    #[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
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
        .stop()
        .await?;

    Ok(())
}

#[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
#[compio::test]
async fn test_get_http_mutual_authentication_with_password(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, "Hello World!"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

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
        .stop()
        .await?;

    Ok(())
}

//
// GET NOT FOUND
//

#[compio::test]
async fn test_get_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut server =
        start_mock_server(|_| async move { Ok(mock_response(StatusCode::NOT_FOUND, "Not found")) })
            .await;

    let client = client_with_cert();

    let response: crate::Result<DeboaResponse> =
        DeboaRequest::get(server.url("/asasa/posts/1ddd"))?
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

    server
        .stop()
        .await?;

    Ok(())
}

//
// GET INVALID SERVER
//

#[compio::test]
async fn test_get_invalid_server() -> Result<(), Box<dyn std::error::Error>> {
    let api = Client::default();

    let request = DeboaRequest::get("https://invalid-server.com/posts")?
        .text("test")
        .build()?;

    let response: crate::Result<DeboaResponse> = api
        .execute(request)
        .await;

    let error = DeboaError::Connection(ConnectionError::Tcp {
        host: "invalid-server.com".to_string(),
        message: "Could not resolve host: invalid-server.com.".to_string(),
    });

    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), error);

    Ok(())
}

//
// GET BY QUERY
//

#[compio::test]
async fn test_get_by_query() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET" && req.uri().path() == "/comments/1" {
            Ok(mock_response(StatusCode::OK, "My comment"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
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

    server
        .stop()
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
async fn test_get_by_query_with_retries() -> Result<(), Box<dyn std::error::Error>> {
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
async fn test_get_with_redirect() -> Result<(), Box<dyn std::error::Error>> {
    do_get_with_redirect().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_with_redirect() {
    let _ = do_get_with_redirect().await;
}
*/

#[compio::test]
async fn test_try_into() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, ""))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = client_with_cert();
    let first_post = server.url("/posts/1");
    let response = client
        .execute(first_post.into_request()?)
        .await?;
    assert_eq!(response.status(), 200);

    server
        .stop()
        .await?;

    Ok(())
}

#[compio::test]
async fn test_fetch_from_str() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, ""))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = client_with_cert();
    let first_post = server.url("/posts/1");
    let response = first_post
        .fetch_with(&client)
        .await?;
    assert_eq!(response.status(), 200);

    server
        .stop()
        .await?;

    Ok(())
}
