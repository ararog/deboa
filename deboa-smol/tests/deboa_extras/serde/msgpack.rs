use deboa::TestResult;
use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]
async fn test_set_msgpack() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::msgpack::test_set_msgpack().await
}

#[apply(test!)]
async fn test_msgpack_response() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::msgpack::test_msgpack_response().await
}
