use std::convert::Infallible;

use bytes::Bytes;
use futures_util::stream::once;
use futures_util::stream::Stream;

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

    let boundary = builder.boundary();

    assert_eq!(form, format!("--{}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\ndeboa\r\n--{}\r\nContent-Disposition: form-data; name=\"version\"\r\n\r\n0.0.1\r\n--{}--\r\n", boundary, boundary, boundary));

    Ok(())
}

#[tokio::test]
async fn test_multipart_validate_form() -> Result<()> {
    let mut builder = MultiPartForm::builder();

    builder.field("name", "deboa");
    builder.field("version", "0.0.1");

    let form = builder.build();

    let my_boundary = builder.boundary().to_string();

    let (stream, boundary) = get_stream(form, &my_boundary).await;

    let mut multer = multer::Multipart::new(stream, boundary);

    while let Ok(Some(field)) = multer.next_field().await {
        let value = field.text().await.unwrap();
        println!("{}", value);
    }

    Ok(())
}

async fn get_stream(
    form: String,
    boundary: &str,
) -> (
    impl Stream<Item = std::result::Result<Bytes, Infallible>>,
    &str,
) {
    let stream =
        once(async move { std::result::Result::<Bytes, Infallible>::Ok(Bytes::from(form)) });

    (stream, boundary)
}
