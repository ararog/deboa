//! End-to-end: a hyper server and a deboa client, both on one glommio executor.
//!
//! Self-contained on purpose. The other bindings test against
//! `easyhttpmock-vetis-*`, which is published per runtime and has no glommio
//! build; standing the server up by hand here keeps the test hermetic and, as
//! a side effect, proves both halves work on the same single-threaded core.

use std::convert::Infallible;
use std::net::SocketAddr;

use deboa::request::{get, FetchWith};
use deboa_glommio::Client;
use http_body_util::Full;
use hyper::body::Bytes;

/// hyper needs its own I/O traits; glommio's `TcpStream` speaks `futures-io`.
/// `smol_hyper`'s adapter bridges the two and pulls in no runtime of its own —
/// the same one the client uses internally.
fn serve(addr: SocketAddr, body: &'static str) {
    let listener = glommio::net::TcpListener::bind(addr).expect("bind");
    glommio::spawn_local(async move {
        while let Ok(stream) = listener.accept().await {
            glommio::spawn_local(async move {
                let service = hyper::service::service_fn(move |_req| async move {
                    Ok::<_, Infallible>(hyper::Response::new(Full::new(Bytes::from(body))))
                });
                let _ = hyper::server::conn::http1::Builder::new()
                    .serve_connection(smol_hyper::rt::FuturesIo::new(stream), service)
                    .await;
            })
            .detach();
        }
    })
    .detach();
}

/// A free port, bound and released so the server can take it.
fn ephemeral_addr() -> SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    listener.local_addr().expect("local_addr")
}

#[test]
fn a_request_round_trips_on_a_local_executor() {
    let addr = ephemeral_addr();

    glommio::LocalExecutorBuilder::default()
        .make()
        .expect("build glommio executor")
        .run(async move {
            serve(addr, "hello from glommio");

            let client = Client::default();
            let response = get(format!("http://{addr}/").as_str())
                .expect("build request")
                .version(http::Version::HTTP_11)
                .send_with(&client)
                .await
                .expect("request failed");

            assert_eq!(response.status(), http::StatusCode::OK);
        });
}

/// The pool is the reason a second request is cheaper than the first; this
/// asserts it at least does not *break* the second request, which is the
/// failure a naive pool produces (a connection handed back mid-body).
#[test]
fn two_requests_reuse_the_connection() {
    let addr = ephemeral_addr();

    glommio::LocalExecutorBuilder::default()
        .make()
        .expect("build glommio executor")
        .run(async move {
            serve(addr, "second");

            let client = Client::default();
            for _ in 0..2 {
                let response = get(format!("http://{addr}/").as_str())
                    .expect("build request")
                    .version(http::Version::HTTP_11)
                    .send_with(&client)
                    .await
                    .expect("request failed");
                assert_eq!(response.status(), http::StatusCode::OK);
            }
        });
}
