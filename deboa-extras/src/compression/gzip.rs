use std::io::Write;

use bytes::Bytes;
use deboa::{compression::Compression, errors::DeboaError};
use flate2::write::GzEncoder;

trait GzipCompression: Compression {
    fn compress(&self) -> Result<&mut Self, DeboaError>;
}

impl GzipCompression for Bytes {
    fn compress(&self) -> Result<&mut Self, DeboaError> {
        let compressed_body = Vec::new();

        let mut writer = GzEncoder::new(compressed_body, flate2::Compression::default());
        let _ = writer.write_all(self.as_ref());

        Ok(&mut Bytes::from_owner(writer.get_ref().to_vec()))
    }
}
