use crate::common::helpers::{create_client, create_server, protocol_version};
use deboa::TestResult;
use deboa_tokio::Client;
use easyhttpmock_vetis_tokio::{vetis_adapter::VetisAdapter, EasyHttpMock};
use http::Version;
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_put(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: Version,
) -> TestResult<()> {
    let mut server = create_server.await;
    deboa_test_utils::base::post::test_post(&create_client, &mut server, protocol_version).await
}
