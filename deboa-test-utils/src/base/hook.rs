use caramelo::{expect, matchers::eq};
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    errors::DeboaError,
    request::DeboaRequest,
    response::DeboaResponse,
    Client, HttpClient as _, InnerClient, Result, TestResult,
};
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::StatusCode;
use tackle::{Chain, Hook, NextHook};

struct PrintRequestHook<H> {
    inner: H,
}

impl<H> Hook<DeboaRequest, DeboaResponse> for PrintRequestHook<H>
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>>,
{
    type Result = Result<DeboaResponse>;
    type Error = DeboaError;

    async fn call(&self, request: DeboaRequest) -> Self::Result {
        println!("Request 3: {:?}", request);
        self.inner
            .call(request)
            .await
    }
}

struct PrintRequest;

impl<H> Chain<H, DeboaError, DeboaRequest, DeboaResponse> for PrintRequest
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>>,
{
    type Hook = PrintRequestHook<H>;

    fn chain(&self, hook: H) -> Self::Hook {
        PrintRequestHook { inner: hook }
    }
}

async fn print_request<H>(request: DeboaRequest, _next: NextHook<H>) -> Result<DeboaResponse>
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>>,
{
    println!("Request 2: {:?}", request);
    _next
        .call(request)
        .await
}

pub async fn test_hook<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
) -> TestResult<()>
where
    I: Identity + Send + Clone + 'static,
    C: Certificate + Send + Clone + 'static,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send + 'static,
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

    let client = client
        .chain(PrintRequest)
        .chain_fn(print_request)
        .chain_fn(|req, next| async move {
            println!("Request 1: {:?}", req);
            next.call(req).await
        });

    let request = DeboaRequest::get(server.url("/posts/1"))?.build()?;
    let response: DeboaResponse = client
        .execute(request)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));

    server
        .stop()
        .await?;

    Ok(())
}
