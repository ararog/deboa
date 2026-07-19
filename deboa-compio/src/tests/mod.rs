pub(crate) type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod cache;
mod client;
mod form;
mod helpers;
mod request;
mod response;
mod url;
