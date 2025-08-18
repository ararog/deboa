use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, strum_macros::Display, PartialEq)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    TRACE,
    HEAD,
    CONNECT,
}