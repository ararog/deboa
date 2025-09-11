use deboa::{errors::DeboaError, interceptor::DeboaInterceptor, request::DeboaRequest, response::DeboaResponse};

#[derive(Debug, serde::Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

struct TestMonitor;

impl DeboaInterceptor for TestMonitor {
    fn on_request(&self, request: &mut DeboaRequest) {
        println!("Request: {:?}", request.url());
    }

    fn on_response(&self, response: &mut DeboaResponse) {
        println!("Response: {:?}", response.status());
    }
}

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    use deboa::Deboa;

    let mut api = Deboa::new();
    api.add_interceptor(Box::new(TestMonitor));

    let _ = DeboaRequest::get("https://jsonplaceholder.typicode.com").send_with(&mut api).await?;

    Ok(())
}
