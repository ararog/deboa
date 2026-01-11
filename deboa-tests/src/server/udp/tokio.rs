use std::{net::SocketAddr, sync::Arc};

use bytes::Bytes;
use h3_quinn::quinn::{self, crypto::rustls::QuicServerConfig};
use http::{Request, Response};
use http_body_util::{BodyExt, Full};
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio::task::JoinHandle;

use crate::server::ServerConfig;
use crate::utils::{generate_port, test_url};

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

impl HttpServer {
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", test_url(Some(self.port)), path)
    }

    pub fn base_url(&self) -> String {
        test_url(Some(self.port))
    }

    pub async fn start(
        &mut self,
        handler: fn(Request<Full<Bytes>>) -> Result<Response<Full<Bytes>>, hyper::Error>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self
            .config
            .is_none()
        {
            return Err("HttpServer - Server config is required".into());
        }

        if let Some(config) = &self.config {
            if config
                .cert
                .is_none()
                || config.key.is_none()
            {
                return Err("HttpServer - Server cert and key are required".into());
            }

            let cert = config
                .cert()
                .unwrap()
                .clone();
            let key = config
                .key()
                .unwrap()
                .clone();

            let cert = CertificateDer::from_pem_slice(&cert);
            if let Err(e) = cert {
                eprintln!("HttpServer - Error loading cert: {}", e);
                return Err(e.into());
            }

            let cert = cert.unwrap();

            let key = PrivateKeyDer::from_pem_slice(&key);
            if let Err(e) = key {
                eprintln!("HttpServer - Error loading private key: {}", e);
                return Err(e.into());
            }

            let key = key.unwrap();

            let provider = rustls::crypto::aws_lc_rs::default_provider();
            let mut tls_config = rustls::ServerConfig::builder_with_provider(Arc::new(provider))
                .with_protocol_versions(&[&rustls::version::TLS13])
                .expect("HttpServer -Failed to set TLS version")
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

    pub async fn stop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}
