use crate::Client;
use deboa::Result;
use http::Uri;

#[tokio::test]
async fn test_shl() -> Result<()> {
    let client = Client::default();
    let builder = &client << "https://httpbin.org/get";
    let request = builder.build()?;
    assert_eq!(*request.uri(), Uri::from_static("https://httpbin.org/get"));

    Ok(())
}
