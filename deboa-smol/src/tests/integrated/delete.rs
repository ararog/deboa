use crate::tests::{
    helpers::{create_client, create_server},
    TestResult,
};
use deboa::request::DeboaRequest;
use easyhttpmock_vetis_smol::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::{Method, StatusCode};
use macro_rules_attribute::apply;
use smol_macros::test;

//
// DELETE
//

#[apply(test!)]
async fn do_delete() -> TestResult<()> {
    let mock = Mock::of(
        given(method(Method::DELETE).and(path("/posts/1"))).will_return(
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

    let response = DeboaRequest::delete(server.url("/posts/1"))?
        .send_with(&client)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server
        .stop()
        .await?;

    Ok(())
}
