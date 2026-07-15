use crate::{
    tests::helpers::{create_client, create_server},
    Client,
};
use deboa::{request::DeboaRequest, HttpClient};
use easyhttpmock_vetis_compio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::{Method, StatusCode};
//
// PATCH
//
#[compio::test]
async fn test_patch() -> Result<(), Box<dyn std::error::Error>> {
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
    let client: Client = create_client();

    let request = DeboaRequest::patch(server.url("/posts/1"))?
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
