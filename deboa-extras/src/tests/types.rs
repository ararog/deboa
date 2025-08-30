use serde::{Deserialize, Serialize};

pub const JSONPLACEHOLDER: &str = "https://jsonplaceholder.typicode.com";
#[cfg(feature = "msgpack")]
pub const MSGPACK_POST: [u8; 23] = [
    147, 1, 164, 84, 101, 115, 116, 175, 83, 111, 109, 101, 32, 116, 101, 115, 116, 32, 116, 111, 32, 100, 111,
];
#[cfg(feature = "xml")]
pub const XML_POST: &[u8; 108] = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><Post><id>1</id><title>Test</title><body>Some test to do</body></Post>";
pub const JSON_POST: &[u8; 48] = b"{\"id\":1,\"title\":\"Test\",\"body\":\"Some test to do\"}";

pub fn sample_post() -> Post {
    Post {
        id: 1,
        title: "Test".to_string(),
        body: "Some test to do".to_string(),
    }
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct Post {
    #[allow(unused)]
    pub id: i32,
    #[allow(unused)]
    pub title: String,
    #[allow(unused)]
    pub body: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[cfg(feature = "json")]
pub struct Comment {
    #[allow(unused)]
    pub id: i32,
    #[allow(unused)]
    pub name: String,
    #[allow(unused)]
    pub email: String,
    #[allow(unused)]
    pub body: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    pub response_code: i32,
    pub response_message: String,
}
