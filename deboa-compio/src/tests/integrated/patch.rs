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
        "{\n  \"userId\": 1,\n  \"id\": 1,\n  \"title\": \"sunt aut facere repellat provident occaecati excepturi optio reprehenderit\",\n  \"body\": \"quia et suscipit\\nsuscipit recusandae consequuntur expedita et cum\\nreprehenderit molestiae ut ut quas totam\\nnostrum rerum est autem sunt rem eveniet architecto\"\n}"
    );

    Ok(())
}
