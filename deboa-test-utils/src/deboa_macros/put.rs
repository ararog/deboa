#![allow(unused_variables)]
use crate::common::data::{Post, PostWithId};
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    Client, InnerClient, TestResult,
};
use deboa_extras::serde::json::JsonBody;
use deboa_macros::put;
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::StatusCode;

pub async fn test_only_put_minimal<S, I, C, P, R>(
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
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = put!(
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

pub async fn test_only_put_minimal_headers<S, I, C, P, R>(
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
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
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

pub async fn test_put<S, I, C, P, R>(
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
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = put!(
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

pub async fn test_put_with_headers<S, I, C, P, R>(
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
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
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

pub async fn test_put_with_json_body_request<S, I, C, P, R>(
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
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
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

pub async fn test_put_with_json_body_no_headers<S, I, C, P, R>(
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
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{\"id\": 1, \"title\": \"Teste\", \"body\": \"Teste\"}"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = put!(
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

pub async fn test_put_with_json_body_response<S, I, C, P, R>(
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
        given(method("PUT").and(path("/posts/1"))).will_return(
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
    let response = put!(
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
