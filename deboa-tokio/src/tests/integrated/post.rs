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
use easyhttpmock::mock_response;
use http::{header::CONTENT_TYPE, StatusCode};

//
// POST
//

#[tokio::test]
async fn test_post() -> TestResult<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "POST" && req.uri().path() == "/posts" {
            Ok(mock_response(StatusCode::CREATED, "{\n  \"id\": 101\n}"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
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
        .stop()
        .await?;

    Ok(())
}

async fn do_post_encoded_form() -> TestResult<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "POST" && req.uri().path() == "/posts" {
            if req
                .headers()
                .contains_key(CONTENT_TYPE)
            {
                let content_type = req
                    .headers()
                    .get(CONTENT_TYPE)
                    .unwrap();
                assert_eq!(
                    content_type
                        .to_str()
                        .unwrap(),
                    mime::APPLICATION_WWW_FORM_URLENCODED.to_string()
                );
            }
            // TODO: check body
            // name=deboa&version=0.0.1
            Ok(mock_response(StatusCode::CREATED, "ping"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
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
        .stop()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_post_encoded_form() -> TestResult<()> {
    do_post_encoded_form().await
}

#[tokio::test]
async fn test_post_multipart_form() -> TestResult<()> {
    let mut form = MultiPartForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let mut server = start_mock_server(|req| async move {
        if req.method() == "POST" && req.uri().path() == "/posts" {
            if req
                .headers()
                .contains_key(CONTENT_TYPE)
            {
                let content_type = req
                    .headers()
                    .get(CONTENT_TYPE)
                    .unwrap();

                assert!(content_type
                    .to_str()
                    .unwrap()
                    .contains("multipart/form-data; boundary="));
            }
            // TODO: check body
            // name=deboa&version=0.0.1
            Ok(mock_response(StatusCode::CREATED, "ping"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
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
        .stop()
        .await?;

    Ok(())
}
