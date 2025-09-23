use deboa::{catcher::DeboaCatcher, errors::DeboaError, response::DeboaResponse};
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::{
    catcher::encoding::EncodingCatcher,
    io::deflate::DeflateDecompressor,
    tests::types::{DECOMPRESSED, DEFLATE_COMPRESSED, url},
};

#[tokio::test]
async fn test_deflate_decompress() -> Result<(), DeboaError> {
    let encoding_catcher = EncodingCatcher::register_decoders(vec![DeflateDecompressor]);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Encoding", HeaderValue::from_static("deflate"));
    let mut response = DeboaResponse::new(url(), StatusCode::OK, headers, DEFLATE_COMPRESSED.as_ref());

    encoding_catcher.on_response(&mut response);

    assert_eq!(response.raw_body(), DECOMPRESSED);
    Ok(())
}
