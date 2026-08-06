use deboa::TestResult;
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_set_xml() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::xml::test_set_xml().await
}

#[rstest]
#[tokio::test]
async fn test_xml_response() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::xml::test_xml_response().await
}
