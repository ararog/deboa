use deboa::TestResult;

#[compio::test]
async fn test_shl() -> TestResult<()> {
    let client = deboa_compio::Client::default();
    deboa_test_utils::base::client::test_shl(&client).await
}
