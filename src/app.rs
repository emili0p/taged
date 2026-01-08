use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};
use std::io;

pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { running: true }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while self.running {
            terminal.draw(|f| crate::tui::draw(f, self))?;

            if event::poll(std::time::Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => self.running = false,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
