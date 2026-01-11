use crate::library;
use crate::tracks::Track;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};
use std::io;
use std::path::PathBuf;

pub struct App {
    pub running: bool,
    pub tracks: Vec<Track>,
    pub cursor: usize,
}

impl App {
    pub fn new() -> Self {
        let tracks = library::load_directory(PathBuf::from(".").as_path()).unwrap_or_default();
        Self {
            running: true,
            tracks,
            cursor: 0,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while self.running {
            terminal.draw(|f| crate::tui::draw(f, self))?;

            if event::poll(std::time::Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => self.running = false,
                        KeyCode::Up => {
                            if self.cursor > 0 {
                                self.cursor -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.cursor + 1 < self.tracks.len() {
                                self.cursor += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
