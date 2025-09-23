use deboa::{catcher::DeboaCatcher, errors::DeboaError, response::DeboaResponse};
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::{
    catcher::encoding::EncodingCatcher,
    io::gzip::GzipDecompressor,
    tests::types::{DECOMPRESSED, GZIP_COMPRESSED, url},
};

#[tokio::test]
async fn test_gzip() -> Result<(), DeboaError> {
    let encoding_catcher = EncodingCatcher::register_decoders(vec![GzipDecompressor]);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Encoding", HeaderValue::from_static("gzip"));
    let mut response = DeboaResponse::new(url(), StatusCode::OK, headers, GZIP_COMPRESSED.as_ref());

    encoding_catcher.on_response(&mut response);

    assert_eq!(response.raw_body(), DECOMPRESSED);
    Ok(())
}
