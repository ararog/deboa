use deboa::{request::DeboaRequest, HttpClient};
use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::{Method, StatusCode};

use crate::tests::{
    helpers::{create_client, start_mock_server},
    TestResult,
};

//
// PUT
//
#[tokio::test]
async fn test_put() -> TestResult<()> {
    let mock = Mock::of(
        Method::PUT
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = create_client();

    let request = DeboaRequest::put(server.url("/posts/1"))?
        .text("ping")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server
        .assert()
        .await?;

    Ok(())
}
