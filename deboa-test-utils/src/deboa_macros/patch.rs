#![allow(unused_variables)]
use crate::common::data::{Post, PostWithId};
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    Client, InnerClient, TestResult,
};
use deboa_extras::serde::json::JsonBody;
use deboa_macros::patch;
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::StatusCode;

pub async fn test_only_patch_minimal<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PATCH").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: PostWithId = PostWithId { id: 1 };
    let response = patch!(
        data => data,
        url => server.url("/posts/1"),
        client => client
    );

    assert_eq!(response.status(), 200);
    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_only_patch_minimal_headers<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PATCH").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let response = patch!(
        data => Post { id: 1, title: "title".to_string(), body: "body".to_string() },
        url => server.url("/posts/1"),
        headers => vec![("Content-Type", "application/json")],
        client => client
    );

    assert_eq!(response.status(), 200);
    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_patch<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PATCH").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(
        data => data,
        url => server.url("/posts/1"),
        client => client
    );

    assert_eq!(response.status(), 200);
    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_patch_with_headers<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PATCH").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(
        data => data,
        url => server.url("/posts/1"),
        headers => headers,
        client => client
    );

    assert_eq!(response.status(), 200);
    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_patch_with_json_body_request<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PATCH").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts/1"),
        headers => headers,
        client => client
    );

    assert_eq!(response.status(), 200);
    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_patch_with_json_body_no_headers<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PATCH").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{\"id\": 1, \"title\": \"Teste\", \"body\": \"Teste\"}"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts/1"),
        client => client,
        res_body_ty => JsonBody,
        res_ty => PostWithId
    );

    assert_eq!(response.id, 1);
    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_patch_with_json_body_response<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PATCH").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{\"id\": 1, \"title\": \"Teste\", \"body\": \"Teste\"}"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts/1"),
        headers => headers,
        client => client,
        res_body_ty => JsonBody,
        res_ty => Post
    );

    assert_eq!(response.id, 1);
    server
        .stop()
        .await?;

    Ok(())
}
