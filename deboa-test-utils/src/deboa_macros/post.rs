#![allow(unused_variables)]
use crate::common::data::Post;
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    Client, InnerClient, TestResult,
};
use deboa_extras::serde::json::JsonBody;
use deboa_macros::post;
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::StatusCode;

pub async fn test_only_post_minimal<S, I, C, P, R>(
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
        given(method("POST").and(path("/posts"))).will_return(
            StatusCode::CREATED
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = post!(
        data => data,
        url => server.url("/posts"),
        client => client
    );

    assert_eq!(response.status(), 201);
    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_only_post_minimal_headers<S, I, C, P, R>(
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
        given(method("POST").and(path("/posts"))).will_return(
            StatusCode::CREATED
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = post!(
        data => data,
        url => server.url("/posts"),
        headers => headers,
        client => client
    );

    assert_eq!(response.status(), 201);
    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_only_post<S, I, C, P, R>(
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
        given(method("POST").and(path("/posts"))).will_return(
            StatusCode::CREATED
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = post!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts"),
        client => client
    );

    assert_eq!(response.status(), 201);
    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_post_with_headers<S, I, C, P, R>(
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
        given(method("POST").and(path("/posts"))).will_return(
            StatusCode::CREATED
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = post!(
        data => data,
        req_body_ty => JsonBody,
        url => server.url("/posts"),
        headers => headers,
        client => client
    );
    assert_eq!(response.status(), 201);
    server
        .stop()
        .await?;

    Ok(())
}
