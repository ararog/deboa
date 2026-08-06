#![allow(unused_variables)]
use deboa::TestResult;

#[compio::test]
async fn test_set_json() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::json::test_set_json().await
}

#[compio::test]
async fn test_response_json() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::json::test_response_json().await
}
