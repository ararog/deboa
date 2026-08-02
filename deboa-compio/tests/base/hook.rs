use crate::common::helpers::{create_client, create_server};
use deboa::{request::DeboaRequest, response::DeboaResponse, HttpClient as _, TestResult};
use easyhttpmock_vetis_compio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt as _},
};
use http::StatusCode;
use tackle::Hook;

#[compio::test]
async fn test_hook() -> TestResult<()> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"Hello World!"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client().chain_fn(|request, next| async move {
        println!("Request: {:?}", request);
        next.call(request)
            .await
    });

    let request = DeboaRequest::get(server.url("/posts/1"))?.build()?;
    let response: DeboaResponse = client
        .execute(request)
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response
            .status()
            .as_u16(),
        StatusCode::OK.as_u16()
    );

    server
        .stop()
        .await?;

    Ok(())
}
