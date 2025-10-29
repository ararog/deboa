use crate::{
    form::{DeboaForm, EncodedForm, MultiPartForm},
    request::DeboaRequest,
    Deboa, Result,
};
use http::{header, StatusCode};
use httpmock::{Method::POST, MockServer};

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// POST
//

async fn do_post() -> Result<()> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(POST).path("/posts");
        then.status::<u16>(StatusCode::CREATED.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let mut client = Deboa::new();

    let request = DeboaRequest::post(server.url("/posts").as_str())?
        .text("ping")
        .build()?;

    let mut response = client.execute(request).await?;

    http_mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(response.raw_body().await, b"ping");

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_post() -> Result<()> {
    do_post().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_post() -> Result<()> {
    do_post().await
}

async fn do_post_encoded_form() -> Result<()> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/posts")
            .header(
                header::CONTENT_TYPE.as_str(),
                mime::APPLICATION_WWW_FORM_URLENCODED.to_string(),
            )
            .body("name=deboa&version=0.0.1");
        then.status::<u16>(StatusCode::CREATED.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let mut client = Deboa::new();

    let mut form = EncodedForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let request = DeboaRequest::post(server.url("/posts").as_str())?
        .form(form.into())
        .build()?;

    let mut response = client.execute(request).await?;

    http_mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(response.raw_body().await, b"ping");

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_post_encoded_form() -> Result<()> {
    do_post_encoded_form().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_post_encoded_form() -> Result<()> {
    do_post_encoded_form().await
}

async fn do_post_multipart_form() -> Result<()> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(POST).path("/posts").header_prefix(
            header::CONTENT_TYPE.as_str(),
            mime::MULTIPART_FORM_DATA.to_string(),
        );
        then.status::<u16>(StatusCode::CREATED.into())
            .header(header::CONTENT_TYPE.as_str(), mime::TEXT_PLAIN.to_string())
            .body("ping");
    });

    let mut client = Deboa::new();

    let mut form = MultiPartForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let request = DeboaRequest::post(server.url("/posts").as_str())?
        .header(
            header::CONTENT_TYPE,
            mime::MULTIPART_FORM_DATA.essence_str(),
        )
        .form(form.into())
        .build()?;

    let mut response = client.execute(request).await?;

    http_mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(response.raw_body().await, b"ping");

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_post_multipart_form() -> Result<()> {
    do_post_multipart_form().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_post_multipart_form() -> Result<()> {
    do_post_multipart_form().await
}
