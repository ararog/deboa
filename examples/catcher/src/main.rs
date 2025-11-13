use deboa::{Result, catcher::DeboaCatcher, request::DeboaRequest, response::DeboaResponse};

#[derive(Debug, serde::Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

struct TestMonitor;

#[deboa::async_trait]
impl DeboaCatcher for TestMonitor {
    async fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>> {
        println!("Request: {:?}", request.url());
        Ok(None)
    }

    async fn on_response(&self, response: &mut DeboaResponse) -> Result<()> {
        println!("Response: {:?}", response.status());
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    use deboa::Deboa;

    let client = Deboa::builder().catch(TestMonitor).build();

    let _ = DeboaRequest::get("https://jsonplaceholder.typicode.com")?
        .send_with(client)
        .await?;

    Ok(())
}
