use crate::common::helpers::{create_client, create_server, default_protocol_version};
use caramelo::{expect, matchers::eq};
use deboa::{request::DeboaRequest, HttpClient, TestResult};
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::{Method, StatusCode};

//
// PUT
//
#[tokio::test]
async fn test_put() -> TestResult<()> {
    let mock = Mock::of(
        given(method(Method::PUT).and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;

    let client = create_client();
    let request = DeboaRequest::put(server.url("/posts/1"))?
        .version(default_protocol_version())
        .text("ping")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));

    server
        .stop()
        .await?;

    Ok(())
}
