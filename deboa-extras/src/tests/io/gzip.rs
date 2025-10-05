use deboa::{catcher::DeboaCatcher, response::DeboaResponse, Result};
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::{catcher::encoding::EncodingCatcher, io::gzip::GzipDecompressor};

use deboa_tests::data::{DECOMPRESSED, GZIP_COMPRESSED};
use deboa_tests::utils::fake_url;

#[tokio::test]
async fn test_gzip() -> Result<()> {
    let encoding_catcher = EncodingCatcher::register_decoders(vec![GzipDecompressor]);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Encoding", HeaderValue::from_static("gzip"));
    let mut response = DeboaResponse::new(fake_url(), StatusCode::OK, headers, GZIP_COMPRESSED.as_ref());

    encoding_catcher.on_response(&mut response);

    assert_eq!(response.raw_body(), DECOMPRESSED);
    Ok(())
}
