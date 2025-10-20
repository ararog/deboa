use crate::{
    InputMode,
    event::{AppEvent, EventHandler, LocalEvent},
};
use crossterm::event::Event;
use deboa::{Deboa, request::DeboaRequest, response::DeboaResponse};
use deboa_extras::http::serde::json::JsonBody;
use futures::StreamExt;
use http::header;
use ratatui::{DefaultTerminal, crossterm::event::KeyCode};
use serde::{Deserialize, Serialize};
use tui_input::{Input, backend::crossterm::EventHandler as _};

const API_KEY: &str = "YOUR_OPENAI_API_KEY";

/// Application.
#[derive(Debug)]
pub struct App {
    pub http_client: Deboa,
    pub input: Input,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub running: bool,
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            http_client: Deboa::new(),
            input: Input::default(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            running: true,
            events: EventHandler::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                LocalEvent::Tick => self.tick(),
                LocalEvent::Crossterm(event) => self.handle_key_events(event).await,
                LocalEvent::App(app_event) => match app_event {
                    AppEvent::MessageReceived(message) => self.messages.push(message),
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub async fn handle_key_events(&mut self, event: Event) {
        if let Event::Key(key_event) = event {
            match self.input_mode {
                InputMode::Normal => match key_event.code {
                    KeyCode::Char('e') => {
                        self.start_editing();
                    }
                    KeyCode::Char('q') => self.quit(), // exit
                    _ => {}
                },
                InputMode::Editing => match key_event.code {
                    KeyCode::Enter => {
                        self.push_message().await;
                    }
                    KeyCode::Esc => {
                        self.stop_editing();
                    }
                    _ => {
                        self.input.handle_event(&event);
                    }
                },
            }
        }
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn start_editing(&mut self) {
        self.input_mode = InputMode::Editing;
    }

    pub fn stop_editing(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub async fn push_message(&mut self) {
        let message = self.input.value_and_reset();
        self.messages.push(message.clone());

        let response = self.make_request(message.as_str()).await;
        let mut stream = response.unwrap().stream();
        while let Some(message) = stream.next().await {
            if let Ok(frame) = message {
                let text_message = String::from_utf8_lossy(frame.as_ref()).to_string();
                self.events.send(AppEvent::MessageReceived(text_message));
            }
        }
    }

    async fn make_request(&mut self, message: &str) -> Result<DeboaResponse, String> {
        let response = DeboaRequest::post("https://api.openai.com/v1/chat/completions")
            .unwrap()
            .bearer_auth(API_KEY)
            .header(header::CONTENT_TYPE, "application/json")
            .body_as(
                JsonBody,
                &Prompt {
                    model: "gpt-3.5-turbo".to_string(),
                    messages: vec![PromptMessage {
                        role: "user".to_string(),
                        content: message.to_string(),
                    }],
                },
            )
            .unwrap()
            .go(&mut self.http_client)
            .await;

        if let Err(message) = response {
            println!("Failed to connect to echo server: {}", message);
            Err(message.to_string())
        } else {
            Ok(response.unwrap())
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Prompt {
    pub model: String,
    pub messages: Vec<PromptMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct PromptMessage {
    pub role: String,
    pub content: String,
}
