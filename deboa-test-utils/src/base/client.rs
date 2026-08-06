use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    Client, InnerClient, TestResult,
};
use http::Uri;

pub async fn test_shl<I, C, P, R>(_client: &Client<InnerClient<I, C, P, R>>) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send + Default,
    R: DnsResolver + Send + Default,
{
    let client = Client::<InnerClient<I, C, P, R>>::default();
    let builder = &client << "https://httpbin.org/get";
    let request = builder.build()?;
    assert_eq!(*request.uri(), Uri::from_static("https://httpbin.org/get"));

    Ok(())
}
