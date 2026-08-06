#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server, protocol_version};
use deboa::TestResult;
use deboa_smol::Client;
use easyhttpmock_vetis_smol::{vetis_adapter::VetisAdapter, EasyHttpMock};
use http::Version;
use macro_rules_attribute::apply;
use rstest::*;
use smol_macros::test;

#[rstest]
#[test_attr(apply(test))]
async fn test_patch(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: Version,
) -> TestResult<()> {
    let mut server = create_server.await;
    deboa_test_utils::base::patch::test_patch(create_client, &mut server, protocol_version).await
}
