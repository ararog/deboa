use crate::tests::{
    helpers::{create_client, create_server},
    TestResult,
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
// PUT
//

async fn do_put() -> TestResult<()> {
    let mock = Mock::of(
        given(method(Method::PUT).and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

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

#[apply(test!)]
async fn test_put() -> TestResult<()> {
    do_put().await
}
