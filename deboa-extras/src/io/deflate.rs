use std::io::{Read, Write};

use bytes::{Buf, Bytes};
use deboa::{
    errors::DeboaError,
    fs::io::{Compressor, Decompressor},
    response::DeboaResponse,
    request::DeboaRequest,
    Result,
};
use flate2::{read::DeflateDecoder, write::DeflateEncoder};

pub struct DeflateCompressor;

#[async_trait::async_trait]
impl Compressor for DeflateCompressor {
    fn name(&self) -> String {
        "deflate".to_string()
    }

    async fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes> {
        let mut writer = DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        let result = writer.write_all(request.raw_body().as_ref());

        if let Err(e) = result {
            return Err(DeboaError::Compress {
                message: e.to_string(),
            });
        }

        let result = writer.flush();

        if let Err(e) = result {
            return Err(DeboaError::Compress {
                message: e.to_string(),
            });
        }

        Ok(Bytes::from_owner(writer.get_ref().to_vec()))
    }
}

#[derive(PartialEq)]
pub struct DeflateDecompressor;

#[async_trait::async_trait]
impl Decompressor for DeflateDecompressor {
    fn name(&self) -> String {
        "deflate".to_string()
    }

    async fn decompress_body(&self, response: &mut DeboaResponse) -> Result<()> {
        let body = response.raw_body().await;
        let mut reader = DeflateDecoder::new(body.reader());
        let mut buffer = Vec::new();
        let result = reader.read_to_end(&mut buffer);

        if let Err(e) = result {
            return Err(DeboaError::Decompress {
                message: e.to_string(),
            });
        }

        response.set_raw_body(Bytes::from(buffer));
        Ok(())
    }
}
