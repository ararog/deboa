use deboa::{request::DeboaRequest, HttpClient};
use http::StatusCode;

use crate::tests::{
    helpers::{client_with_cert, start_mock_server},
    TestResult,
};

//
// PUT
//
#[tokio::test]
async fn test_put() -> TestResult<()> {
    let mut server = start_mock_server(|when| async move {
        Ok(when
            .path(String::from("/posts/1"))
            .method(String::from("PUT"))
            .then()
            .with_status(StatusCode::OK)
            .with_body(String::from("")))
    })
    .await;

    let client = client_with_cert();

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
