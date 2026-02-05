use deboa::{
    catcher::DeboaCatcher,
    response::{DeboaResponse, IntoBody},
    Result,
};
use http::{Response, StatusCode};

use crate::{catcher::encoding::EncodingCatcher, io::deflate::DeflateDecompressor};

use deboa_tests::{
    data::{DECOMPRESSED, DEFLATE_COMPRESSED},
    utils::fake_url,
};

#[tokio::test]
async fn test_deflate_decompress() -> Result<()> {
    let encoding_catcher = EncodingCatcher::register_decoders(vec![DeflateDecompressor]);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Encoding", "deflate")
        .body(DEFLATE_COMPRESSED.into_body())
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
