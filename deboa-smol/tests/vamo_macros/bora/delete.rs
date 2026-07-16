use crate::common::{
    helpers::{create_client, create_server},
    TestResult,
};
use easyhttpmock_vetis_smol::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::test;
use vamo::Vamo;
use vamo_macros::bora;

#[bora(api(delete(name = "delete_post", path = "/posts/<id:i32>")))]
pub struct PostService;

#[apply(test!)]
async fn test_delete_by_id() -> TestResult<()> {
    let mock = Mock::of(
        given(method("DELETE").and(path("/posts/1"))).will_return(
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

    let mut vamo = Vamo::new(server.base_url())?;
    vamo.client(client);
    let mut post_service = PostService::new(vamo);
    post_service
        .delete_post(1)
        .await?;

    server
        .stop()
        .await?;

    Ok(())
}
