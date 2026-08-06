use deboa::TestResult;
use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]
async fn test_raw_body() -> TestResult<()> {
    deboa_test_utils::base::response::test_raw_body().await
}

#[apply(test!)]
async fn test_text_body() -> TestResult<()> {
    deboa_test_utils::base::response::test_text_body().await
}

#[apply(test!)]
async fn test_to_file() -> TestResult<()> {
    deboa_test_utils::base::response::test_to_file().await
}
