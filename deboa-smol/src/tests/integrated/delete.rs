use crate::tests::{
    helpers::{client_with_cert, start_mock_server},
    TestResult,
};

use deboa::request::DeboaRequest;
use http::StatusCode;

use macro_rules_attribute::apply;
use smol_macros::test;

//
// DELETE
//

async fn do_delete() -> TestResult<()> {
    let mut server = start_mock_server(|when| async move {
        Ok(when
            .method(String::from("DELETE"))
            .path(String::from("/posts/1"))
            .then()
            .with_status(StatusCode::OK)
            .with_body(String::from("")))
    })
    .await;

    let client = client_with_cert();

    let response = DeboaRequest::delete(server.url("/posts/1"))?
        .send_with(&client)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server
        .assert()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_delete() -> TestResult<()> {
    do_delete().await
}
