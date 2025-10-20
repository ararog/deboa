//use ratatui_elm::{Task, Update};
#[allow(unused_imports)]
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::app::App;

mod app;
mod event;
mod ui;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
