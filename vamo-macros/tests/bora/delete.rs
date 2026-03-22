use deboa::{
    cert::{Certificate, ContentEncoding},
    Client as DeboaClient,
};
use deboa_tests::{
    mock_response,
    utils::{start_mock_server, CA_CERT},
};
use http::StatusCode;
use vamo::Vamo;
use vamo_macros::bora;

use crate::SKIP_CERT_VERIFICATION;

#[bora(api(delete(name = "delete_post", path = "/posts/<id:i32>")))]
pub struct PostService;

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_delete_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "DELETE" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, ""))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = DeboaClient::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.base_url())?;
    vamo.client(client);

    let mut post_service = PostService::new(vamo);

    post_service
        .delete_post(1)
        .await?;

    server
        .stop()
        .await?;

    Ok(())
}
