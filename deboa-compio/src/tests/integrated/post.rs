use crate::Client;
use deboa::{
    form::{DeboaForm, EncodedForm, MultiPartForm},
    request::DeboaRequest,
    HttpClient,
};
use http::StatusCode;
//
// POST
//

#[compio::test]
async fn test_post() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::default();

    let request = DeboaRequest::post("https://jsonplaceholder.typicode.com/posts")?
        .text("{ \"title\": \"foo\", \"body\": \"bar\", \"userId\": 1 }")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        response
            .bytes()
            .await,
        b"{\n  \"id\": 101\n}",
    );

    Ok(())
}

#[compio::test]
async fn test_post_encoded_form() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::default();

    let mut form = EncodedForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let request = DeboaRequest::post("https://jsonplaceholder.typicode.com/posts")?
        .form(form.into())
        .build()?;

    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        response
            .bytes()
            .await,
        [
            123, 10, 32, 32, 34, 110, 97, 109, 101, 34, 58, 32, 34, 100, 101, 98, 111, 97, 34, 44,
            10, 32, 32, 34, 118, 101, 114, 115, 105, 111, 110, 34, 58, 32, 34, 48, 46, 48, 46, 49,
            34, 44, 10, 32, 32, 34, 105, 100, 34, 58, 32, 49, 48, 49, 10, 125
        ]
    );

    Ok(())
}

#[compio::test]
async fn test_post_multipart_form() -> Result<(), Box<dyn std::error::Error>> {
    let mut form = MultiPartForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let client: Client = Client::default();

    let request = DeboaRequest::post("https://jsonplaceholder.typicode.com/posts")?
        .form(form.into())
        .build()?;

    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        response
            .bytes()
            .await,
        [123, 10, 32, 32, 34, 105, 100, 34, 58, 32, 49, 48, 49, 10, 125]
    );

    Ok(())
}
