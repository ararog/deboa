use std::sync::Arc;

use bytes::Bytes;
use h3_quinn::quinn::{self, crypto::rustls::QuicServerConfig};
use http::{Request, Response};
use http_body_util::Full;
use hyper::body;
use tokio::task::JoinHandle;

pub struct HttpServer {
    port: u16,
    task: Option<smol::Task<()>>,
    config: Option<ServerConfig>,
}

impl HttpServer {
    pub fn new(config: Option<ServerConfig>) -> Self {
        Self { port: rand::random_range(20000..65535), task: None, config }
    }
}

impl HttpServer {
    pub fn url(&self, path: &str) -> String {
        format!("http://localhost:{}{}", self.port, path)
    }

    pub async fn start(
        &mut self,
        handler: fn(Request<body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)?;

        tls_config.max_early_data_size = u32::MAX;
        tls_config.alpn_protocols = vec![b"h3".to_vec()];

        let server_config =
            quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(tls_config)?));
        let endpoint = quinn::Endpoint::server(server_config, opt.listen)?;

        let handle = smol::spawn(async move {
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
                                        tokio::spawn(async {
                                            //
                                        });
                                    }
                                    // indicating that the remote sent a goaway frame
                                    // all requests have been processed
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

        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}
