#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server};
use deboa::TestResult;
use deboa_smol::Client;
use easyhttpmock_vetis_smol::{vetis_adapter::VetisAdapter, EasyHttpMock};
use macro_rules_attribute::apply;
use rstest::*;
use smol_macros::test;

#[rstest]
#[test_attr(apply(test))]
async fn test_submit_str_minimal(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server = create_server.await;
    deboa_test_utils::deboa_macros::submit::test_submit_str_minimal(&create_client, &mut server)
        .await
}

#[rstest]
#[test_attr(apply(test))]
async fn test_submit_str_method(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    let mut server = create_server.await;
    deboa_test_utils::deboa_macros::submit::test_submit_str_method(&create_client, &mut server)
        .await
}
