#![allow(unused_variables)]
use crate::common::helpers::{create_client, create_server};
use deboa::TestResult;
use deboa_compio::Client;
use easyhttpmock_vetis_compio::{vetis_adapter::VetisAdapter, EasyHttpMock};
use rstest::*;

#[rstest]
#[compio::test]
async fn test_delete(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
) -> TestResult<()> {
    deboa_test_utils::base::delete::test_delete(&create_client, &mut create_server.await).await
}
