use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct Post {
    #[allow(unused)]
    pub id: i32,
    #[allow(unused)]
    pub title: String,
    #[allow(unused)]
    pub body: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[cfg(feature = "json")]
pub(crate) struct Comment {
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
pub(crate) struct Response {
    pub response_code: i32,
    pub response_message: String,
}
