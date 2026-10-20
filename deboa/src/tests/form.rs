use crate::{TestResult, form::EncodedForm, request::DeboaRequest};
use caramelo::{expect, matchers::eq};
use http::{Method, Uri};

#[test]
fn test_encoded_form() -> TestResult<()> {
    let form = EncodedForm::builder()
        .field("name", "deboa")
        .field("version", "0.0.1");

    let form = form.build();

    assert_eq!(form.to_vec(), b"name=deboa&version=0.0.1");

    Ok(())
}

#[tokio::test]
async fn test_try_into_request() -> TestResult<()> {
    let url = url::Url::parse("https://google.com")?;
    let form = EncodedForm::builder()
        .field("name", "deboa")
        .field("version", "0.0.1");

    let request: DeboaRequest = (url, Method::GET, form).try_into()?;

    expect(request.method()).to_be(eq(&Method::GET));
    expect(request.uri()).to_be(eq(&Uri::from_static("https://google.com")));

    Ok(())
}

#[test]
fn test_encoded_form_content_type() -> TestResult<()> {
    let form = EncodedForm::builder()
        .field("name", "deboa")
        .field("version", "0.0.1");

    assert_eq!(form.content_type(), "application/x-www-form-urlencoded");

    Ok(())
}
