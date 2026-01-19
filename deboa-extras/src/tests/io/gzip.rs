use deboa::{catcher::DeboaCatcher, response::DeboaResponse, Result};
use deboa_tests::utils::fake_url;
use http::Response;
use http::StatusCode;
use http_body_util::Either;
use http_body_util::Full;

use crate::{catcher::encoding::EncodingCatcher, io::gzip::GzipDecompressor};

use deboa_tests::data::{DECOMPRESSED, GZIP_COMPRESSED};

#[tokio::test]
async fn test_gzip() -> Result<()> {
    let encoding_catcher = EncodingCatcher::register_decoders(vec![GzipDecompressor]);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Encoding", "gzip")
        .body(Either::Right(Full::from(GZIP_COMPRESSED.to_vec())))
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
