use caramelo::{expect, matchers::eq};
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    form::{DeboaForm, EncodedForm, MultiPartForm},
    request::DeboaRequest,
    Client, HttpClient, InnerClient, TestResult,
};
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::{header::CONTENT_TYPE, Method, StatusCode};

pub async fn test_post<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method(Method::POST).and(path("/posts"))).will_return(
            StatusCode::CREATED
                .respond()
                .with_body(b"{\n  \"id\": 101\n}"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let request = DeboaRequest::post(server.url("/posts"))?
        .version(protocol_version)
        .text("{ \"title\": \"foo\", \"body\": \"bar\", \"userId\": 1 }")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::CREATED));
    assert_eq!(
        response
            .bytes()
            .await?,
        b"{\n  \"id\": 101\n}",
    );

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_post_encoded_form<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
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

    server
        .register_mock(mock)
        .await?;

    let mut form = EncodedForm::builder();
    form.field("name", "deboa");
    form.field("version", "0.0.1");

    let request = DeboaRequest::post(server.url("/posts"))?
        .version(protocol_version)
        .form(form.into())?
        .build()?;

    let response = client
        .execute(request)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::CREATED));
    assert_eq!(
        response
            .bytes()
            .await?,
        b"ping"
    );

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_post_multipart_form<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
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

    server
        .register_mock(mock)
        .await?;

    let request = DeboaRequest::post(server.url("/posts"))?
        .version(protocol_version)
        .form(form.into())?
        .build()?;

    let response = client
        .execute(request)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::CREATED));
    assert_eq!(
        response
            .bytes()
            .await?,
        b"ping"
    );

    server
        .stop()
        .await?;

    Ok(())
}
