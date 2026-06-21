use std::error::Error;

use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use vamo::Vamo;
use vamo_macros::bora;

use crate::common::helpers::{create_client, start_mock_server};

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
        "PATCH"
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

    let mut vamo = Vamo::new(server.base_url())?;
    vamo.client(client);

    let mut post_service = PostService::new(vamo);

    post_service
        .patch_post(1, Post { title: "title".to_string(), body: "body".to_string(), user_id: 1 })
        .await?;

    server
        .assert()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_patch_by_id() -> Result<(), Box<dyn Error>> {
    do_patch_by_id().await
}
