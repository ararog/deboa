use deboa::{
    catcher::DeboaCatcher,
    response::{DeboaResponse, IntoBody},
    Result,
};
use deboa_tests::{
    data::{DECOMPRESSED, GZIP_COMPRESSED},
    utils::fake_url,
};
use http::{Response, StatusCode};

use crate::{catcher::encoding::EncodingCatcher, io::gzip::GzipDecompressor};

#[tokio::test]
async fn test_gzip() -> Result<()> {
    let encoding_catcher = EncodingCatcher::register_decoders(vec![GzipDecompressor]);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Encoding", "gzip")
        .body(GZIP_COMPRESSED.into_body())
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
