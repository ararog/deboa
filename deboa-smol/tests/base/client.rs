use deboa::TestResult;
use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]
async fn test_shl() -> TestResult<()> {
    let client = deboa_smol::Client::default();
    deboa_test_utils::base::client::test_shl(&client).await
}
