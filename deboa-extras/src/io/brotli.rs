use bytes::{Buf, Bytes};
use std::io::{Read, Write};

use brotli::CompressorWriter;
use deboa::{
    errors::DeboaError,
    fs::io::{Compressor, Decompressor},
    request::DeboaRequest,
    response::DeboaResponse,
};

#[derive(PartialEq)]
pub struct BrotliCompressor;

impl Compressor for BrotliCompressor {
    fn name(&self) -> String {
        "br".to_string()
    }

    fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes, DeboaError> {
        let mut writer = CompressorWriter::new(Vec::new(), 0, 11, 22);
        let result = writer.write_all(request.raw_body().as_ref());

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

#[derive(PartialEq)]
pub struct BrotliDecompressor;

impl Decompressor for BrotliDecompressor {
    fn name(&self) -> String {
        "br".to_string()
    }

    fn decompress_body(&self, response: &mut DeboaResponse) -> Result<(), DeboaError> {
        let binding = response.raw_body();
        let mut reader = brotli::Decompressor::new(binding.reader(), 0);
        let mut buffer = Vec::new();
        let result = reader.read_to_end(&mut buffer);

        if let Err(e) = result {
            return Err(DeboaError::Decompress { message: e.to_string() });
        }

        response.set_raw_body(&buffer);

        Ok(())
    }
}
