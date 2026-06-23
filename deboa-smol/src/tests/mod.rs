pub(crate) const SKIP_CERT_VERIFICATION: bool = cfg!(feature = "native-tls");

pub(crate) type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod cert;
mod client;
mod form;
mod helpers;
mod integrated;
mod request;
mod response;
