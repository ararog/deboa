use deboa::{
    request::DeboaRequest,
};
use http::StatusCode;
//
// PUT
//
#[compio::test]
async fn test_put() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "PUT" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, ""))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
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
        .stop()
        .await?;

    Ok(())
}
