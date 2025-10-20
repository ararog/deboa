use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, List, Paragraph, Widget},
};

use crate::{InputMode, app::App};

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [messages_area, input_area] =
            Layout::vertical([Constraint::Fill(4), Constraint::Min(1)]).areas(area);

        let title = Line::from("Talk with AI");

        let block = Block::default()
            .title(title.centered())
            .borders(Borders::ALL);

        let list = List::new(
            self.messages
                .iter()
                .map(|message| Line::from(message.as_str()))
                .collect::<Vec<_>>(),
        )
        .block(block);

        list.render(messages_area, buf);

        // keep 2 for borders and 1 for cursor
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Color::Yellow.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered().title("Input"));

        input.render(input_area, buf);
    }
}
