use std::io::{Read, Write};

use bytes::{Buf, Bytes};
use deboa::{
    Deboa,
    errors::DeboaError,
    io::{Compressor, Decompressor},
    response::DeboaResponse,
};
use flate2::{read::GzDecoder, write::GzEncoder};

#[derive(PartialEq)]
pub struct GzipCompressor;

impl Compressor for GzipCompressor {
    fn name(&self) -> String {
        "gzip".to_string()
    }

    fn compress_body(&self, request: &Deboa) -> Result<Bytes, DeboaError> {
        let mut writer = GzEncoder::new(Vec::new(), flate2::Compression::default());
        let result = writer.write_all(request.body().as_ref());

        if let Err(e) = result {
            return Err(DeboaError::Compress { message: e.to_string() });
        }

        let result = writer.flush();

        if let Err(e) = result {
            return Err(DeboaError::Compress { message: e.to_string() });
        }

        Ok(Bytes::from(writer.get_ref().to_vec()))
    }
}

#[derive(PartialEq)]
pub struct GzipDecompressor;

impl Decompressor for GzipDecompressor {
    fn name(&self) -> String {
        "gzip".to_string()
    }

    fn decompress_body(&self, response: &mut DeboaResponse) -> Result<(), DeboaError> {
        let binding = response.body();
        let mut reader = GzDecoder::new(binding.reader());
        let mut buffer = Vec::new();
        let result = reader.read_to_end(&mut buffer);

        if let Err(e) = result {
            return Err(DeboaError::Decompress { message: e.to_string() });
        }

        response.set_body(buffer);

        Ok(())
    }
}
