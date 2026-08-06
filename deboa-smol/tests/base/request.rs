use deboa::TestResult;
use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]
async fn test_from_str_body() -> TestResult<()> {
    deboa_test_utils::base::request::test_from_str_body().await
}

#[apply(test!)]
async fn test_set_text_body() -> TestResult<()> {
    deboa_test_utils::base::request::test_set_text_body().await
}

#[apply(test!)]
async fn test_raw_body() -> TestResult<()> {
    deboa_test_utils::base::request::test_raw_body().await
}
