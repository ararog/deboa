use deboa::TestResult;

#[compio::test]
async fn test_from_str_body() -> TestResult<()> {
    deboa_test_utils::base::request::test_from_str_body().await
}

#[compio::test]
async fn test_set_text_body() -> TestResult<()> {
    deboa_test_utils::base::request::test_set_text_body().await
}

#[compio::test]
async fn test_raw_body() -> TestResult<()> {
    deboa_test_utils::base::request::test_raw_body().await
}
