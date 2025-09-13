use deboa::{catcher::DeboaCatcher, errors::DeboaError, request::DeboaRequest, response::DeboaResponse};

#[derive(Debug, serde::Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

struct TestMonitor;

impl DeboaCatcher for TestMonitor {
    fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>, DeboaError> {
        println!("Request: {:?}", request.url());
        Ok(None)
    }

    fn on_response(&self, response: &mut DeboaResponse) {
        println!("Response: {:?}", response.status());
    }
}

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    use deboa::Deboa;

    let client = Deboa::builder().catch(TestMonitor).build();

    let _ = DeboaRequest::get("https://jsonplaceholder.typicode.com").send_with(client).await?;

    Ok(())
}
