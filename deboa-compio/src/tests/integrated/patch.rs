use crate::Client;
use deboa::{request::DeboaRequest, HttpClient};
use http::StatusCode;
//
// PATCH
//
#[compio::test]
async fn test_patch() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::default();

    let request = DeboaRequest::patch("https://jsonplaceholder.typicode.com/posts/1")?
        .text("text")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .text()
            .await?,
        "done"
    );

    Ok(())
}
