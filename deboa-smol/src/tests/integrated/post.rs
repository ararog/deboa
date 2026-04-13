use crate::{
    tests::{
        helpers::{client_with_cert, start_mock_server},
        TestResult,
    },
    Client,
};

use deboa::{
    form::{DeboaForm, EncodedForm, MultiPartForm},
    request::DeboaRequest,
    HttpClient,
};
use http::{header::CONTENT_TYPE, StatusCode};

use macro_rules_attribute::apply;
use smol_macros::test;

//
// POST
//

async fn do_post() -> TestResult<()> {
    let mut server = start_mock_server(|when| async move {
        Ok(when
            .path(String::from("/posts"))
            .method(String::from("POST"))
            .then()
            .with_status(StatusCode::CREATED)
            .with_body(String::from("{\n  \"id\": 101\n}")))
    })
    .await;

    let client: Client = client_with_cert();

    let request = DeboaRequest::post(server.url("/posts"))?
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

    server
        .assert()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_post() -> TestResult<()> {
    do_post().await
}

async fn do_post_encoded_form() -> TestResult<()> {
    let mut server = start_mock_server(|when| async move {
        Ok(when
            .path(String::from("/posts"))
            .method(String::from("POST"))
            .then()
            .with_header(
                "CONTENT_TYPE".to_owned(),
                mime::APPLICATION_WWW_FORM_URLENCODED.to_string(),
            )
            .with_status(StatusCode::CREATED)
            .with_body(String::from("ping")))
    })
    .await;

    let client: Client = client_with_cert();

    let mut form = EncodedForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let request = DeboaRequest::post(server.url("/posts"))?
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

    server
        .assert()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_post_encoded_form() -> TestResult<()> {
    do_post_encoded_form().await
}

async fn do_post_multipart_form() -> TestResult<()> {
    let mut form = MultiPartForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let mut server = start_mock_server(|when| async move {
        Ok(when
            .path(String::from("/posts"))
            .method(String::from("POST"))
            .then()
            .with_header("CONTENT_TYPE".to_owned(), mime::MULTIPART_FORM_DATA.to_string())
            .with_status(StatusCode::CREATED)
            .with_body(String::from("ping")))
    })
    .await;

    let client: Client = client_with_cert();

    let request = DeboaRequest::post(server.url("/posts"))?
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

    server
        .assert()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_post_multipart_form() -> TestResult<()> {
    do_post_multipart_form().await
}
