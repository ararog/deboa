use crate::common::helpers::{create_client, create_server, protocol_version};
use deboa::TestResult;
use deboa_smol::Client;
use easyhttpmock_vetis_smol::{vetis_adapter::VetisAdapter, EasyHttpMock};
use macro_rules_attribute::apply;
use rstest::*;
use smol_macros::test;

#[rstest]
#[test_attr(apply(test))]
async fn test_get_by_id(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo_macros::bora::get::test_get_by_id(
        create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

#[rstest]
#[test_attr(apply(test))]
async fn test_get_all(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo_macros::bora::get::test_get_all(
        create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

#[rstest]
#[test_attr(apply(test))]
async fn test_query_by_id(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo_macros::bora::get::test_query_by_id(
        create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

#[rstest]
#[test_attr(apply(test))]
async fn test_query_by_title(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo_macros::bora::get::test_query_by_title(
        create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}
