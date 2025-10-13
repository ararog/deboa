use deboa::{Deboa, errors::DeboaError};
use deboa_extras::http::sse::listener::IntoSSE;

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    let mut client = Deboa::new();

    let response = client.execute("https://sse.dev/test").await?.into_sse();

    response
        .poll_event(|event| {
            println!("event: {}", event);
            Ok(())
        })
        .await?;

    Ok(())
}
