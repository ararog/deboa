#[derive(Debug)]
pub struct ServerEvent {
    event: Option<String>,
    data: String,
}

impl ServerEvent {
    pub fn new(data: String) -> Self {
        Self { event: None, data }
    }

    pub fn event(&self) -> &Option<String> {
        &self.event
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}
