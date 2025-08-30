use std::io::{Read, Write};

use bytes::Bytes;
use deboa::{errors::DeboaError, io::Compress, io::Decompress, response::DeboaResponse};
use flate2::{read::GzDecoder, write::GzEncoder};

pub trait GzipCompress: Compress {
    fn register_encoding(&mut self) -> &mut Self;
    fn compress(&self) -> Result<Bytes, DeboaError>;
}

impl GzipCompress for Vec<u8> {
    fn register_encoding(&mut self) -> &mut Self {
        self.edit_header(header::ACCEPT_ENCODING, "gzip".to_string());
        self
    }

    fn compress(&self) -> Result<Bytes, DeboaError> {
        let mut writer = GzEncoder::new(Vec::new(), flate2::Compression::default());
        let result = writer.write_all(self);

        if let Err(e) = result {
            return Err(DeboaError::Compress { message: e.to_string() });
        }

        Ok(Bytes::from(writer.get_ref().to_vec()))
    }
}

pub trait GzipDecompress: Decompress {
    fn decompress(&mut self) -> Result<(), DeboaError>;
}

impl GzipDecompress for DeboaResponse {
    fn decompress(&mut self) -> Result<(), DeboaError> {
        let binding = self.raw_body();
        let mut reader = GzDecoder::new(binding.reader());
        let mut buffer = Vec::new();
        let result = reader.read_to_end(&mut buffer);

        if let Err(e) = result {
            return Err(DeboaError::Decompress { message: e.to_string() });
        }

        self.set_body(buffer);

        Ok(())
    }
}
