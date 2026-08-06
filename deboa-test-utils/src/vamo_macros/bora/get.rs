use deboa::{
    cert::{Certificate, Identity},
    conn::{HttpConnectionDispatcher, HttpConnectionPool},
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
use serde::Deserialize;
use vamo::Vamo;
use vamo_macros::bora;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
}

#[bora(
      api(
        get(name="get_all", path="/posts", res_body=Vec<Post>, format="json"),
        get(name="get_by_id", path="/posts/<id:i32>", res_body=Post, format="json"),
        get(name="query_by_id", path="/posts?<id:i32>", res_body=Vec<Post>, format="json"),
        get(name="query_by_title", path="/posts?<id:i32>&<title:&str>", res_body=Vec<Post>, format="json")
      )
    )]
pub struct PostService;

pub async fn test_get_by_id<S, I, C, P, R>(
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
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{ \"id\": 1, \"title\": \"title\" }"),
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
    let post = post_service
        .get_by_id(1)
        .await?;

    server
        .stop()
        .await?;

    println!("id...: {}", post.id);
    println!("title: {}", post.title);

    assert_eq!(post.id, 1);
    Ok(())
}

pub async fn test_get_all<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send + Default,
    P::Certificate: Certificate,
    P::Identity: Identity,
    P::ConnectionDispather: HttpConnectionDispatcher,
    R: DnsResolver + Send + Default,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(
                    b"[{ \"id\": 1, \"title\": \"title\" }, { \"id\": 2, \"title\": \"title\" }]",
                ),
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
    let posts = post_service
        .get_all()
        .await?;

    server
        .stop()
        .await?;

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 2);
    Ok(())
}

pub async fn test_query_by_id<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send + Default,
    P::Certificate: Certificate,
    P::Identity: Identity,
    P::ConnectionDispather: HttpConnectionDispatcher,
    R: DnsResolver + Send + Default,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{ \"id\": 1, \"title\": \"title\" }]"),
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
    let posts = post_service
        .query_by_id(1)
        .await?;

    server
        .stop()
        .await?;

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 1);
    Ok(())
}

pub async fn test_query_by_title<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send + Default,
    P::Certificate: Certificate,
    P::Identity: Identity,
    P::ConnectionDispather: HttpConnectionDispatcher,
    R: DnsResolver + Send + Default,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{ \"id\": 6, \"title\": \"dolorem eum magni eos aperiam quia\" }]"),
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
    let posts = post_service
        .query_by_title(6, "dolorem eum magni eos aperiam quia")
        .await?;

    server
        .stop()
        .await?;

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 1);
    Ok(())
}
