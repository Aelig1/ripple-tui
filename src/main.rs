use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use std::io::stdout;

use crate::app::App;

pub mod app;
pub mod event;
pub mod pond;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;

    let result = App::new().run(terminal).await;

    execute!(stdout(), DisableMouseCapture)?;
    ratatui::restore();

    result
}
