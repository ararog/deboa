use std::error::Error;

use easyhttpmock_vetis_smol::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;
use macro_rules_attribute::apply;
use serde::{Deserialize, Serialize};
use smol_macros::test;
use vamo::Vamo;
use vamo_macros::bora;

use crate::common::helpers::{client_with_cert, start_mock_server};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
    #[serde(rename = "userId")]
    pub user_id: u32,
}

#[bora(
  api(
    post(name="create_post", path="/posts", req_body=Post, format="json"),
  )
)]
pub struct PostService;

async fn do_post_by_id() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "POST"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = client_with_cert();

    let mut vamo = Vamo::new(server.base_url())?;
    vamo.client(client);

    let mut post_service = PostService::new(vamo);

    post_service
        .create_post(Post {
            id: 1,
            title: "title".to_string(),
            body: "body".to_string(),
            user_id: 1,
        })
        .await?;

    server
        .assert()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_post_by_id() -> Result<(), Box<dyn Error>> {
    do_post_by_id().await
}
