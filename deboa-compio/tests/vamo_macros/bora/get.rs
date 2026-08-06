#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server, protocol_version};
use deboa::TestResult;
use deboa_compio::Client;
use easyhttpmock_vetis_compio::{vetis_adapter::VetisAdapter, EasyHttpMock};
use rstest::*;

#[rstest]
#[compio::test]
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
#[compio::test]
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
#[compio::test]
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
#[compio::test]
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
