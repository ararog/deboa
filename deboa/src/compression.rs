use bytes::Bytes;

use crate::errors::DeboaError;

pub struct CompressionImpl;

pub trait Compression: Send + Sync + 'static {
    fn compress(&self) -> Result<Bytes, DeboaError>;
}

impl Compression for Bytes {
    fn compress(&self) -> Result<Bytes, DeboaError> {
        Ok(self.clone())
    }
}
