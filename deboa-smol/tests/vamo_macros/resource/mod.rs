use crate::common::helpers::{create_client, create_server, protocol_version};
use deboa::TestResult;
use deboa_smol::Client;
use easyhttpmock_vetis_smol::{vetis_adapter::VetisAdapter, EasyHttpMock};
use macro_rules_attribute::apply;
use rstest::*;
use smol_macros::test;

#[rstest]
#[test_attr(apply(test))]
async fn test_post_resource(
    create_client: Client,
    #[future] _create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo_macros::resource::test_post_resource(
        create_client,
        &mut _create_server.await,
        protocol_version,
    )
    .await
}
