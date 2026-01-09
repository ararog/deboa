use std::sync::Arc;

use bytes::Bytes;
use h3_quinn::quinn::{self, crypto::rustls::QuicServerConfig};
use http::{Request, Response};
use http_body_util::Full;
use hyper::body;
use tokio::task::JoinHandle;

use crate::server::ServerConfig;
use crate::utils::{generate_port, TEST_HOST};

pub struct HttpServer {
    port: u16,
    task: Option<smol::Task<()>>,
    config: Option<ServerConfig>,
}

impl HttpServer {
    pub fn new(config: Option<ServerConfig>) -> Self {
        Self { port: generate_port(), task: None, config }
    }
}

impl HttpServer {
    pub fn url(&self, path: &str) -> String {
        format!("{}:{}{}", TEST_HOST, self.port, path)
    }

    pub fn base_url(&self) -> String {
        format!("{}:{}", TEST_HOST, self.port)
    }

    pub async fn start(
        &mut self,
        handler: fn(Request<body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self
            .config
            .is_none()
        {
            return Err("Server config is required".into());
        }

        if let Some(config) = &self.config {
            if config
                .cert
                .is_none()
                || config.key.is_none()
            {
                return Err("Server cert and key are required".into());
            }

            let cert = CertificateDer::from(std::fs::read(
                config
                    .cert
                    .as_ref()
                    .unwrap(),
            )?);
            let key = PrivateKeyDer::try_from(std::fs::read(
                config
                    .key
                    .as_ref()
                    .unwrap(),
            )?)?;

            let provider = rustls::crypto::aws_lc_rs::default_provider();
            let mut tls_config = rustls::ServerConfig::builder_with_provider(Arc::new(provider))
                .with_protocol_versions(&[&rustls::version::TLS13])
                .expect("Failed to set TLS version")
                .with_no_client_auth()
                .with_single_cert(vec![cert], key)?;

            tls_config.max_early_data_size = u32::MAX;
            tls_config.alpn_protocols = vec![b"h3".to_vec()];

            let server_config =
                quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(tls_config)?));
            let endpoint = quinn::Endpoint::server(
                server_config,
                SocketAddr::from(([127, 0, 0, 1], self.port)),
            )?;

            let handle = smol::spawn(async move {
                while let Some(new_conn) = endpoint
                    .accept()
                    .await
                {
                    smol::spawn(async move {
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
                                            smol::spawn(async move {
                                                let result = resolver
                                                    .resolve_request()
                                                    .await;
                                                if let Ok((req, mut stream)) = result {
                                                    let (parts, _) = req.into_parts();

                                                    let request = http::Request::from_parts(
                                                        parts,
                                                        Full::new(Bytes::new()),
                                                    );

                                                    let response = handler(request)
                                                        .expect("Could not process request!");

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
                                                            .expect("Failed to collect response")
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

    pub async fn stop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}
