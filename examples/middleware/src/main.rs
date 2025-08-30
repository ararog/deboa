use deboa::{errors::DeboaError, response::DeboaResponse};

#[derive(Debug, serde::Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

struct TestMonitor;

impl deboa::middleware::DeboaMiddleware for TestMonitor {
    fn on_request(&self, request: &deboa::Deboa) {
        println!("Request: {:?}", request.base_url());
    }

    fn on_response(&self, _request: &deboa::Deboa, response: &mut DeboaResponse) {
        println!("Response: {:?}", response.status());
    }
}

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    use deboa::Deboa;

    let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    api.add_middleware(Box::new(TestMonitor));

    let _ = api.get("/posts/1").await;

    Ok(())
}
