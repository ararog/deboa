pub(crate) const SKIP_CERT_VERIFICATION: bool =
    cfg!(any(feature = "tokio-native-tls", feature = "smol-native-tls"));

mod cache;
//mod catcher;
mod cert;
mod client;
mod cookie;
mod form;
mod integrated;
mod request;
mod response;
mod url;

mod helpers;
