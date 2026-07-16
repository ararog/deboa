pub(crate) type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) mod data;
pub(crate) mod helpers;
