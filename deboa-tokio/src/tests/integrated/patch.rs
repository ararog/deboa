use crate::{
    tests::{
        helpers::{create_client, start_mock_server},
        TestResult,
    },
    Client,
};

use deboa::{request::DeboaRequest, HttpClient};
use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::{Method, StatusCode};

//
// PATCH
//

#[tokio::test]
async fn test_patch() -> TestResult<()> {
    let mock = Mock::of(
        Method::PATCH
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"done"),
            ),
    );

    let mut server = start_mock_server(mock).await;

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
        .assert()
        .await?;

    Ok(())
}
