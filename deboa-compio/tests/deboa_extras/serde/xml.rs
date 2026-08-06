#![allow(unused_variables)]
use deboa::TestResult;

#[compio::test]
async fn test_set_xml() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::xml::test_set_xml().await
}

#[compio::test]
async fn test_xml_response() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::xml::test_xml_response().await
}
