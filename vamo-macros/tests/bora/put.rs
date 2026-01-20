use deboa::{
    cert::{Certificate, ContentEncoding},
    Client as DeboaClient, Result,
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

async fn do_put_by_id() -> Result<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "PUT" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, b""))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, b"Not found"))
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
        .update_post(1, Post { title: "title".to_string(), body: "body".to_string(), user_id: 1 })
        .await?;
    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_put_by_id() -> Result<()> {
    do_put_by_id().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_put_by_id() -> Result<()> {
    do_put_by_id().await
}
