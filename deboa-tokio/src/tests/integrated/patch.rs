use crate::{
    tests::{
        helpers::{client_with_cert, start_mock_server},
        TestResult,
    },
    Client,
};

use deboa::{request::DeboaRequest, HttpClient};
use easyhttpmock::mock_response;
use http::{header::HOST, StatusCode};

//
// PATCH
//

#[tokio::test]
async fn test_patch() -> TestResult<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "PATCH" && req.uri().path() == "/posts/1" {
            assert!(req
                .headers()
                .contains_key(HOST));
            Ok(mock_response(StatusCode::OK, "done"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client: Client = client_with_cert();

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
