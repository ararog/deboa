use bytes::Bytes;
use caramelo::{
    expect,
    matchers::{eq, truthy},
};
use deboa::{
    form::{DeboaForm, MultiPartForm},
    Result,
};
use futures_util::stream::{once, Stream};
use std::{
    convert::Infallible,
    fs::{read, remove_file, write as write_file},
    path::Path,
};

#[tokio::test]
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
        expect(value).to_be(eq("deboa"));
    }

    if let Ok(Some(field)) = multer
        .next_field()
        .await
    {
        let value = field
            .text()
            .await
            .unwrap();
        expect(value).to_be(eq("0.0.1"));
    }

    Ok(())
}

#[tokio::test]
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
        expect(result).to_be(eq(b"teste".to_vec()));
    }

    expect(Path::exists(Path::new(input_file))).to_be(truthy());
    expect(Path::exists(Path::new(output_file))).to_be(truthy());

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

async fn get_stream(
    form: Bytes,
    boundary: &str,
) -> (impl Stream<Item = std::result::Result<Bytes, Infallible>>, &str) {
    let stream = once(async move { std::result::Result::<Bytes, Infallible>::Ok(form) });

    (stream, boundary)
}
