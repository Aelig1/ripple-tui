use crate::{
    event::{AppEvent, Event, EventHandler},
    pond::Pond,
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEvent, MouseEventKind};
use ratatui::DefaultTerminal;
use std::time::Duration;
use tokio::time::MissedTickBehavior;

/// The frequency at which the application state is updated.
const TICK_FPS: f64 = 30.0;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Rippling pond.
    pub pond: Pond,
    /// Event handler.
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            pond: Pond::default(),
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
        let size = terminal.size()?;
        self.pond.resize(size.width, size.height);

        let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut tick = tokio::time::interval(tick_rate);
        tick.set_missed_tick_behavior(MissedTickBehavior::Skip);

        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            tokio::select! {
                _ = tick.tick() => self.tick(),
                event = self.events.next() => match event? {
                    Event::Crossterm(event) => match event {
                        crossterm::event::Event::Key(key_event) => {
                            self.handle_key_events(key_event)?
                        }
                        crossterm::event::Event::Mouse(mouse_event) => {
                            self.handle_mouse_events(mouse_event)?
                        }
                        crossterm::event::Event::Resize(width, height) => {
                            self.pond.resize(width, height);
                        }
                        _ => {}
                    },
                    Event::App(app_event) => match app_event {
                        AppEvent::Droplet { x, y } => self.pond.droplet(x, y),
                        AppEvent::Quit => self.quit(),
                    },
                }
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    fn handle_mouse_events(&mut self, mouse_event: MouseEvent) -> color_eyre::Result<()> {
        if matches!(mouse_event.kind, MouseEventKind::Down(_)) {
            self.events.send(AppEvent::Droplet {
                x: mouse_event.column,
                y: mouse_event.row,
            });
        }
        // TODO: Other kinds of splashes for Drag event.
        Ok(())
    }

    /// Updates the application state for one tick.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        self.pond.tick();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
