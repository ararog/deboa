use std::io::Write;

use bytes::Bytes;
use deboa::compression::Compression;
use deboa::errors::DeboaError;
use flate2::write::DeflateEncoder;

trait DeflateCompression: Compression {
    fn compress(&self) -> Result<&mut Self, DeboaError>;
}

impl DeflateCompression for Bytes {
    fn compress(&self) -> Result<&mut Self, DeboaError> {
        let compressed_body = Vec::new();

        let mut writer = DeflateEncoder::new(compressed_body, flate2::Compression::default());
        let _ = writer.write_all(self.as_ref());

        Ok(&mut Bytes::from_owner(writer.get_ref().to_vec()))
    }
}
