use bytes::Bytes;

use crate::{errors::DeboaError, response::DeboaResponse, Deboa};

pub trait Compress: Send + Sync + 'static {
    /// This method compress the body of the request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    ///
    /// # Returns
    ///
    /// * `Result<Bytes, DeboaError>` - The compressed body of the request.
    ///
    fn compress_body(&self) -> Result<Bytes, DeboaError>;
}

impl Compress for Deboa {
    fn compress_body(&self) -> Result<Bytes, DeboaError> {
        Ok(Bytes::copy_from_slice(&self.body))
    }
}

pub trait Decompress: Send + Sync + 'static {
    /// This method decompress the body of the response.
    ///
    /// # Arguments
    ///
    /// * `response` - The response that was received.
    ///
    /// # Returns
    ///
    /// * `Result<Bytes, DeboaError>` - The decompressed body of the response.
    ///
    fn decompress_body(&self) -> Result<Bytes, DeboaError>;
}

impl Decompress for DeboaResponse {
    fn decompress_body(&self) -> Result<Bytes, DeboaError> {
        Ok(Bytes::copy_from_slice(&self.raw_body()))
    }
}
