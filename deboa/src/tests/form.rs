use crate::{
    form::{DeboaForm, EncodedForm, MultiPartForm},
    Result,
};

#[test]
fn test_encoded_form() -> Result<()> {
    let mut form = EncodedForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let form = form.build();

    assert_eq!(form.to_vec(), b"name=deboa&version=0.0.1");

    Ok(())
}

#[test]
fn test_multipart_form() -> Result<()> {
    let mut builder = MultiPartForm::builder();

    builder.field("name", "deboa");
    builder.field("version", "0.0.1");

    let boundary = builder.boundary();

    let form = builder.build();

    assert_eq!(form.to_vec(), format!("--{}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\ndeboa\r\n--{}\r\nContent-Disposition: form-data; name=\"version\"\r\n\r\n0.0.1\r\n--{}--\r\n", boundary, boundary, boundary).as_bytes());

    Ok(())
}
