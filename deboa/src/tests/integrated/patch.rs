use crate::{request::DeboaRequest, tests::helpers::client_with_cert, Client, Result};

use deboa_tests::{
    server::Server,
    utils::{make_response, start_mock_server},
};
use http::{header::HOST, StatusCode};

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// PATCH
//

async fn do_patch() -> Result<()> {
    let mut server = start_mock_server(|req| {
        if req.method() == "PATCH" && req.uri().path() == "/posts/1" {
            assert!(req
                .headers()
                .contains_key(HOST));
            Ok(make_response(StatusCode::OK, b"done"))
        } else {
            Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
        }
    })
    .await;

    let client: Client = client_with_cert();

    let request = DeboaRequest::patch(server.url("/posts/1"))?
        .text("text")
        .build()?;

    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .text()
            .await?,
        "done"
    );

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_patch() -> Result<()> {
    do_patch().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_patch() -> Result<()> {
    do_patch().await
}
