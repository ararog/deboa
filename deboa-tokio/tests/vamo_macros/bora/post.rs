use crate::common::helpers::{create_client, create_server};
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use vamo::Vamo;
use vamo_macros::bora;

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

#[tokio::test]
async fn do_post_by_id() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("POST").and(path("/posts"))).will_return(
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
        .create_post(Post {
            id: 1,
            title: "title".to_string(),
            body: "body".to_string(),
            user_id: 1,
        })
        .await?;

    server
        .stop()
        .await?;

    Ok(())
}
