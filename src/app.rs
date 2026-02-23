use crate::mode::Mode;
use crate::tracks::Track;
use crate::{help, library};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};

use std::io;
use std::path::PathBuf;
use std::time::Duration;

pub struct App {
    pub running: bool,
    pub show_help: bool,
    pub tracks: Vec<Track>,
    pub cursor: usize,
    pub mode: Mode,
}

impl App {
    pub fn new(dir: PathBuf) -> Self {
        let tracks = library::load_directory(&dir).unwrap_or_default();

        Self {
            running: true,
            show_help: false,
            tracks,
            cursor: 0,
            mode: Mode::Normal,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while self.running {
            terminal.draw(|f| {
                crate::tui::draw(f, self);

                if self.show_help {
                    help::Help::draw(f);
                }
            })?;

            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key.code);
                }
            }
        }

        Ok(())
    }
    fn handle_key(&mut self, key: KeyCode) {
        if self.show_help {
            match key {
                KeyCode::Esc | KeyCode::Char('?') => {
                    self.show_help = false;
                }
                _ => {}
            }
            return;
        }

        match self.mode {
            Mode::Normal => self.handle_normal_mode(key),
            Mode::Insert => self.handle_insert_mode(key),
            Mode::Visual => self.handle_visual_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('i') | KeyCode::Char('a') => {
                self.mode = Mode::Insert;
            }

            KeyCode::Char('v') => {
                self.mode = Mode::Visual;
            }

            KeyCode::Char('?') => {
                self.show_help = !self.show_help;
            }

            KeyCode::Char('q') => {
                if !self.show_help {
                    self.running = false;
                }
            }

            KeyCode::Char('k') | KeyCode::Up => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }

            KeyCode::Char('j') | KeyCode::Down => {
                if self.cursor + 1 < self.tracks.len() {
                    self.cursor += 1;
                }
            }

            _ => {}
        }
    }

    fn handle_insert_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
            }

            _ => {}
        }
    }

    fn handle_visual_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
            }

            KeyCode::Char('k') | KeyCode::Up => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }

            KeyCode::Char('j') | KeyCode::Down => {
                if self.cursor + 1 < self.tracks.len() {
                    self.cursor += 1;
                }
            }

            _ => {}
        }
    }
}
