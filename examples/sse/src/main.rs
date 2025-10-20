use crossterm::event::{Event, KeyCode};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, List, Paragraph},
};

use ratatui_elm::{Task, Update};
#[allow(unused_imports)]
use tui_input::{backend::crossterm::EventHandler, Input};
use crate::ai::{ai, Message};

mod ai;

#[derive(Debug, Clone, Default)]
pub struct Model {
    input: Input,
    input_mode: InputMode,
    messages: Vec<String>,
    connection: Option<ai::Connection>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

fn main() {
    ratatui_elm::App::new(update, view)
        .subscription(ai())
        .run()
        .unwrap();
}

fn update(state: &mut Model, event: Update<ai::Event>) -> (Task<ai::Event>, bool) {
    let (task, should_render) = match event {
        Update::Terminal(term_event) => {
            let _event = term_event.clone();
            if let Event::Key(key_event) = term_event {
                match state.input_mode {
                    InputMode::Normal => match key_event.code {
                        KeyCode::Char('e') => {
                            start_editing(state);
                            (Task::None, true)
                        }
                        KeyCode::Char('q') => (Task::Quit, false), // exit
                        _ => (Task::None, false),
                    },
                    InputMode::Editing => match key_event.code {
                        KeyCode::Enter => {
                            push_message(state);
                            (Task::None, true)
                        }
                        KeyCode::Esc => {
                            stop_editing(state);
                            (Task::None, true)
                        }
                        _ => {
                            //state.input.handle_event(&term_event);
                            (Task::None, true)
                        }
                    },
                }
            } else {
                (Task::None, false)
            }
        }
        #[allow(clippy::collapsible_match)]
        Update::Message::<ai::Event>(message) => match message {
            ai::Event::MessageReceived(Message::User(message)) => {
                state.messages.push(message);
                (Task::None, true)
            }
            ai::Event::Connected(conn) => {
                state.messages.push("Connected to AI server".to_string());
                state.connection = Some(conn);
                (Task::None, true)
            }
            _ => (Task::None, false),
        },
        _ => (Task::None, false),
    };
    (task, should_render)
}

fn start_editing(state: &mut Model) {
    state.input_mode = InputMode::Editing
}

fn stop_editing(state: &mut Model) {
    state.input_mode = InputMode::Normal
}

fn push_message(state: &mut Model) {
    state.messages.push(state.input.value_and_reset());
    if let Some(conn) = &mut state.connection {
        conn.send(Message::User(state.messages.last().unwrap().clone()));
    }
}

fn view(state: &mut Model, frame: &mut ratatui::Frame) {
    let [messages_area, input_area] =
        Layout::vertical([Constraint::Fill(4), Constraint::Min(1)]).areas(frame.area());

    render_messages(frame, messages_area, state);
    render_input(frame, input_area, state);
}

fn render_messages(frame: &mut Frame, area: Rect, state: &mut Model) {
    let title = Line::from("Talk with AI");

    let block = Block::default()
        .title(title.centered())
        .borders(Borders::ALL);

    let list = List::new(
        state
            .messages
            .iter()
            .map(|message| Line::from(message.as_str()))
            .collect::<Vec<_>>(),
    )
    .block(block);

    frame.render_widget(list, area);
}

fn render_input(frame: &mut Frame, area: Rect, state: &mut Model) {
    // keep 2 for borders and 1 for cursor
    let width = area.width.max(3) - 3;
    let scroll = state.input.visual_scroll(width as usize);
    let style = match state.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Color::Yellow.into(),
    };
    let input = Paragraph::new(state.input.value())
        .style(style)
        .scroll((0, scroll as u16))
        .block(Block::bordered().title("Input"));
    frame.render_widget(input, area);

    if state.input_mode == InputMode::Editing {
        // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
        // end of the input text and one line down from the border to the input line
        let x = state.input.visual_cursor().max(scroll) - scroll + 1;
        frame.set_cursor_position((area.x + x as u16, area.y + 1))
    }
}
