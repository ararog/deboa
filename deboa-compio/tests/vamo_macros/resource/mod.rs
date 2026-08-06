#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server, protocol_version};
use deboa::TestResult;
use deboa_compio::Client;
use easyhttpmock_vetis_compio::{vetis_adapter::VetisAdapter, EasyHttpMock};
use rstest::*;

#[rstest]
#[compio::test]
async fn test_post_resource(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    deboa_test_utils::vamo_macros::resource::test_post_resource(
        create_client,
        &mut create_server.await,
        protocol_version,
    )
    .await
}
