use crate::{
    alpn,
    cert::{DeboaCertificate, DeboaIdentity},
    client::http::conn::{
        stream::{plain_connection, tls_connection},
        BaseHttpConnection,
    },
    Result, MAX_ERROR_MESSAGE_SIZE,
};
use cyper_core::CompioExecutor;
use deboa::{
    conn::{ConnectionConfig, HttpConnection, ProtoConnection},
    errors::{DeboaError, RequestError, ResponseError},
    request::Http2Request,
};
use http::{version::Version, StatusCode};
use http_body_util::BodyExt;
use hyper::{body::Incoming, client::conn::http2::handshake, Request, Response};
use hyper_body_utils::HttpBody;
use std::future::Future;

impl BaseHttpConnection<Http2Request, HttpBody, HttpBody> {
    /// Process a response.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to connect.
    /// * `method` - The method to use.
    /// * `response` - The response to process.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Incoming>>` - The response or error.
    ///
    fn process_response(
        &self,
        _method: &str,
        response: std::result::Result<Response<Incoming>, hyper::Error>,
    ) -> impl Future<Output = Result<Response<HttpBody>>> + Send {
        async {
            if let Err(err) = response {
                return Err(DeboaError::Request(RequestError::Send { message: err.to_string() }));
            }

            let response = response.unwrap();
            let status_code = response.status();
            if (!status_code.is_success()
                && !status_code.is_informational()
                && !status_code.is_redirection())
                || status_code == StatusCode::TOO_MANY_REQUESTS
            {
                let mut body = response.into_body();
                let mut error_message = Vec::new();
                let mut downloaded = 0;
                while let Some(chunk) = body.frame().await {
                    if let Ok(frame) = chunk {
                        if let Some(data) = frame.data_ref() {
                            if downloaded + data.len() > MAX_ERROR_MESSAGE_SIZE {
                                break;
                            }
                            error_message.extend_from_slice(data);
                            downloaded += data.len();
                        }
                    }
                }
                return Err(DeboaError::Response(ResponseError::Receive {
                    status_code,
                    message: format!(
                        "Could not process request ({}): {}",
                        status_code,
                        String::from_utf8_lossy(&error_message)
                    ),
                }));
            }

            let (parts, body) = response.into_parts();
            let response = Response::from_parts(parts, HttpBody::from_incoming(body));

            Ok(response)
        }
    }
}

impl HttpConnection for BaseHttpConnection<Http2Request, HttpBody, HttpBody> {
    type Sender = Http2Request;
    type ReqBody = HttpBody;
    type ResBody = HttpBody;

    fn sender(&mut self) -> &mut Self::Sender {
        &mut self.sender
    }
}

impl ProtoConnection for BaseHttpConnection<Http2Request, HttpBody, HttpBody> {
    type ReqBody = HttpBody;
    type ResBody = HttpBody;
    type Connection = BaseHttpConnection<Http2Request, HttpBody, HttpBody>;
    type Identity = DeboaIdentity;
    type Certificate = DeboaCertificate;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_2
    }

    async fn connect<'a>(
        config: &ConnectionConfig<'a, Self::Identity, Self::Certificate>,
    ) -> Result<Self::Connection> {
        let stream = if config.is_secure() {
            tls_connection(
                *config.ip(),
                config.host(),
                config.port(),
                config.identity(),
                config.certificate(),
                config.skip_cert_verification(),
                alpn(),
            )
            .await
        } else {
            plain_connection(*config.ip(), config.host(), config.port()).await
        };

        if let Err(e) = stream {
            return Err(e);
        }

        let result = handshake(CompioExecutor::default(), stream.unwrap()).await;

        let (sender, conn) = result.unwrap();

        compio::runtime::spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(_err) => {}
            };
        })
        .detach();

        Ok(BaseHttpConnection::new(sender))
    }

    async fn send_request(
        &mut self,
        request: Request<Self::ReqBody>,
    ) -> Result<Response<Self::ResBody>> {
        let method = request
            .method()
            .to_string();

        let result = self
            .sender
            .send_request(request)
            .await;

        self.process_response(&method, result)
            .await
    }
}
