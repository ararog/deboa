use crate::{
    form::{DeboaForm, EncodedForm},
    Result,
};

#[test]
fn test_form() -> Result<()> {
    let form = EncodedForm::builder()
        .field("name", "deboa")
        .field("version", "0.0.1")
        .build();

    assert_eq!(form, "name=deboa&version=0.0.1");

    Ok(())
}
