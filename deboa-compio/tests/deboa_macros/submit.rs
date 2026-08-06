#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server};
use deboa::TestResult;
use deboa_compio::Client;
use easyhttpmock_vetis_compio::{vetis_adapter::VetisAdapter, EasyHttpMock};
use rstest::*;

#[rstest]
#[compio::test]
async fn test_submit_str_minimal(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server: EasyHttpMock<VetisAdapter> = create_server.await;
    deboa_test_utils::deboa_macros::delete::test_delete_with_headers(&create_client, &mut server)
        .await
}

#[rstest]
#[compio::test]
async fn test_submit_str_method(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server: EasyHttpMock<VetisAdapter> = create_server.await;
    deboa_test_utils::deboa_macros::submit::test_submit_str_method(&create_client, &mut server)
        .await
}
