use bytes::Bytes;
use std::io::Write;

use brotli::CompressorWriter;
use deboa::{
    Deboa,
    errors::DeboaError,
    io::{Compress, Decompress},
    response::DeboaResponse,
};

pub trait BrotliCompression: Compress {
    fn compress(&self) -> Result<Bytes, DeboaError>;
}

impl BrotliCompression for Deboa {
    fn compress(&self) -> Result<Bytes, DeboaError> {
        let mut writer = CompressorWriter::new(Vec::new(), self.raw_body().len(), 11, 22);
        let result = writer.write_all(self.raw_body());

        if let Err(e) = result {
            return Err(DeboaError::Compression { message: e.to_string() });
        }

        Ok(Bytes::from(writer.into_inner()))
    }
}

pub trait BrotliDecompression: Decompress {
    fn decompress(&self) -> Result<Bytes, DeboaError>;
}

impl BrotliDecompression for DeboaResponse {
    fn decompress(&self) -> Result<Bytes, DeboaError> {
        Ok(Bytes::new())
    }
}
