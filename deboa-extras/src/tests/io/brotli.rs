use deboa::{
    catcher::DeboaCatcher,
    response::{DeboaResponse, IntoBody},
    Result,
};
use http::{Response, StatusCode};

use crate::{catcher::encoding::EncodingCatcher, io::brotli::BrotliDecompressor};

use deboa_tests::{
    data::{BROTLI_COMPRESSED, DECOMPRESSED},
    utils::fake_url,
};

#[tokio::test]
async fn test_brotli_decompress() -> Result<()> {
    let encoding_catcher = EncodingCatcher::register_decoders(vec![BrotliDecompressor]);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Encoding", "br")
        .body(BROTLI_COMPRESSED.into_body())
        .unwrap();

    let mut response = DeboaResponse::new(fake_url().into(), response);

    encoding_catcher
        .on_response(&mut response)
        .await?;

    assert_eq!(
        response
            .raw_body()
            .await,
        DECOMPRESSED
    );
    Ok(())
}
