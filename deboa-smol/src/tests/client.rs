use deboa::Result;
use http::Uri;
use macro_rules_attribute::apply;
use smol_macros::test;

use crate::Client;

#[apply(test!)]
async fn test_shl() -> Result<()> {
    let client = Client::default();
    let builder = &client << "https://httpbin.org/get";
    let request = builder.build()?;

    assert_eq!(*request.uri(), Uri::from_static("https://httpbin.org/get"));

    Ok(())
}
