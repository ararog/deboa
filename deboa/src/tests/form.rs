use std::convert::Infallible;
use std::fs::{read, remove_file, write as write_file};
use std::path::Path;

use bytes::Bytes;
use futures_util::stream::once;
use futures_util::stream::Stream;

use crate::{
    form::{DeboaForm, EncodedForm, MultiPartForm},
    Result,
};

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

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

async fn multipart_validate_form() -> Result<()> {
    let mut builder = MultiPartForm::builder();
    builder.field("name", "deboa");
    builder.field("version", "0.0.1");

    let my_boundary = builder
        .boundary()
        .to_string();

    let form = builder.build();

    let (stream, boundary) = get_stream(form, &my_boundary).await;

    let mut multer = multer::Multipart::new(stream, boundary);

    if let Ok(Some(field)) = multer
        .next_field()
        .await
    {
        let value = field
            .text()
            .await
            .unwrap();
        assert_eq!(value, "deboa");
    }

    if let Ok(Some(field)) = multer
        .next_field()
        .await
    {
        let value = field
            .text()
            .await
            .unwrap();
        assert_eq!(value, "0.0.1");
    }

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_multipart_validate_form() -> Result<()> {
    multipart_validate_form().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_multipart_validate_form() -> Result<()> {
    multipart_validate_form().await
}

async fn multipart_validate_form_file() -> Result<()> {
    let input_file = "input.txt";
    let output_file = "output.txt";

    let result = write_file(input_file, "teste");
    if let Err(e) = result {
        eprintln!("Error writing input file: {}", e);
    }

    let builder = MultiPartForm::builder().file("file", input_file);

    let my_boundary = builder
        .boundary()
        .to_string();

    let form = builder.build();

    let (stream, boundary) = get_stream(form, &my_boundary).await;

    let mut multer = multer::Multipart::new(stream, boundary);

    while let Ok(Some(field)) = multer
        .next_field()
        .await
    {
        let file = field.bytes().await;
        if let Ok(file) = file {
            if let Err(e) = write_file(output_file, file) {
                eprintln!("Error writing output file: {}", e);
            }
        }
    }

    let result = read(output_file);
    if let Ok(result) = result {
        assert_eq!(result, b"teste");
    }

    assert!(Path::exists(Path::new(input_file)));
    assert!(Path::exists(Path::new(output_file)));

    let result = remove_file(input_file);
    if let Err(e) = result {
        eprintln!("Error removing input file: {}", e);
    }

    let result = remove_file(output_file);
    if let Err(e) = result {
        eprintln!("Error removing output file: {}", e);
    }

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_multipart_validate_form_file() -> Result<()> {
    multipart_validate_form_file().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_multipart_validate_form_file() -> Result<()> {
    multipart_validate_form_file().await
}

async fn get_stream(
    form: Bytes,
    boundary: &str,
) -> (impl Stream<Item = std::result::Result<Bytes, Infallible>>, &str) {
    let stream = once(async move { std::result::Result::<Bytes, Infallible>::Ok(form) });

    (stream, boundary)
}
