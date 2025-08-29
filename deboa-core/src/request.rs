#![allow(clippy::upper_case_acronyms)]

#[derive(Debug, strum_macros::Display, PartialEq)]
/// This enum define the request method.
///
/// # Examples
///
/// ```rust
/// use deboa_core::request::RequestMethod;
///
/// // Allow define the request method, in this case GET.
/// let method = RequestMethod::GET;
/// ```
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

pub trait DeboaRequest {}
