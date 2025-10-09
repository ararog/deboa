use deboa::{catcher::DeboaCatcher, response::DeboaResponse, Result};
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::{catcher::encoding::EncodingCatcher, io::brotli::BrotliDecompressor};

use deboa_tests::{
    data::{BROTLI_COMPRESSED, DECOMPRESSED},
    utils::fake_url,
};

#[tokio::test]
async fn test_brotli_decompress() -> Result<()> {
    let encoding_catcher = EncodingCatcher::register_decoders(vec![BrotliDecompressor]);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Encoding", HeaderValue::from_static("br"));
    let response = DeboaResponse::new(
        fake_url(),
        StatusCode::OK,
        headers,
        BROTLI_COMPRESSED.as_ref(),
    );

    let mut response = encoding_catcher.on_response(response).await?;

    assert_eq!(response.raw_body().await, DECOMPRESSED);
    Ok(())
}
