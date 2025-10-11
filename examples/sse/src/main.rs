use deboa::{Deboa, errors::DeboaError};
use deboa_extras::http::sse::listener::EventListener;

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    let mut client = Deboa::new();
    
    let mut response = client.execute("https://sse.dev/test").await?;
    response.poll_event(|event| {
        println!("event: {}", event);
        Ok(())
    }).await?;

    Ok(())
}
