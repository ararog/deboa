pub(crate) const SKIP_CERT_VERIFICATION: bool = cfg!(feature = "native-tls");

mod client;
mod form;
mod helpers;
mod request;
mod response;
