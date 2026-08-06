use deboa::TestResult;
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_set_yaml() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::yaml::test_set_yaml().await
}

#[rstest]
#[tokio::test]
async fn test_response_yaml() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::yaml::test_response_yaml().await
}
