use std::io::{Read, Write};

use bytes::Bytes;
use deboa::{errors::DeboaError, io::Compress, io::Decompress, response::DeboaResponse};
use flate2::{read::DeflateDecoder, write::DeflateEncoder};

pub trait DeflateCompress: Compress {
    fn compress_body(&self) -> Result<Bytes, DeboaError>;
}

impl DeflateCompress for Vec<u8> {
    fn compress_body(&self) -> Result<Bytes, DeboaError> {
        let mut writer = DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        let result = writer.write_all(self);

        if let Err(e) = result {
            return Err(DeboaError::Compress { message: e.to_string() });
        }

        Ok(Bytes::from_owner(writer.get_ref().to_vec()))
    }
}

pub trait DeflateDecompress: Decompress {
    fn decompress_body(&mut self) -> Result<(), DeboaError>;
}

impl DeflateDecompress for DeboaResponse {
    fn decompress_body(&mut self) -> Result<(), DeboaError> {
        let binding = self.raw_body();
        let mut reader = DeflateDecoder::new(binding.reader());
        let mut buffer = Vec::new();
        let result = reader.read_to_end(&mut buffer);

        if let Err(e) = result {
            return Err(DeboaError::Decompress { message: e.to_string() });
        }

        self.set_body(buffer);

        Ok(())
    }
}
