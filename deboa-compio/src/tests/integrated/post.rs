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
        b"ping"
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
        b"ping"
    );

    Ok(())
}
