#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server, protocol_version};
use deboa::TestResult;
use deboa_compio::Client;
use easyhttpmock_vetis_compio::{vetis_adapter::VetisAdapter, EasyHttpMock};
use rstest::*;

#[rstest]
#[compio::test]
async fn test_get(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo::test_get(create_client, &mut create_server.await, protocol_version)
        .await
}

#[rstest]
#[compio::test]
async fn test_put(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo::test_put(create_client, &mut create_server.await, protocol_version)
        .await
}

#[rstest]
#[compio::test]
async fn test_post(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo::test_post(create_client, &mut create_server.await, protocol_version)
        .await
}

#[rstest]
#[compio::test]
async fn test_patch(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo::test_patch(create_client, &mut create_server.await, protocol_version)
        .await
}

#[rstest]
#[compio::test]
async fn test_delete(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo::test_delete(create_client, &mut create_server.await, protocol_version)
        .await
}

#[rstest]
#[compio::test]
async fn test_post_resource(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo::test_post_resource(
        create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

#[rstest]
#[compio::test]
async fn test_put_resource(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo::test_put_resource(
        create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

#[rstest]
#[compio::test]
async fn test_patch_resource(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo::test_patch_resource(
        create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}

#[rstest]
#[compio::test]
async fn test_remove_resource(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo::test_remove_resource(
        create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}
