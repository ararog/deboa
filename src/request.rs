#![allow(clippy::upper_case_acronyms)]


#[derive(Debug, strum_macros::Display, PartialEq)]
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