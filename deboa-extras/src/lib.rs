#[cfg(any(
    all(feature = "deflate", feature = "brotli"),
    all(feature = "deflate", feature = "gzip"),
    all(feature = "brotli", feature = "gzip")
))]
compile_error!("Only one compression feature can be enabled at a time.");

#[cfg(feature = "compression")]
pub mod compression;

pub mod serialization;

#[cfg(test)]
mod tests;
