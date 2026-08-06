#![allow(unused_variables)]
use deboa::TestResult;

#[compio::test]
async fn test_set_msgpack() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::msgpack::test_set_msgpack().await
}

#[compio::test]
async fn test_msgpack_response() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::msgpack::test_msgpack_response().await
}
