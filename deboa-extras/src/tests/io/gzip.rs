use deboa::{errors::DeboaError, interceptor::DeboaInterceptor, response::DeboaResponse};
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::{
    interceptor::encoding::EncodingInterceptor,
    io::gzip::GzipDecompressor,
    tests::types::{DECOMPRESSED, GZIP_COMPRESSED},
};

#[tokio::test]
async fn test_gzip() -> Result<(), DeboaError> {
    let encoding_interceptor = EncodingInterceptor::register_decoders(vec![Box::new(GzipDecompressor)]);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Encoding", HeaderValue::from_static("gzip"));
    let mut response = DeboaResponse::new(StatusCode::OK, headers, GZIP_COMPRESSED.as_ref());

    encoding_interceptor.on_response(&mut response);

    assert_eq!(response.raw_body(), DECOMPRESSED);
    Ok(())
}
