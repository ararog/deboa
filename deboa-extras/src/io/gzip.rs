use std::io::Write;

use bytes::Bytes;
use deboa::{compression::Compression, errors::DeboaError, io::Decompression, response::DeboaResponse};
use flate2::write::GzEncoder;

pub trait GzipCompression: Compression {
    fn compress(&self) -> Result<Bytes, DeboaError>;
}

impl GzipCompression for Vec<u8> {
    fn compress(&self) -> Result<Bytes, DeboaError> {
        let mut writer = GzEncoder::new(Vec::new(), flate2::Compression::default());
        let result = writer.write_all(self);

        if let Err(e) = result {
            return Err(DeboaError::Compression { message: e.to_string() });
        }

        Ok(Bytes::from(writer.get_ref().to_vec()))
    }
}

pub trait GzipDecompression: Decompression {
    fn decompress(&self) -> Result<Bytes, DeboaError>;
}

impl GzipDecompression for DeboaResponse {
    fn decompress(&self) -> Result<Bytes, DeboaError> {
        Ok(Bytes::new())
    }
}
