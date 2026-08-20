// No `native` module: `native-tls` is rejected with a `compile_error!` in this
// binding, because `async-native-tls` selects its runtime by feature and has no
// glommio one. Declaring the module anyway would leave `cargo fmt` chasing a
// file that does not exist.
#[cfg(feature = "rust-tls")]
mod rustls;

#[cfg(feature = "rust-tls")]
pub(crate) use rustls::*;
