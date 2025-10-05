#![allow(unused_variables)]
use bytes::Bytes;

use crate::{request::DeboaRequest, response::DeboaResponse, Result};

/// Trait that represents the compressor.
pub trait Compressor: Send + Sync + 'static {
    /// This method returns the name of encoding for this compressor.
    ///
    /// # Returns
    ///
    /// * `String` - The name of the encoding.
    ///
    fn name(&self) -> String;
    /// This method compress the body of the request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    ///
    /// # Returns
    ///
    /// * `Result<Bytes>` - The compressed body of the request.
    ///
    fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes>;
}

impl<T: Compressor> Compressor for Box<T> {
    fn name(&self) -> String {
        self.as_ref().name()
    }

    fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes> {
        self.as_ref().compress_body(request)
    }
}

/// Trait that represents the decompressor.
pub trait Decompressor: Send + Sync + 'static {
    /// This method register the encoding of the response.
    ///
    /// # Arguments
    ///
    /// * `response` - The response that was received.
    ///
    fn name(&self) -> String;
    /// This method decompress the body of the response.
    ///
    /// # Arguments
    ///
    /// * `response` - The response that was received.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - The decompressed body of the response.
    ///
    fn decompress_body(&self, response: &mut DeboaResponse) -> Result<()> {
        Ok(())
    }
}

impl<T: Decompressor> Decompressor for Box<T> {
    fn name(&self) -> String {
        self.as_ref().name()
    }

    fn decompress_body(&self, response: &mut DeboaResponse) -> Result<()> {
        self.as_ref().decompress_body(response)
    }
}
