use crate::http::serde::flex::FlexBody;
use deboa::{request::DeboaRequest, response::DeboaResponse, Result};
use deboa_tests::data::{sample_post, Post, FLEX_POST};
use deboa_tests::utils::fake_url;

#[test]
fn test_set_flex() -> Result<()> {
    let request = DeboaRequest::post(fake_url())
        .body_as(FlexBody, sample_post())?
        .build();

    assert_eq!(*request.raw_body(), FLEX_POST[..]);

    Ok(())
}

#[tokio::test]
async fn test_response_flex() -> Result<()> {
    let data = sample_post();

    let response = DeboaResponse::builder(fake_url())
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/flex")
        .body(&FLEX_POST[..])
        .build();
    let response: Post = response
        .body_as(FlexBody)
        .await?;

    assert_eq!(response, data);

    Ok(())
}
