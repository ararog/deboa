use deboa::{errors::DeboaError, interceptor::DeboaInterceptor, response::DeboaResponse};
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::{
    interceptor::encoding::EncodingInterceptor,
    io::deflate::DeflateDecompressor,
    tests::types::{DECOMPRESSED, DEFLATE_COMPRESSED},
};

#[tokio::test]
async fn test_deflate_decompress() -> Result<(), DeboaError> {
    let encoding_interceptor = EncodingInterceptor::register_decoders(vec![Box::new(DeflateDecompressor)]);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Encoding", HeaderValue::from_static("deflate"));
    let mut response = DeboaResponse::new(StatusCode::OK, headers, DEFLATE_COMPRESSED.as_ref());

    encoding_interceptor.on_response(&mut response);

    assert_eq!(response.raw_body(), DECOMPRESSED);
    Ok(())
}
