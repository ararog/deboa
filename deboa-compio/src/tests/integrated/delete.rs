use deboa::request::DeboaRequest;
use http::StatusCode;

use crate::Client;

//
// DELETE
//
#[compio::test]
async fn test_delete() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let response = DeboaRequest::delete("https://httpbin.org/delete")?
        .send_with(&client)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
