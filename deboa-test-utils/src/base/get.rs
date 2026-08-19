use caramelo::{
    expect,
    matchers::{eq, err, truthy},
};
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    errors::{ConnectionError, DeboaError},
    request::{DeboaRequest, IntoRequest},
    response::DeboaResponse,
    Client, HttpClient, InnerClient, TestResult,
};
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::{StatusCode, Version};

pub async fn test_get_http<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"Hello World!"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let request = DeboaRequest::get(server.url("/posts/1"))?
        .version(protocol_version)
        .build()?;
    let response: DeboaResponse = client
        .execute(request)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_skip_cert_verification<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
    skip: bool,
) -> TestResult<()>
where
    I: Identity + Send + Clone + 'static,
    C: Certificate + Send + Clone + 'static,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send + Default + 'static,
    R: DnsResolver + Send + Default + 'static,
    S: ServerAdapter + 'static,
{
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"Hello World!"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let request = DeboaRequest::get(server.url("/posts/1"))?
        .version(protocol_version)
        .build()?;
    let http_version = request.version();
    let response = client
        .execute(request)
        .await;

    if skip {
        match http_version {
            Version::HTTP_11 => {
                let response = response?;
                expect(response.status()).to_be(eq(StatusCode::OK));
            }
            Version::HTTP_2 => {
                let response = response?;
                expect(response.status()).to_be(eq(StatusCode::OK));
            }
            Version::HTTP_3 => {
                let error = DeboaError::Connection(ConnectionError::Udp {
                    message: "Could not connect to server: aborted by peer: the cryptographic handshake failed: error 120: peer doesn't support any known protocol".to_string(),
                });
                expect(response.unwrap_err()).to_be(eq(error));
            }
            _ => unreachable!(),
        }
    } else {
        expect(response).to_be(err());
    }

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_get_http_mutual_authentication<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: Version,
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
                .with_body(b"Hello World!"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let request = DeboaRequest::get(server.url("/posts/1"))?
        .version(protocol_version)
        .build()?;
    let response = client
        .execute(request)
        .await;

    expect(response?.status()).to_be(eq(StatusCode::OK));

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_get_not_found<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::NOT_FOUND
                .respond()
                .with_body(b"Not found"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let response = DeboaRequest::get(server.url("/asasa/posts/1ddd"))?
        .version(protocol_version)
        .send_with(client)
        .await?;
    expect(response.status()).to_be(eq(StatusCode::NOT_FOUND));

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_get_invalid_server<I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
{
    let request = DeboaRequest::get("https://invalid-server.com/posts")?
        .text("test")
        .build()?;
    let response: deboa::Result<DeboaResponse> = client
        .execute(request)
        .await;

    expect(response.is_err()).to_be(truthy());
    expect(response).to_be(err());

    Ok(())
}

pub async fn test_get_by_query<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("GET").and(path("/comments/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"My comment"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let response = DeboaRequest::get(server.url("/comments/1"))?
        .version(protocol_version)
        .send_with(client)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));

    let comments = response
        .text()
        .await;

    expect(comments.is_ok()).to_be(truthy());
    expect(comments.unwrap()).to_be(eq("My comment"));

    server
        .stop()
        .await?;

    Ok(())
}

/*
async fn test_get_by_query_with_retries() {
    let mut server = start_mock_server(|_req| async move {
        Ok(make_response(StatusCode::BAD_GATEWAY, "pong"))
    })
    .await;

    let client = client_with_cert();

    let response = DeboaRequest::get(server.url("/comments/1"))?
        .retries(2)
        .send_with(client)
        .await;

    if let Err(err) = response {
        assert_eq!(
            err,
            DeboaError::Response(ResponseError::Receive {
                status_code: StatusCode::BAD_GATEWAY,
                message: "Could not process request (502 Bad Gateway): pong".to_string(),
            }),
        );
    }

    server.stop().await;

    Ok(())
}
*/

/*
async fn test_get_with_redirect() -> TestResult<()> {
    let client = Client::default();

    let url = if cfg!(feature = "http3-tokio") {
        "https://tinyurl.com/bccjpjd7"
    } else {
        "https://tinyurl.com/bp6e548"
    };

    let response = DeboaRequest::get(url)?
        .send_with(client)
        .await?;

    let server = if cfg!(feature = "http3-tokio") { "facebook.com" } else { "github.com" };

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("server")
            .unwrap()
            .to_str()
            .unwrap(),
        server
    );

    Ok(())
}
*/

pub async fn test_try_into<S, I, C, P, R>(
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
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let first_post = server.url("/posts/1");
    let response = client
        .execute(first_post.into_request()?)
        .await?;
    expect(response.status()).to_be(eq(200));

    server
        .stop()
        .await?;

    Ok(())
}

/*
pub async fn test_fetch_from_str<S, I, C, P, R>(
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
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let first_post = server.url("/posts/1");
    let response = first_post
        .fetch_with(client)
        .await?;
    expect(response.status()).to_be(eq(200));

    server
        .stop()
        .await?;

    Ok(())
}
*/
