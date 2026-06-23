use ::url::Url;

pub(crate) type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod cache;
//mod catcher;
mod cookie;
mod form;
mod request;
mod response;
mod url;

pub const TEST_URL: &str = "https://localhost:8000";

pub(crate) fn test_url() -> Url {
    Url::parse(TEST_URL).unwrap()
}
