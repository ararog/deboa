use bytes::Bytes;
use crate::errors::DeboaExtrasError;

#[derive(Debug)]
pub struct ServerEvent {
    id: Option<String>,
    event: Option<String>,
    data: Vec<String>,
    retry: Option<u64>,
}

impl ServerEvent {
    pub fn new() -> Self {
        Self {
            id: None,
            event: None,
            data: Vec::new(),
            retry: None,
        }
    }

    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    pub fn event(&self) -> &Option<String> {
        &self.event
    }

    pub fn data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn add_data(&mut self, data: String) {
        self.data.push(data);
    }

    pub fn retry(&self) -> &Option<u64> {
        &self.retry
    }

    pub fn parse(data: &Bytes) -> Result<ServerEvent, DeboaExtrasError> {
        let data = String::from_utf8_lossy(data.as_ref());
        let text_message = data;
        let lines = text_message.lines();
        let mut event = ServerEvent::new();
        for line in lines {
            if let Some(stripped) = line.strip_prefix("id: ") {
                event.id = Some(stripped.to_string());
            }

            if let Some(stripped) = line.strip_prefix("event: ") {
                event.event = Some(stripped.to_string());
            }

            if let Some(stripped) = line.strip_prefix("retry: ") {
                event.retry = Some(stripped.parse::<u64>().unwrap());
            }

            if let Some(stripped) = line.strip_prefix("data: ") {
                if stripped == "[DONE]" {
                    break;
                }

                event.add_data(stripped.to_string());
            }
        }

        Ok(event)
    }
}
