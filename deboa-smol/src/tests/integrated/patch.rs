use crate::{
    tests::{
        helpers::{create_client, create_server},
        TestResult,
    },
    Client,
};

use deboa::{request::DeboaRequest, HttpClient};
use easyhttpmock_vetis_smol::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::{Method, StatusCode};

use macro_rules_attribute::apply;
use smol_macros::test;

//
// PATCH
//

async fn do_patch() -> TestResult<()> {
    let mock = Mock::of(
        given(method(Method::PATCH).and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"done"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client: Client = create_client();

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

#[apply(test!)]
async fn test_patch() -> TestResult<()> {
    do_patch().await
}
