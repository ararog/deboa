#![allow(unused_variables)]
use crate::common::data::Post;
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    Client, InnerClient, TestResult,
};
use deboa_extras::serde::json::JsonBody;
use deboa_macros::fetch;
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::StatusCode;

pub async fn test_fetch_str_minimal<S, I, C, P, R>(
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
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let response = fetch!(url => server.url("/posts"), client => client);
    assert!(response
        .status()
        .is_success());

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_fetch_str_minimal_headers<S, I, C, P, R>(
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
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let headers = vec![("User-Agent", "deboa")];
    let response = fetch!(
        url => server.url("/posts"),
        headers => headers,
        client => client
    );
    assert!(response
        .status()
        .is_success());

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_fetch_str<S, I, C, P, R>(
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
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let response = fetch!(
        url => server.url("/posts"),
        client => client,
        res_body_ty => JsonBody,
        res_ty => Vec<Post>
    );
    assert_eq!(response.len(), 1);

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_fetch_ident<S, I, C, P, R>(
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
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let url = server.url("/posts");
    let response = fetch!(
        url => url,
        client => client,
        res_body_ty => JsonBody,
        res_ty => Vec<Post>
    );
    assert_eq!(response.len(), 1);

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_fetch_ident_with_headers<S, I, C, P, R>(
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
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{\"id\": 20, \"title\": \"Teste\", \"body\": \"Teste\"}]"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let url = server.url("/posts");
    let headers = vec![("User-Agent", "deboa")];
    let response = fetch!(
        url => url,
        headers => headers,
        client => client,
        res_body_ty => JsonBody,
        res_ty => Vec<Post>
    );
    assert_eq!(response.len(), 1);
    server
        .stop()
        .await?;
    Ok(())
}
