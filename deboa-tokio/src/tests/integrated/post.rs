use crate::{
    tests::{
        helpers::{create_client, create_server},
        TestResult,
    },
    Client,
};
use deboa::{
    form::{DeboaForm, EncodedForm, MultiPartForm},
    request::DeboaRequest,
    HttpClient,
};
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::{header::CONTENT_TYPE, Method, StatusCode};

//
// POST
//

#[tokio::test]
async fn test_post() -> TestResult<()> {
    let mock = Mock::of(
        given(method(Method::POST).and(path("/posts"))).will_return(
            StatusCode::CREATED
                .respond()
                .with_body(b"{\n  \"id\": 101\n}"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client: Client = create_client();

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
    let mock = Mock::of(
        given(method(Method::POST).and(path("/posts"))).will_return(
            StatusCode::CREATED
                .respond()
                .with_header(
                    CONTENT_TYPE.as_str(),
                    mime::APPLICATION_WWW_FORM_URLENCODED.essence_str(),
                )
                .with_body(b"ping"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client: Client = create_client();

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

    let mock = Mock::of(
        given(method(Method::POST).and(path("/posts"))).will_return(
            StatusCode::CREATED
                .respond()
                .with_header(CONTENT_TYPE.as_str(), mime::MULTIPART_FORM_DATA.essence_str())
                .with_body(b"ping"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client: Client = create_client();

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
