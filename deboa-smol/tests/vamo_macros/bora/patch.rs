use crate::common::helpers::{create_client, create_server};
use easyhttpmock_vetis_smol::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::StatusCode;
use macro_rules_attribute::apply;
use serde::{Deserialize, Serialize};
use smol_macros::test;
use std::error::Error;
use vamo::Vamo;
use vamo_macros::bora;

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub body: String,
    #[serde(rename = "userId")]
    pub user_id: u32,
}

#[bora(
  api(
    patch(name="patch_post", path="/posts/<id:i32>", req_body=Post, format="json"),
  )
)]
pub struct PostService;

async fn do_patch_by_id() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("PATCH").and(path("/posts/1"))).will_return(
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
        .patch_post(1, Post { title: "title".to_string(), body: "body".to_string(), user_id: 1 })
        .await?;

    server
        .stop()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_patch_by_id() -> Result<(), Box<dyn Error>> {
    do_patch_by_id().await
}
