use deboa::TestResult;
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_set_flex() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::flex::test_set_flex().await
}

#[rstest]
#[tokio::test]
async fn test_response_flex() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::flex::test_response_flex().await
}
