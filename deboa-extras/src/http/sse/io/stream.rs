use std::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures::{ready, Stream};
use hyper::body::Body;
use pin_project_lite::pin_project;

use crate::http::sse::event::ServerEvent;

use deboa::response::DeboaBody;
use deboa::Result;

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
    type Item = Result<Vec<ServerEvent>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        fn parse_event(data: &Bytes) -> Result<Vec<ServerEvent>> {
            let data = String::from_utf8_lossy(data.as_ref());
            let text_message = data;
            let lines = text_message.lines();
            let mut events = Vec::new();
            for line in lines {
                if let Some(stripped) = line.strip_prefix("data: ") {
                    if stripped == "[DONE]" {
                        break;
                    }

                    events.push(ServerEvent::new(stripped.to_string()));
                }
            }

            Ok(events)
        }

        loop {
            return match ready!(self.as_mut().project().stream.poll_frame(cx)) {
                Some(Ok(frame)) => match frame.into_data() {
                    Ok(bytes) => Poll::Ready(Some(parse_event(&bytes))),
                    Err(_) => continue,
                },
                Some(Err(err)) => Poll::Ready(Some(Err(deboa::errors::DeboaError::SSE {
                    message: err.to_string(),
                }))),
                None => Poll::Ready(None),
            };
        }
    }
}
