#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server, protocol_version};
#[cfg(feature = "rust-tls")]
use deboa::cert::IdentityExt as _;
#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
use deboa::cert::{CertificateExt as _, ContentEncoding};
use deboa::TestResult;
use deboa_compio::{
    cert::{DeboaCertificate, DeboaIdentity},
    Client,
};
use easyhttpmock_vetis_compio::{vetis_adapter::VetisAdapter, EasyHttpMock};
use rstest::*;

#[rstest]
#[compio::test]
async fn test_get_http(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::base::get::test_get_http(
        &create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

#[rstest]
#[compio::test]
async fn test_get_http_skip_verification(
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(
            deboa_test_utils::common::helpers::CA_CERT,
            ContentEncoding::DER,
        ))
        .skip_cert_verification(true)
        .build();

    deboa_test_utils::base::get::test_skip_cert_verification(
        &client,
        &mut create_server.await,
        protocol_version,
        true,
    )
    .await
}

#[rstest]
#[compio::test]
async fn test_get_http_verify(
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    let client = Client::builder()
        .skip_cert_verification(false)
        .build();

    deboa_test_utils::base::get::test_skip_cert_verification(
        &client,
        &mut create_server.await,
        protocol_version,
        false,
    )
    .await
}

#[cfg(feature = "rust-tls")]
#[rstest]
#[compio::test]
async fn test_get_http_mutual_authentication(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    let identity = DeboaIdentity::from_pkcs8(
        deboa_test_utils::common::helpers::CLIENT_CERT,
        deboa_test_utils::common::helpers::CLIENT_KEY,
        ContentEncoding::DER,
    );

    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(
            deboa_test_utils::common::helpers::CA_CERT,
            ContentEncoding::DER,
        ))
        .identity(identity)
        .build();

    deboa_test_utils::base::get::test_get_http_mutual_authentication(
        &client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

#[cfg(feature = "native-tls")]
#[rstest]
#[compio::test]
async fn test_get_http_mutual_authentication_with_password(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    let identity = DeboaIdentity::from_pkcs12(
        deboa_test_utils::common::helpers::CLIENT_P12,
        Some("test".to_string()),
    );

    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(
            deboa_test_utils::common::helpers::CA_CERT,
            ContentEncoding::DER,
        ))
        .identity(identity)
        .build();

    deboa_test_utils::base::get::test_get_http_mutual_authentication(
        &client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

#[rstest]
#[compio::test]
async fn test_get_not_found(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::base::get::test_get_not_found(
        &create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

#[rstest]
#[compio::test]
async fn test_get_invalid_server(create_client: Client) -> TestResult<()> {
    deboa_test_utils::base::get::test_get_invalid_server(&create_client).await
}

#[rstest]
#[compio::test]
async fn test_get_by_query(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::base::get::test_get_by_query(
        &create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

/*
async fn do_get_by_query_with_retries() -> Result<()> {
    let mut server = start_mock_server(|_req| async move {
        Ok(make_response(StatusCode::BAD_GATEWAY, "pong"))
    })
    .await;

    let client = create_client();

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
#[rstest]
#[tokio::test]
async fn test_get_by_query_with_retries() -> TestResult<()> {
    do_get_by_query_with_retries().await
}

#[cfg(feature = "smol-rt")]
#[rstest]
#[compio::test]
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
#[rstest]
#[tokio::test]
async fn test_get_with_redirect() -> TestResult<()> {
    do_get_with_redirect().await
}

#[cfg(feature = "smol-rt")]
#[rstest]
#[compio::test]
async fn test_get_with_redirect() {
    let _ = do_get_with_redirect().await;
}
*/

#[rstest]
#[compio::test]
async fn test_try_into(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::base::get::test_try_into(&create_client, &mut create_server.await).await
}

/*
#[rstest]
#[compio::test]
async fn test_fetch_from_str(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::base::get::test_fetch_from_str(&create_client, &mut create_server.await).await
}
*/
