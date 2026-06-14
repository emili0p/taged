use crate::mode::Mode;
use crate::tracks::Track;
use crate::{edit, help, library};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
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
    pub edit_field: usize,
    pub edit_state: Option<edit::EditState>,
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
            edit_field: 0,
            edit_state: None, // Inicializar como None
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

        if let Some(edit_state) = &mut self.edit_state {
            let key_event = KeyEvent::from(key);
            if let Some(action) = edit_state.handle_key(key_event) {
                match action {
                    edit::EditAction::Save(updated_track) => {
                        // Guardar los cambios
                        self.tracks[self.cursor] = updated_track;
                        self.edit_state = None;
                        self.mode = Mode::Normal;
                        self.edit_field = 0;
                    }
                    edit::EditAction::Cancel => {
                        // Cancelar edición
                        self.edit_state = None;
                        self.mode = Mode::Normal;
                        self.edit_field = 0;
                    }
                }
            }
            return;
        }

        // Si no estamos editando, manejar modos normales
        match self.mode {
            Mode::Normal => self.handle_normal_mode(key),
            Mode::Insert => self.handle_insert_mode(key),
            Mode::Visual => self.handle_visual_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('i') => {
                self.mode = Mode::Insert;
            }

            KeyCode::Char('e') => {
                // Tecla para editar el track actual
                if let Some(track) = self.tracks.get(self.cursor) {
                    self.edit_state = Some(edit::EditState::new(track));
                    // No cambiamos Mode::Insert porque el editor tiene su propio modo
                }
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

            KeyCode::Char('e') => {
                if let Some(track) = self.tracks.get(self.cursor) {
                    self.edit_state = Some(edit::EditState::new(track));
                }
            }

            KeyCode::Tab => {
                self.edit_field = (self.edit_field + 1) % 5;
            }

            KeyCode::BackTab => {
                if self.edit_field == 0 {
                    self.edit_field = 4;
                } else {
                    self.edit_field -= 1;
                }
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
