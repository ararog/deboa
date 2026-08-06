use deboa::TestResult;

#[tokio::test]
async fn test_shl() -> TestResult<()> {
    let client = deboa_tokio::Client::default();
    deboa_test_utils::base::client::test_shl(&client).await
}
