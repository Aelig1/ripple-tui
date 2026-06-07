use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use std::{io::stdout, panic};

use crate::app::App;

pub mod app;
pub mod event;
pub mod pond;
pub mod stencil;
pub mod ui;

fn init() -> ratatui::DefaultTerminal {
    let terminal = ratatui::init();

    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        disable_mouse_capture();
        original_hook(info);
    }));
    enable_mouse_capture();

    terminal
}

fn restore() {
    disable_mouse_capture();
    ratatui::restore();
}

fn enable_mouse_capture() {
    execute!(stdout(), EnableMouseCapture).expect("failed to enable mouse capture");
}

fn disable_mouse_capture() {
    let _ = execute!(stdout(), DisableMouseCapture);
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let terminal = init();
    let result = App::new().run(terminal).await;
    restore();

    result
}
