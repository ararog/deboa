use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    url::IntoUrl,
    Client, InnerClient, TestResult,
};
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::StatusCode;
use vamo::Vamo;
use vamo_macros::bora;

#[bora(api(delete(name = "delete_post", path = "/posts/<id:i32>")))]
pub struct PostService;

pub async fn test_delete_by_id<S, I, C, P, R>(
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
        given(method("DELETE").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let vamo = Vamo::from_client(client)?
        .base_url(
            server
                .base_url()
                .into_url()?,
        )
        .version(protocol_version);
    let mut post_service = PostService::new(vamo);
    post_service
        .delete_post(1)
        .await?;

    server
        .stop()
        .await?;

    Ok(())
}
