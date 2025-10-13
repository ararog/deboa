use deboa::{Deboa, errors::DeboaError};
use deboa_extras::http::sse::response::IntoStream;

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    let mut client = Deboa::new();

    let response = client.execute("https://sse.dev/test").await?.into_stream();

    response
        .poll_event(|event| {
            println!("event: {}", event);
            Ok(())
        })
        .await?;

    Ok(())
}
