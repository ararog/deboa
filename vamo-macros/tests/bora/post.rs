use deboa::{
    cert::{Certificate, ContentEncoding},
    Client as DeboaClient,
};
use deboa_tests::{
    mock_response,
    utils::{start_mock_server, CA_CERT},
};

use http::StatusCode;
use serde::{Deserialize, Serialize};
use vamo::Vamo;
use vamo_macros::bora;

use crate::SKIP_CERT_VERIFICATION;

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

async fn do_post_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "POST" && req.uri().path() == "/posts" {
            Ok(mock_response(StatusCode::OK, ""))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = DeboaClient::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

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

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_post_by_id() -> Result<(), Box<dyn std::error::Error>> {
    do_post_by_id().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_post_by_id() -> Result<(), Box<dyn std::error::Error>> {
    do_post_by_id().await
}
