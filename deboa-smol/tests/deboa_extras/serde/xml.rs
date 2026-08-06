use deboa::TestResult;
use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]
async fn test_set_xml() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::xml::test_set_xml().await
}

#[apply(test!)]
async fn test_xml_response() -> TestResult<()> {
    deboa_test_utils::deboa_extras::serde::xml::test_xml_response().await
}
