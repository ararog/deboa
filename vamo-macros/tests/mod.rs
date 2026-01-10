pub(crate) const SKIP_CERT_VERIFICATION: bool =
    cfg!(any(feature = "_tokio-native-tls", feature = "_smol-native-tls"));

mod bora;
mod resource;
