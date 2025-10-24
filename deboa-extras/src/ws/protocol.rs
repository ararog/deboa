/// Message enum
///
/// # Variants
///
/// * `Text(String)` - A text message.
/// * `Binary(Vec<u8>)` - A binary message.
/// * `Close(u16, String)` - A close message.
/// * `Ping(Vec<u8>)` - A ping message.
/// * `Pong(Vec<u8>)` - A pong message.
#[derive(Clone)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
    Close(u16, String),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}
