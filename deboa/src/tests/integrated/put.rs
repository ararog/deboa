use crate::{
    request::DeboaRequest,
    tests::{helpers::client_with_cert, TestResult},
};

use deboa_tests::{mock_response, utils::start_mock_server};
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// PUT
//

async fn do_put() -> TestResult<()> {
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

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_put() -> TestResult<()> {
    do_put().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_put() -> TestResult<()> {
    do_put().await
}
