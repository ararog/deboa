use crate::common::helpers::{create_client, create_server};
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use vamo::Vamo;
use vamo_macros::bora;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub body: String,
    #[serde(rename = "userId")]
    pub user_id: u32,
}

#[bora(
  api(
    put(name="update_post", path="/posts/<id:i32>", req_body=Post, format="json"),
  )
)]
pub struct PostService;

async fn do_put_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let mock = Mock::of(
        given(method("PUT").and(path("/posts/1"))).will_return(
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
        .update_post(1, Post { title: "title".to_string(), body: "body".to_string(), user_id: 1 })
        .await?;

    server
        .stop()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_put_by_id() -> Result<(), Box<dyn std::error::Error>> {
    do_put_by_id().await
}
