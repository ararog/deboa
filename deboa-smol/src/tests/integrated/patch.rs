use crate::{
    tests::{
        helpers::{client_with_cert, start_mock_server},
        TestResult,
    },
    Client,
};

use deboa::{request::DeboaRequest, HttpClient};
use http::StatusCode;

use macro_rules_attribute::apply;
use smol_macros::test;

//
// PATCH
//

async fn do_patch() -> TestResult<()> {
    let mut server = start_mock_server(|when| async move {
        Ok(when
            .method(String::from("PATCH"))
            .path(String::from("/posts/1"))
            .then()
            .with_status(StatusCode::OK)
            .with_body(String::from("done")))
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
        .assert()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_patch() -> TestResult<()> {
    do_patch().await
}
