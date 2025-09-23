use deboa::{catcher::DeboaCatcher, errors::DeboaError, response::DeboaResponse};
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::{
    catcher::encoding::EncodingCatcher,
    io::brotli::BrotliDecompressor,
    tests::types::{BROTLI_COMPRESSED, DECOMPRESSED, url},
};

#[tokio::test]
async fn test_brotli_decompress() -> Result<(), DeboaError> {
    let encoding_catcher = EncodingCatcher::register_decoders(vec![BrotliDecompressor]);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Encoding", HeaderValue::from_static("br"));
    let mut response = DeboaResponse::new(url(), StatusCode::OK, headers, BROTLI_COMPRESSED.as_ref());

    encoding_catcher.on_response(&mut response);

    assert_eq!(response.raw_body(), DECOMPRESSED);
    Ok(())
}
