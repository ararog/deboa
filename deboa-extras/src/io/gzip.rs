use std::io::{Read, Write};

use bytes::{Buf, Bytes};
use deboa::{
    errors::{DeboaError, IoError},
    fs::io::{Compressor, Decompressor},
    request::DeboaRequest,
    response::DeboaResponse,
    Result,
};
use flate2::{read::GzDecoder, write::GzEncoder};

#[derive(PartialEq)]
pub struct GzipCompressor;

#[deboa::async_trait]
impl Compressor for GzipCompressor {
    fn name(&self) -> String {
        "gzip".to_string()
    }

    async fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes> {
        let mut writer = GzEncoder::new(Vec::new(), flate2::Compression::default());
        let result = writer.write_all(request.raw_body().as_ref());

        if let Err(e) = result {
            return Err(DeboaError::Io(IoError::Compress {
                message: e.to_string(),
            }));
        }

        let result = writer.finish();

        if let Err(e) = result {
            return Err(DeboaError::Io(IoError::Compress {
                message: e.to_string(),
            }));
        }

        Ok(Bytes::from(result.unwrap()))
    }
}

#[derive(PartialEq)]
pub struct GzipDecompressor;

#[deboa::async_trait]
impl Decompressor for GzipDecompressor {
    fn name(&self) -> String {
        "gzip".to_string()
    }

    async fn decompress_body(&self, response: &mut DeboaResponse) -> Result<()> {
        let body = response.raw_body().await;
        let mut reader = GzDecoder::new(body.reader());
        let mut buffer = Vec::new();
        let result = reader.read_to_end(&mut buffer);

        if let Err(e) = result {
            return Err(DeboaError::Io(IoError::Decompress {
                message: e.to_string(),
            }));
        }

        response.set_raw_body(Bytes::from(buffer));
        Ok(())
    }
}
