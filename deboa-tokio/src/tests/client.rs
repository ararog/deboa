use std::sync::Arc;

use url::Url;

use crate::{default_protocol, Client, Result};

#[tokio::test]
async fn test_shl() -> Result<()> {
    let client = Client::default();
    let request = &client << "https://httpbin.org/get";

    assert_eq!(request.url(), Arc::new(Url::parse("https://httpbin.org/get").unwrap()));

    Ok(())
}
