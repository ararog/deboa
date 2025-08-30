use bytes::Bytes;
use http::header;

use crate::{errors::DeboaError, response::DeboaResponse, Deboa};

pub trait Compress: Send + Sync + 'static {
    /// This method register the encoding of encoding of response.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request with the encoding registered.
    ///
    fn register_encoding(&mut self) -> &mut Self;

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
    fn register_encoding(&mut self) -> &mut Self {
        self.edit_header(header::ACCEPT_ENCODING, "identity".to_string());
        self
    }

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
    /// * `Result<(), DeboaError>` - The decompressed body of the response.
    ///
    fn decompress_body(&mut self) -> Result<(), DeboaError>;
}

impl Decompress for DeboaResponse {
    fn decompress_body(&mut self) -> Result<(), DeboaError> {
        Ok(())
    }
}
