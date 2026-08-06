use deboa::TestResult;
use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]

async fn test_set_flex() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::flex::test_set_flex().await
}

#[apply(test!)]
async fn test_response_flex() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::flex::test_response_flex().await
}
