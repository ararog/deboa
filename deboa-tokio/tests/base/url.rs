use deboa::TestResult;

#[test]
fn test_url() -> TestResult<()> {
    deboa_test_utils::base::url::test_url()
}

#[test]
fn test_url_invalid() -> TestResult<()> {
    deboa_test_utils::base::url::test_url_invalid()
}
