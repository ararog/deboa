use caramelo::{expect, matchers::eq};
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    request::DeboaRequest,
    Client, HttpClient, InnerClient, TestResult,
};
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::{Method, StatusCode};

pub async fn test_patch<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone + 'static,
    C: Certificate + Send + Clone + 'static,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send + 'static,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method(Method::PATCH).and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"done"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let request = DeboaRequest::patch(server.url("/posts/1"))?
        .version(protocol_version)
        .text("text")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));
    assert_eq!(
        response
            .text()
            .await?,
        "done"
    );

    server
        .stop()
        .await?;

    Ok(())
}
