use deboa::TestResult;
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_set_json() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::json::test_set_json().await
}

#[rstest]
#[tokio::test]
async fn test_response_json() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::json::test_response_json().await
}
