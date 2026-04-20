use crate::tests::{
    helpers::{client_with_cert, start_mock_server},
    TestResult,
};

use deboa::request::DeboaRequest;
use easyhttpmock_vetis_smol::mock::{MethodExt, Mock, StatusCodeExt};
use http::{Method, StatusCode};

use macro_rules_attribute::apply;
use smol_macros::test;

//
// DELETE
//

async fn do_delete() -> TestResult<()> {
    let mock = Mock::of(
        Method::DELETE
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;

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
