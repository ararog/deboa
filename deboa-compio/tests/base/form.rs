use deboa::TestResult;

#[test]
fn test_encoded_form() -> TestResult<()> {
    deboa_test_utils::base::form::test_encoded_form()
}

#[test]
fn test_multipart_form() -> TestResult<()> {
    deboa_test_utils::base::form::test_multipart_form()
}

#[compio::test]
async fn test_multipart_validate_form() -> TestResult<()> {
    deboa_test_utils::base::form::test_multipart_validate_form().await
}

#[compio::test]
async fn test_multipart_validate_form_file() -> TestResult<()> {
    deboa_test_utils::base::form::test_multipart_validate_form_file().await
}
