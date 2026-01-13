use crate::{cert::Certificate, tests::SKIP_CERT_VERIFICATION};
#[cfg(test)]
use crate::{request::DeboaRequest, Client, Result};

use deboa_tests::utils::{make_response, start_mock_server, CA_CERT};
use http::StatusCode;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

//
// DELETE
//

async fn do_delete() -> Result<()> {
    let mut server = start_mock_server(|req| {
        if req.method() == "DELETE" && req.uri().path() == "/posts/1" {
            Ok(make_response(StatusCode::OK, b""))
        } else {
            Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
        }
    })
    .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

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
