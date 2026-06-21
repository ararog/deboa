use crate::tests::{
    helpers::{create_client, start_mock_server},
    TestResult,
};

use deboa::{request::DeboaRequest, HttpClient};
use easyhttpmock_vetis_smol::mock::{MethodExt, Mock, StatusCodeExt};
use http::{Method, StatusCode};

use macro_rules_attribute::apply;
use smol_macros::test;

//
// PUT
//

async fn do_put() -> TestResult<()> {
    let mock = Mock::of(
        Method::PUT
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = create_client();

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

#[apply(test!)]
async fn test_put() -> TestResult<()> {
    do_put().await
}
