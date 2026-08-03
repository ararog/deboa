use crate::Client;
use caramelo::{expect, matchers::eq};
use deboa::Result;
use http::Uri;

#[tokio::test]
async fn test_shl() -> Result<()> {
    let client = Client::default();
    let builder = &client << "https://httpbin.org/get";
    let request = builder.build()?;
    expect(request.uri()).to_be(eq(&Uri::from_static("https://httpbin.org/get")));

    Ok(())
}
