use std::rc::Rc;

use crate::common::helpers::{create_client, create_server};
use deboa::{
    errors::DeboaError, request::DeboaRequest, response::DeboaResponse, HttpClient as _, Result,
    TestResult,
};
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt as _},
};
use http::StatusCode;
use tackle::{Chain, Hook};

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

async fn print_request<H>(request: DeboaRequest, _next: Rc<H>) -> Result<DeboaResponse>
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>>,
{
    println!("Request 2: {:?}", request);
    _next
        .call(request)
        .await
}

#[tokio::test]
async fn test_hook() -> TestResult<()> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"Hello World!"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;

    let client = create_client()
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

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response
            .status()
            .as_u16(),
        StatusCode::OK.as_u16()
    );

    server
        .stop()
        .await?;

    Ok(())
}
