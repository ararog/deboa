use crate::common::helpers::{create_client, create_server};
use caramelo::{expect, matchers::eq};
use deboa::{request::DeboaRequest, TestResult};
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::{Method, StatusCode};

//
// DELETE
//
#[tokio::test]
async fn test_delete() -> TestResult<()> {
    let mock = Mock::of(
        given(method(Method::DELETE).and(path("/posts/1"))).will_return(
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
    let response = DeboaRequest::delete(server.url("/posts/1"))?
        .send_with(&client)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));

    server
        .stop()
        .await?;

    Ok(())
}
