pub(crate) const SKIP_CERT_VERIFICATION: bool =
    cfg!(any(feature = "_tokio-native-tls", feature = "_smol-native-tls"));

#[cfg(feature = "tests")]
mod bora;
#[cfg(feature = "tests")]
mod resource;
