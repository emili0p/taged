use crate::tracks::Track;
use crate::{help, library};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};
use std::io;
use std::path::PathBuf;

pub struct App {
    pub running: bool,
    pub show_help: bool,
    pub tracks: Vec<Track>,
    pub cursor: usize,
}

impl App {
    pub fn new(dir: PathBuf) -> Self {
        let tracks = library::load_directory(&dir).unwrap_or_default();

        Self {
            running: true,
            show_help: false,
            tracks,
            cursor: 0,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while self.running {
            terminal.draw(|f| {
                crate::tui::draw(f, self);

                // Dibujar help encima si está activo
                if self.show_help {
                    help::Help::draw(f);
                }
            })?;

            if event::poll(std::time::Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('?') => {
                            self.show_help = !self.show_help;
                        }

                        KeyCode::Esc => {
                            if self.show_help {
                                self.show_help = false;
                            }
                        }

                        KeyCode::Char('q') => {
                            if !self.show_help {
                                self.running = false;
                            }
                        }

                        KeyCode::Up => {
                            if !self.show_help && self.cursor > 0 {
                                self.cursor -= 1;
                            }
                        }

                        KeyCode::Down => {
                            if !self.show_help && self.cursor + 1 < self.tracks.len() {
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
