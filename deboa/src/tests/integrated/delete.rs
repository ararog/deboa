use crate::tests::helpers::client_with_cert;
#[cfg(test)]
use crate::{request::DeboaRequest, Result};

use deboa_tests::{
    server::Server,
    utils::{make_response, start_mock_server},
};
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// DELETE
//

async fn do_delete() -> Result<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "DELETE" && req.uri().path() == "/posts/1" {
            Ok(make_response(StatusCode::OK, b""))
        } else {
            Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
        }
    })
    .await;

    let client = client_with_cert();

    let response = DeboaRequest::delete(server.url("/posts/1"))?
        .send_with(client)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_delete() -> Result<()> {
    do_delete().await?;
    Ok(())
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_delete() -> Result<()> {
    do_delete().await
}
