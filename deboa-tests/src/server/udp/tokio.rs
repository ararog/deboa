use std::{net::SocketAddr, sync::Arc};

use bytes::Bytes;
use h3_quinn::quinn::{self, crypto::rustls::QuicServerConfig};
use http::{Request, Response};
use http_body_util::{BodyExt, Full};
use tokio::task::JoinHandle;

use crate::server::{BoxedError, Server, ServerConfig};
use crate::utils::generate_port;

pub struct HttpServer {
    port: u16,
    task: Option<JoinHandle<()>>,
    config: Option<ServerConfig>,
}

impl HttpServer {
    pub fn new(config: Option<ServerConfig>) -> Self {
        Self { port: generate_port(), task: None, config }
    }
}

impl Server<Full<Bytes>, Full<Bytes>> for HttpServer {
    type RequestFunction = fn(Request<Full<Bytes>>) -> Result<Response<Full<Bytes>>, hyper::Error>;

    fn port(&self) -> u16 {
        self.port
    }

    async fn start(&mut self, handler: Self::RequestFunction) -> Result<(), BoxedError> {
        if let Some(config) = &self.config {
            if config
                .cert
                .is_none()
                || config.key.is_none()
            {
                return Err("HttpServer - Server cert and key are required".into());
            }

            let tls_config = self.setup_tls(config.clone(), b"h3".to_vec())?;

            let server_config =
                quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(tls_config)?));
            let endpoint = quinn::Endpoint::server(
                server_config,
                SocketAddr::from(([127, 0, 0, 1], self.port)),
            )?;

            let handle = tokio::spawn(async move {
                while let Some(new_conn) = endpoint
                    .accept()
                    .await
                {
                    tokio::spawn(async move {
                        match new_conn.await {
                            Ok(conn) => {
                                let mut h3_conn =
                                    h3::server::Connection::new(h3_quinn::Connection::new(conn))
                                        .await
                                        .unwrap();

                                loop {
                                    match h3_conn
                                        .accept()
                                        .await
                                    {
                                        Ok(Some(resolver)) => {
                                            tokio::spawn(async move {
                                                let result = resolver
                                                    .resolve_request()
                                                    .await;
                                                if let Ok((req, mut stream)) = result {
                                                    let (parts, _) = req.into_parts();

                                                    let request = http::Request::from_parts(
                                                        parts,
                                                        Full::new(Bytes::new()),
                                                    );

                                                    let response = handler(request).expect(
                                                        "HttpServer - Could not process request!",
                                                    );

                                                    let resp = http::Response::builder()
                                                        .status(response.status())
                                                        .body(())
                                                        .unwrap();

                                                    match stream
                                                        .send_response(resp)
                                                        .await
                                                    {
                                                        Ok(_) => {
                                                            println!(
                                                            "successfully respond to connection"
                                                        );
                                                        }
                                                        Err(err) => {
                                                            eprintln!("unable to send response to connection peer: {:?}", err);
                                                        }
                                                    }

                                                    let collected = response
                                                        .collect()
                                                        .await;

                                                    let buf = Bytes::from(
                                                        collected
                                                            .expect("HttpServer - Failed to collect response")
                                                            .to_bytes()
                                                            .to_vec(),
                                                    );

                                                    stream
                                                        .send_data(buf)
                                                        .await;

                                                    stream
                                                        .finish()
                                                        .await;
                                                }
                                            });
                                        }
                                        Ok(None) => {
                                            break;
                                        }
                                        Err(err) => {
                                            eprintln!("error on accept {}", err);
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("accepting connection failed: {:?}", err);
                            }
                        }
                    });
                }

                endpoint
                    .wait_idle()
                    .await;
            });

            self.task = Some(handle);
        }

        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(task) = self.task.take() {
            task.abort();
        }
        Ok(())
    }
}
