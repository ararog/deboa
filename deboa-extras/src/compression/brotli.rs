use bytes::Bytes;
use std::io::Write;

use brotli::CompressorWriter;
use deboa::compression::Compression;
use deboa::errors::DeboaError;

pub trait BrotliCompression: Compression {
    fn compress(&self) -> Result<Bytes, DeboaError>;
}

impl BrotliCompression for Bytes {
    fn compress(&self) -> Result<Bytes, DeboaError> {
        let compressed_body = Vec::new();

        let compressed_bytes = Bytes::new();

        let mut writer = CompressorWriter::new(compressed_body, self.len(), 11, 22);
        let _ = writer.write_all(&compressed_bytes);

        Ok(compressed_bytes)
    }
}
