use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    url::IntoUrl as _,
    Client, InnerClient, TestResult,
};
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
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

pub async fn test_patch_by_id<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send + Default,
    R: DnsResolver + Send + Default,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PATCH").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let mut vamo = Vamo::from_client(client)?
        .base_url(
            server
                .base_url()
                .into_url()?,
        )
        .version(protocol_version);
    let mut post_service = PostService::new(vamo);
    post_service
        .patch_post(1, Post { title: "title".to_string(), body: "body".to_string(), user_id: 1 })
        .await?;

    server
        .stop()
        .await?;

    Ok(())
}
