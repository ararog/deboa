use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use bytes::Bytes;
use futures::future;
use http::{version::Version, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::{Request, Response};
use quinn::crypto::rustls::QuicClientConfig;
use quinn::Endpoint;

use crate::cert::{Certificate, Identity};
use crate::client::conn::stream::setup_rust_tls;
use crate::request::Http3Request;
use crate::{
    client::conn::{udp::DeboaUdpConnection, BaseHttpConnection},
    errors::{ConnectionError, DeboaError, RequestError, ResponseError},
    Result,
};

use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

async fn lookup_and_connect(
    host: &str,
    port: u16,
    client_endpoint: &Endpoint,
) -> std::result::Result<h3_quinn::Connection, Box<dyn std::error::Error>> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    let response = resolver
        .lookup_ip(host)
        .await?;

    let addr = response
        .iter()
        .next()
        .expect("no addresses returned!");

    let conn = client_endpoint
        .connect(SocketAddr::new(addr, port), host)?
        .await?;

    let quinn_conn: h3_quinn::Connection = h3_quinn::Connection::new(conn);

    Ok(quinn_conn)
}

impl DeboaUdpConnection for BaseHttpConnection<Http3Request> {
    type Sender = Http3Request;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_3
    }

    async fn connect(
        host: &str,
        port: u16,
        identity: &Option<Identity>,
        certificate: &Option<Certificate>,
        skip_cert_verification: bool,
    ) -> Result<BaseHttpConnection<Http3Request>> {
        let client_endpoint =
            Endpoint::client(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)));

        if let Err(e) = client_endpoint {
            return Err(DeboaError::Connection(ConnectionError::Udp {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let mut client_endpoint = client_endpoint.unwrap();

        let tls_config =
            setup_rust_tls(host, identity, certificate, skip_cert_verification, Some("h3"))?;

        let quic_config = QuicClientConfig::try_from(tls_config);
        if let Err(e) = quic_config {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let quic_config = quic_config.unwrap();

        let client_config = quinn::ClientConfig::new(Arc::new(quic_config));
        client_endpoint.set_default_client_config(client_config);

        let result = lookup_and_connect(host, port, &client_endpoint).await;

        if let Err(e) = result {
            return Err(DeboaError::Connection(ConnectionError::Udp {
                host: host.to_string(),
                message: format!("Could not connect to server: {}", e),
            }));
        }

        let conn = result.unwrap();

        let client = h3::client::new(conn).await;

        if let Err(e) = client {
            return Err(DeboaError::Connection(ConnectionError::Udp {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let (mut conn, send_request) = client.unwrap();

        tokio::spawn(async move {
            future::poll_fn(|cx| conn.poll_close(cx)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        Ok(BaseHttpConnection::<Http3Request> { sender: send_request })
    }

    async fn send_request(
        &mut self,
        request: Request<Full<Bytes>>,
    ) -> Result<Response<Full<Bytes>>> {
        let mut sender = self.sender.clone();

        let url = request
            .uri()
            .to_string();

        let method = request
            .method()
            .to_string();

        let (parts, mut body) = request.into_parts();

        let bodyless_request = Request::from_parts(parts, ());

        let request = sender
            .send_request(bodyless_request)
            .await;

        if let Err(err) = request {
            return Err(DeboaError::Request(RequestError::Send {
                url: url.to_string(),
                method: method.to_string(),
                message: err.to_string(),
            }));
        }

        let request_stream = request.unwrap();
        let (mut send_stream, mut recv_stream) = request_stream.split();

        if method == "POST" || method == "PUT" || method == "PATCH" {
            while let Some(chunk) = body.frame().await {
                let frame = chunk.unwrap();
                if let Some(bytes) = frame.data_ref() {
                    let result = send_stream
                        .send_data(bytes.clone())
                        .await;

                    if let Err(err) = result {
                        return Err(DeboaError::Request(RequestError::Send {
                            url: url.to_string(),
                            method: method.to_string(),
                            message: err.to_string(),
                        }));
                    }
                }
            }
        }

        let finish_request = send_stream
            .finish()
            .await;
        if let Err(err) = finish_request {
            return Err(DeboaError::Request(RequestError::Send {
                url: url.to_string(),
                method: method.to_string(),
                message: err.to_string(),
            }));
        }

        let response = recv_stream
            .recv_response()
            .await;
        if let Err(err) = response {
            return Err(DeboaError::Response(ResponseError::Receive {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.to_string(),
            }));
        }

        let response = self
            .process_response(response, recv_stream)
            .await?;

        Ok(response)
    }
}

impl crate::client::conn::udp::private::DeboaUdpConnectionSealed
    for BaseHttpConnection<Http3Request>
{
}
