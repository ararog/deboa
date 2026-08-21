#[cfg(feature = "generic")]
pub mod generic {

    use bytes::Bytes;
    use h3::{client::RequestStream, error::StreamError};
    use h3_quinn::{OpenStreams, RecvStream};
    use http::{Request, Response};
    use http_body_util::BodyExt as _;
    use hyper_body_utils::HttpBody;
    use std::marker::PhantomData;

    pub type QuicRequest = h3::client::SendRequest<OpenStreams, Bytes>;
    pub type Http3Request = SendRequest<QuicRequest, HttpBody>;
    pub type HttpStream = RequestStream<RecvStream, Bytes>;

    pub struct SendRequest<Sender, Body> {
        sender: Sender,
        _p: PhantomData<Body>,
    }

    impl SendRequest<QuicRequest, HttpBody> {
        pub fn new(sender: QuicRequest) -> Self {
            Self { sender, _p: PhantomData }
        }

        pub async fn send_request(
            &mut self,
            request: http::Request<HttpBody>,
        ) -> std::result::Result<http::Response<HttpBody>, StreamError> {
            let mut sender = self.sender.clone();

            let (parts, mut body) = request.into_parts();

            let bodyless_request = Request::from_parts(parts, ());

            let request_stream = sender
                .send_request(bodyless_request)
                .await?;

            let (mut send_stream, mut recv_stream) = request_stream.split();

            while let Some(chunk) = body.frame().await {
                let frame = chunk.unwrap();
                if let Some(bytes) = frame.data_ref() {
                    send_stream
                        .send_data(bytes.clone())
                        .await?;
                }
            }

            send_stream
                .finish()
                .await?;

            let response = recv_stream
                .recv_response()
                .await?;

            let (parts, _) = response.into_parts();

            let body = HttpBody::from_generic_client(recv_stream);
            let response = Response::from_parts(parts, body);
            Ok(response)
        }
    }
}

#[cfg(feature = "compio")]
pub mod compio {
    use bytes::Bytes;
    use compio_quic::{h3::OpenStreams, RecvStream};
    use h3::{client::RequestStream, error::StreamError};
    use http::{Request, Response};
    use http_body_util::BodyExt as _;
    use hyper_body_utils::HttpBody;
    use std::marker::PhantomData;

    pub type QuicRequest = h3::client::SendRequest<OpenStreams, Bytes>;
    pub type Http3Request = SendRequest<QuicRequest, HttpBody>;
    pub type HttpStream = RequestStream<RecvStream, Bytes>;

    pub struct SendRequest<Sender, Body> {
        sender: Sender,
        _p: PhantomData<Body>,
    }

    impl SendRequest<QuicRequest, HttpBody> {
        pub fn new(sender: QuicRequest) -> Self {
            Self { sender, _p: PhantomData }
        }

        pub async fn send_request(
            &mut self,
            request: http::Request<HttpBody>,
        ) -> std::result::Result<http::Response<HttpBody>, StreamError> {
            let mut sender = self.sender.clone();

            let (parts, mut body) = request.into_parts();

            let bodyless_request = Request::from_parts(parts, ());

            let request_stream = sender
                .send_request(bodyless_request)
                .await?;

            let (mut send_stream, mut recv_stream) = request_stream.split();

            while let Some(chunk) = body.frame().await {
                let frame = chunk.unwrap();
                if let Some(bytes) = frame.data_ref() {
                    send_stream
                        .send_data(bytes.clone())
                        .await?;
                }
            }

            send_stream
                .finish()
                .await?;

            let response = recv_stream
                .recv_response()
                .await?;

            let (parts, _) = response.into_parts();

            let body = HttpBody::from_compio_client(recv_stream);
            let response = Response::from_parts(parts, body);
            Ok(response)
        }
    }
}
