use deboa::{request::DeboaRequest, HttpClient};
use http::StatusCode;

use crate::Client;
//
// PUT
//
#[compio::test]
async fn test_put() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let request = DeboaRequest::put("https://jsonplaceholder.typicode.com/posts/1")?
        .text("ping")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
