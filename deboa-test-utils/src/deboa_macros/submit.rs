#![allow(unused_variables)]
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    Client, InnerClient, TestResult,
};
use deboa_macros::submit;
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::{Method, StatusCode};

pub async fn test_submit_str_minimal<S, I, C, P, R>(
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
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let response = submit!(
        method => Method::POST,
        data => "user=deboa",
        url => server.url("/posts"),
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

pub async fn test_submit_str_method<S, I, C, P, R>(
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
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let headers = vec![("Content-Type", "application/x-www-form-urlencoded")];
    let response = submit!(
        method => Method::POST,
        data => "user=deboa",
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
