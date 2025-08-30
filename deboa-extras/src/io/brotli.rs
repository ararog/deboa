use bytes::{Buf, Bytes};
use std::io::{Read, Write};

use brotli::{CompressorReader, CompressorWriter};
use deboa::{
    Deboa,
    errors::DeboaError,
    io::{Compress, Decompress},
    response::DeboaResponse,
};

pub trait BrotliCompress: Compress {
    fn compress_body(&self) -> Result<Bytes, DeboaError>;
}

impl BrotliCompress for Deboa {
    fn compress_body(&self) -> Result<Bytes, DeboaError> {
        let mut writer = CompressorWriter::new(Vec::new(), 0, 11, 22);
        let result = writer.write_all(self.body().as_ref());

        if let Err(e) = result {
            return Err(DeboaError::Compress { message: e.to_string() });
        }

        let result = writer.flush();

        if let Err(e) = result {
            return Err(DeboaError::Compress { message: e.to_string() });
        }

        Ok(Bytes::from(writer.into_inner()))
    }
}

pub trait BrotliDecompress: Decompress {
    fn decompress_body(&mut self) -> Result<(), DeboaError>;
}

impl BrotliDecompress for DeboaResponse {
    fn decompress_body(&mut self) -> Result<(), DeboaError> {
        let binding = self.body();
        let mut reader = CompressorReader::new(binding.reader(), 0, 11, 22);
        let mut buffer = Vec::new();
        let result = reader.read_to_end(&mut buffer);

        if let Err(e) = result {
            return Err(DeboaError::Decompress { message: e.to_string() });
        }

        self.set_body(buffer);

        Ok(())
    }
}
