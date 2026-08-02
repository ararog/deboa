use crate::common::helpers::{
    create_server, default_protocol_version, CA_CERT, SKIP_CERT_VERIFICATION,
};
use deboa::{
    cert::{CertificateExt as _, ContentEncoding},
    TestResult,
};
use deboa_compio::{cert::DeboaCertificate, Client};
use easyhttpmock_vetis_compio::{
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

#[compio::test]
async fn test_put_by_id() -> TestResult<()> {
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
    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.base_url())?;
    vamo.version(default_protocol_version());
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
