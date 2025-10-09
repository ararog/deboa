use deboa::{catcher::DeboaCatcher, response::DeboaResponse, Result};
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::{catcher::encoding::EncodingCatcher, io::deflate::DeflateDecompressor};

use deboa_tests::{
    data::{DECOMPRESSED, DEFLATE_COMPRESSED},
    utils::fake_url,
};

#[tokio::test]
async fn test_deflate_decompress() -> Result<()> {
    let encoding_catcher = EncodingCatcher::register_decoders(vec![DeflateDecompressor]);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Encoding", HeaderValue::from_static("deflate"));
    let response = DeboaResponse::new(
        fake_url(),
        StatusCode::OK,
        headers,
        DEFLATE_COMPRESSED.as_ref(),
    );

    let mut response = encoding_catcher.on_response(response).await?;

    assert_eq!(response.raw_body().await, DECOMPRESSED);
    Ok(())
}
