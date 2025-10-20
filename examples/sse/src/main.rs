use crossterm::event::{Event, KeyCode};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, List, Paragraph},
};

use ratatui_elm::{Task, Update};
use tui_input::Input;

use crate::ai::{Message, ai};

mod ai;

#[derive(Debug, Clone, Default)]
pub struct Model {
    input: Input,
    input_mode: InputMode,
    messages: Vec<String>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

fn main() {
    ratatui_elm::App::new(update, view)
        .run()
        .unwrap();
}

fn update(state: &mut Model, event: Update<ai::Event>) -> (Task<ai::Event>, bool) {
    let task = match event {
        Update::Terminal(Event::Key(e)) => match e.code {
            KeyCode::Char('q') | KeyCode::Esc => Task::Quit,
            _ => Task::None,
        },
        Update::Message::<ai::Event>(message) => match message {
            ai::Event::MessageReceived(Message::User(message)) => {
                state.messages.push(message);
                Task::None
            }
            _ => Task::None,
        },
        _ => Task::None,
    };
    (task, false)
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
