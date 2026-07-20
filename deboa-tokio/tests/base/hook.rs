use crate::common::{
    helpers::{create_client, create_server},
    TestResult,
};
use deboa::{hook::hook_fn, request::DeboaRequest, response::DeboaResponse, HttpClient as _};
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt as _},
};
use http::StatusCode;

#[tokio::test]
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
    let client = create_client().hook(hook_fn(|request: DeboaRequest, next| async move {
        println!("{} {}", request.method(), request.url());
        next.call(request)
            .await
    }));

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
