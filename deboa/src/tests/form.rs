use crate::{
    form::{DeboaForm, EncodedForm, MultiPartForm},
    Result,
};

#[test]
fn test_encoded_form() -> Result<()> {
    let form = EncodedForm::builder()
        .field("name", "deboa")
        .field("version", "0.0.1")
        .build();

    assert_eq!(form, "name=deboa&version=0.0.1");

    Ok(())
}

#[test]
fn test_multipart_form() -> Result<()> {
    let mut builder = MultiPartForm::builder();

    builder.field("name", "deboa");
    builder.field("version", "0.0.1");

    let form = builder.build();

    print!("{}", form);

    assert_eq!(form, format!("\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\ndeboa\r\n{}\r\nContent-Disposition: form-data; name=\"version\"\r\n\r\n0.0.1\r\n{}--", builder.boundary(), builder.boundary()));

    Ok(())
}
