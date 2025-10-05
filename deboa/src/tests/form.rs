use crate::{errors::DeboaError, form::{DeboaForm, EncodedForm}};

#[test]
fn test_form() -> Result<(), DeboaError> {
    let form = EncodedForm::builder()
        .field("name", "deboa")
        .field("version", "0.0.1")
        .build();

    assert_eq!(form, "name=deboa&version=0.0.1");

    Ok(())
}