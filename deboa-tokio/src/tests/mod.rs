pub(crate) const SKIP_CERT_VERIFICATION: bool = cfg!(any(
    feature = "tokio-native-tls",
    feature = "smol-native-tls",
    feature = "compio-native-tls"
));

pub(crate) type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod cert;
mod client;
mod helpers;
mod integrated;
