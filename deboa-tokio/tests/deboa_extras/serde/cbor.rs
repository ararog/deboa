use deboa::TestResult;
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_set_cbor() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::cbor::test_set_cbor().await
}

#[rstest]
fn test_set_cbor_registers_headers() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::cbor::test_set_cbor_register_headers()
}

#[rstest]
#[tokio::test]
async fn test_response_cbor() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::cbor::test_response_cbor().await
}

#[rstest]
#[tokio::test]
async fn test_response_cbor_invalid_body() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::cbor::test_response_cbor_invalid_body().await
}
