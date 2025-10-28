use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{ready, Stream};
use hyper::body::Body;
use pin_project_lite::pin_project;

use crate::{errors::{DeboaExtrasError, SSEError}, http::sse::event::ServerEvent};

use deboa::{
    response::DeboaBody,
};

pin_project! {
    /// A data stream created from a [`Body`].
    #[derive(Debug)]
    pub struct ServerEventStream{
        #[pin]
        stream: DeboaBody,
    }
}

impl ServerEventStream {
    pub fn new(stream: DeboaBody) -> Self {
        Self { stream }
    }
}

impl Stream for ServerEventStream {
    type Item = Result<ServerEvent, DeboaExtrasError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            return match ready!(self.as_mut().project().stream.poll_frame(cx)) {
                Some(Ok(frame)) => match frame.into_data() {
                    Ok(bytes) => {
                        let event = ServerEvent::parse(&bytes);
                        match event {
                            Ok(event) => Poll::Ready(Some(Ok(event))),
                            Err(err) => Poll::Ready(Some(Err(err))),
                        }
                    }
                    Err(_) => continue,
                },
                Some(Err(err)) => Poll::Ready(Some(Err(DeboaExtrasError::SSE(SSEError::ReceiveEvent {
                    message: err.to_string(),
                })))),
                None => Poll::Ready(None),
            };
        }
    }
}
