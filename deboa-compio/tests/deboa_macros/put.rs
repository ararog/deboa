#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server};
use deboa::TestResult;
use deboa_compio::Client;
use easyhttpmock_vetis_compio::{vetis_adapter::VetisAdapter, EasyHttpMock};
use rstest::*;

#[rstest]
#[compio::test]
async fn test_only_put_minimal(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server: EasyHttpMock<VetisAdapter> = create_server.await;
    deboa_test_utils::deboa_macros::put::test_only_put_minimal(&create_client, &mut server).await
}

#[rstest]
#[compio::test]
async fn test_only_put_minimal_headers(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server: EasyHttpMock<VetisAdapter> = create_server.await;
    deboa_test_utils::deboa_macros::put::test_only_put_minimal_headers(&create_client, &mut server)
        .await
}

#[rstest]
#[compio::test]
async fn test_put(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server: EasyHttpMock<VetisAdapter> = create_server.await;
    deboa_test_utils::deboa_macros::put::test_put(&create_client, &mut server).await
}

#[rstest]
#[compio::test]
async fn test_put_with_headers(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server: EasyHttpMock<VetisAdapter> = create_server.await;
    deboa_test_utils::deboa_macros::put::test_put_with_headers(&create_client, &mut server).await
}

#[rstest]
#[compio::test]
async fn test_put_with_json_body_request(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server: EasyHttpMock<VetisAdapter> = create_server.await;
    deboa_test_utils::deboa_macros::put::test_put_with_json_body_request(
        &create_client,
        &mut server,
    )
    .await
}

#[rstest]
#[compio::test]
async fn test_put_with_json_body_no_headers(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server: EasyHttpMock<VetisAdapter> = create_server.await;
    deboa_test_utils::deboa_macros::put::test_put_with_json_body_no_headers(
        &create_client,
        &mut server,
    )
    .await
}

#[rstest]
#[compio::test]
async fn test_put_with_json_body_response(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server: EasyHttpMock<VetisAdapter> = create_server.await;
    deboa_test_utils::deboa_macros::put::test_put_with_json_body_response(
        &create_client,
        &mut server,
    )
    .await
}
