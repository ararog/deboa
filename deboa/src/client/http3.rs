use std::{
    marker::PhantomData,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use rt_gate::spawn_worker;

use futures::future;
use http::{version::Version, StatusCode};
use http_body_util::BodyExt;
use hyper::{Request, Response};
use quinn::{crypto::rustls::QuicClientConfig, Endpoint};

use crate::{
    alpn,
    client::conn::{
        stream::setup_rust_tls, udp::DeboaUdpConnection, BaseHttpConnection, ConnectionConfig,
    },
    errors::{ConnectionError, DeboaError, RequestError, ResponseError},
    request::{BytesBody, Http3Request},
    response::DeboaBody,
    Result,
};

#[cfg(feature = "smol-rt")]
use async_std_resolver::{
    config::{ResolverConfig, ResolverOpts},
    resolver,
};

#[cfg(feature = "tokio-rt")]
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

async fn lookup_and_connect(
    host: &str,
    port: u16,
    client_endpoint: &Endpoint,
) -> std::result::Result<h3_quinn::Connection, Box<dyn std::error::Error>> {
    #[cfg(feature = "smol-rt")]
    let resolver = resolver(ResolverConfig::default(), ResolverOpts::default()).await;

    #[cfg(feature = "tokio-rt")]
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

impl DeboaUdpConnection for BaseHttpConnection<Http3Request, BytesBody, DeboaBody> {
    type Sender = Http3Request;
    type ReqBody = BytesBody;
    type ResBody = DeboaBody;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_3
    }

    async fn connect<'a>(
        config: &ConnectionConfig<'a>,
    ) -> Result<BaseHttpConnection<Self::Sender, Self::ReqBody, Self::ResBody>> {
        let client_endpoint =
            Endpoint::client(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)));

        if let Err(e) = client_endpoint {
            return Err(DeboaError::Connection(ConnectionError::Udp {
                host: config
                    .host()
                    .to_string(),
                message: e.to_string(),
            }));
        }

        let mut client_endpoint = client_endpoint.unwrap();

        let tls_config = setup_rust_tls(
            config.host(),
            config.identity(),
            config.certificate(),
            config.skip_cert_verification(),
            alpn(),
        )?;

        let quic_config = QuicClientConfig::try_from(tls_config);
        if let Err(e) = quic_config {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: config
                    .host()
                    .to_string(),
                message: e.to_string(),
            }));
        }

        let quic_config = quic_config.unwrap();

        let client_config = quinn::ClientConfig::new(Arc::new(quic_config));
        client_endpoint.set_default_client_config(client_config);

        let result = lookup_and_connect(config.host(), config.port(), &client_endpoint).await;

        if let Err(e) = result {
            return Err(DeboaError::Connection(ConnectionError::Udp {
                host: config
                    .host()
                    .to_string(),
                message: format!("Could not connect to server: {}", e),
            }));
        }

        let conn = result.unwrap();

        let client = h3::client::new(conn).await;

        if let Err(e) = client {
            return Err(DeboaError::Connection(ConnectionError::Udp {
                host: config
                    .host()
                    .to_string(),
                message: e.to_string(),
            }));
        }

        let (mut conn, send_request) = client.unwrap();

        spawn_worker(async move {
            future::poll_fn(|cx| conn.poll_close(cx)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        Ok(BaseHttpConnection::<Self::Sender, Self::ReqBody, Self::ResBody> {
            sender: send_request,
            req_body: PhantomData,
            res_body: PhantomData,
        })
    }

    async fn send_request(
        &mut self,
        request: Request<Self::ReqBody>,
    ) -> Result<Response<Self::ResBody>> {
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

        let (parts, _) = response
            .unwrap()
            .into_parts();

        let response = self
            .process_response(parts, recv_stream)
            .await?;

        Ok(response)
    }
}

impl crate::client::conn::udp::private::DeboaUdpConnectionSealed
    for BaseHttpConnection<Http3Request, BytesBody, DeboaBody>
{
}
