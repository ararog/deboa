use deboa::{Client, Result, request::post};
use deboa_extras::http::serde::cbor::CborBody;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: i32,
    message: String,
    value: f64,
    data: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::default();
    let payload = Message {
        id: 1233,
        message: "Hello, from CBOR!".to_string(),
        value: 12321.123,
        data: vec![1, 2, 3],
    };

    let url = "http://localhost:8080";

    let response: Message = post(url)
        .body_as(CborBody, &payload)?
        .send_with(&mut client)
        .await?
        .body_as(CborBody)
        .await?;

    println!("Decoded CBOR response:\n{response:#?}");

    Ok(())
}
