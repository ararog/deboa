use std::io::{Read, Write};

use bytes::{Buf, Bytes};
use deboa::{
    errors::DeboaError,
    fs::io::{Compressor, Decompressor},
    request::DeboaRequest,
    response::DeboaResponse,
    Result,
};
use flate2::{read::GzDecoder, write::GzEncoder};

#[derive(PartialEq)]
pub struct GzipCompressor;

impl Compressor for GzipCompressor {
    fn name(&self) -> String {
        "gzip".to_string()
    }

    fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes> {
        let mut writer = GzEncoder::new(Vec::new(), flate2::Compression::default());
        let result = writer.write_all(request.raw_body().as_ref());

        if let Err(e) = result {
            return Err(DeboaError::Compress { message: e.to_string() });
        }

        let result = writer.finish();

        if let Err(e) = result {
            return Err(DeboaError::Compress { message: e.to_string() });
        }

        Ok(Bytes::from(result.unwrap()))
    }
}

#[derive(PartialEq)]
pub struct GzipDecompressor;

impl Decompressor for GzipDecompressor {
    fn name(&self) -> String {
        "gzip".to_string()
    }

    fn decompress_body(&self, response: &mut DeboaResponse) -> Result<()> {
        let binding = response.raw_body();
        let mut reader = GzDecoder::new(binding.reader());
        let mut buffer = Vec::new();
        let result = reader.read_to_end(&mut buffer);

        if let Err(e) = result {
            return Err(DeboaError::Decompress { message: e.to_string() });
        }

        response.set_raw_body(&buffer);

        Ok(())
    }
}
