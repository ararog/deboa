use crate::common::helpers::{create_client, create_server, default_protocol_version};
use deboa::{request::DeboaRequest, HttpClient, TestResult};
use easyhttpmock_vetis_compio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::{Method, StatusCode};
//
// PATCH
//
#[compio::test]
async fn test_patch() -> TestResult<()> {
    let mock = Mock::of(
        given(method(Method::PATCH).and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"done"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let request = DeboaRequest::patch(server.url("/posts/1"))?
        .version(default_protocol_version())
        .version(default_protocol_version())
        .text("text")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
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
