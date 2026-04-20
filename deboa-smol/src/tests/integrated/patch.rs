use crate::{
    tests::{
        helpers::{client_with_cert, start_mock_server},
        TestResult,
    },
    Client,
};

use deboa::{request::DeboaRequest, HttpClient};
use easyhttpmock_vetis_smol::mock::{MethodExt, Mock, StatusCodeExt};
use http::{Method, StatusCode};

use macro_rules_attribute::apply;
use smol_macros::test;

//
// PATCH
//

async fn do_patch() -> TestResult<()> {
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
