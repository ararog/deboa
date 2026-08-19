#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server, protocol_version};
#[cfg(feature = "rust-tls")]
use deboa::cert::IdentityExt as _;
#[cfg(feature = "native-tls")]
use deboa::cert::IdentityNativeExt as _;
#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
use deboa::cert::{CertificateExt as _, ContentEncoding};
use deboa::TestResult;
use deboa_tokio::{
    cert::{DeboaCertificate, DeboaIdentity},
    Client,
};
use easyhttpmock_vetis_tokio::{vetis_adapter::VetisAdapter, EasyHttpMock};
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_get_http(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo::test_get(create_client, &mut create_server.await, protocol_version)
        .await
}

#[rstest]
#[tokio::test]
async fn test_get_http_skip_verification(
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
#[tokio::test]
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
#[tokio::test]
async fn test_get_http_mutual_authentication(
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(
            deboa_test_utils::common::helpers::CA_CERT,
            ContentEncoding::DER,
        ))
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
#[tokio::test]
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
#[tokio::test]
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
#[tokio::test]
async fn test_get_invalid_server(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::base::get::test_get_invalid_server(&create_client).await
}

#[rstest]
#[tokio::test]
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

#[rstest]
#[tokio::test]
async fn test_try_into(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::base::get::test_try_into(&create_client, &mut create_server.await).await
}

/*
#[rstest]
#[tokio::test]
async fn test_fetch_from_str(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::base::get::test_fetch_from_str(&create_client, &mut create_server.await).await
}
*/
