use std::io::Write;

use bytes::Bytes;
use deboa::{compression::Compression, errors::DeboaError, io::Decompression, response::DeboaResponse};
use flate2::write::DeflateEncoder;

pub trait DeflateCompression: Compression {
    fn compress(&self) -> Result<Bytes, DeboaError>;
}

impl DeflateCompression for Vec<u8> {
    fn compress(&self) -> Result<Bytes, DeboaError> {
        let mut writer = DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        let result = writer.write_all(self);

        if let Err(e) = result {
            return Err(DeboaError::Compression { message: e.to_string() });
        }

        Ok(Bytes::from_owner(writer.get_ref().to_vec()))
    }
}

pub trait DeflateDecompression: Decompression {
    fn decompress(&self) -> Result<Bytes, DeboaError>;
}

impl DeflateDecompression for DeboaResponse {
    fn decompress(&self) -> Result<Bytes, DeboaError> {
        Ok(Bytes::new())
    }
}
