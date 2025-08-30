#[cfg(any(
    all(feature = "deflate", feature = "brotli"),
    all(feature = "deflate", feature = "gzip"),
    all(feature = "brotli", feature = "gzip")
))]
compile_error!("Only one compression feature can be enabled at a time.");

#[cfg(feature = "compression")]
pub mod io;

pub mod http;

#[cfg(test)]
mod tests;
