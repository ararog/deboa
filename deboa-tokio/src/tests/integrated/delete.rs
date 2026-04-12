use crate::tests::{
    helpers::{client_with_cert, start_mock_server},
    TestResult,
};

use deboa::request::DeboaRequest;
use easyhttpmock::mock_response;
use http::StatusCode;

//
// DELETE
//
#[tokio::test]
async fn test_delete() -> TestResult<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "DELETE" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, ""))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = client_with_cert();

    let response = DeboaRequest::delete(server.url("/posts/1"))?
        .send_with(&client)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server
        .stop()
        .await?;

    Ok(())
}
