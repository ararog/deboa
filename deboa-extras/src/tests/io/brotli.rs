use deboa::{errors::DeboaError, interceptor::DeboaInterceptor, response::DeboaResponse};
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::{
    interceptor::encoding::EncodingInterceptor,
    io::brotli::BrotliDecompressor,
    tests::types::{BROTLI_COMPRESSED, DECOMPRESSED},
};

#[tokio::test]
async fn test_brotli_decompress() -> Result<(), DeboaError> {
    let encoding_interceptor = EncodingInterceptor::register_decoders(vec![BrotliDecompressor]);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Encoding", HeaderValue::from_static("br"));
    let mut response = DeboaResponse::new(StatusCode::OK, headers, BROTLI_COMPRESSED.as_ref());

    encoding_interceptor.on_response(&mut response);

    assert_eq!(response.raw_body(), DECOMPRESSED);
    Ok(())
}
