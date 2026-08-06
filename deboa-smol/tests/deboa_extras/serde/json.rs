use deboa::TestResult;
use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]
async fn test_set_json() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::json::test_set_json().await
}

#[apply(test!)]
async fn test_response_json() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::json::test_response_json().await
}
