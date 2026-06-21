use crate::tracks::Track;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

pub struct EditState {
    pub active_field: usize,
    pub editing: bool,
    pub current_input: String,
    pub fields: Vec<String>,
}

impl EditState {
    pub fn new(track: &Track) -> Self {
        let fields = vec![
            track.title.clone().unwrap_or_default(),
            track.artist.clone().unwrap_or_default(),
            track.album.clone().unwrap_or_default(),
            track.genre.clone().unwrap_or_default(),
            track.year.clone().unwrap_or_default(),
        ];

        Self {
            active_field: 0,
            editing: false,
            current_input: String::new(),
            fields,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<EditAction> {
        if self.editing {
            match key.code {
                KeyCode::Enter => {
                    self.fields[self.active_field] = self.current_input.clone();
                    self.editing = false;
                    self.current_input.clear();
                    None
                }
                KeyCode::Esc => {
                    self.editing = false;
                    self.current_input.clear();
                    None
                }
                KeyCode::Char(c) => {
                    self.current_input.push(c);
                    None
                }
                KeyCode::Backspace => {
                    self.current_input.pop();
                    None
                }
                _ => None,
            }
        } else {
            match key.code {
                KeyCode::Tab => {
                    self.active_field = (self.active_field + 1) % self.fields.len();
                    None
                }
                KeyCode::Enter => {
                    let updated_track = Track {
                        title: Some(self.fields[0].clone()).filter(|s| !s.is_empty()),
                        artist: Some(self.fields[1].clone()).filter(|s| !s.is_empty()),
                        album: Some(self.fields[2].clone()).filter(|s| !s.is_empty()),
                        genre: Some(self.fields[3].clone()).filter(|s| !s.is_empty()),
                        year: Some(self.fields[4].clone()).filter(|s| !s.is_empty()),
                        ..Default::default()
                    };
                    Some(EditAction::Save(updated_track))
                }
                KeyCode::Char('e') => {
                    self.start_editing();
                    None
                }
                KeyCode::Esc => Some(EditAction::Cancel),
                _ => None,
            }
        }
    }

    fn start_editing(&mut self) {
        self.editing = true;
        self.current_input = self.fields[self.active_field].clone();
    }
}

pub enum EditAction {
    Save(Track),
    Cancel,
}

impl Edit {
    pub fn draw(f: &mut Frame, track: &Track, active_field: usize) {
        let area = centered_rect(60, 70, f.size());

        let block = Block::default()
            .title(" Edit Metadata ")
            .borders(Borders::ALL);

        let inner = block.inner(area);
        let width = inner.width as usize;

        let fields = vec![
            ("Title", track.title.as_deref().unwrap_or("")),
            ("Artist", track.artist.as_deref().unwrap_or("")),
            ("Album", track.album.as_deref().unwrap_or("")),
            ("Genre", track.genre.as_deref().unwrap_or("")),
            ("Year", track.year.as_deref().unwrap_or("")),
        ];

        let mut lines: Vec<Line> = Vec::new();

        for (i, (label, value)) in fields.iter().enumerate() {
            let label_style = if i == active_field {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Cyan)
            };

            let value_style = if i == active_field {
                Style::default().fg(Color::Black).bg(Color::Yellow)
            } else {
                Style::default()
            };

            lines.push(Line::from(vec![
                Span::styled(format!("{label:<8}: "), label_style),
                Span::styled(format!("{value:<width$}", width = width - 10), value_style),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from("[Tab] Next field"));
        lines.push(Line::from("[Enter] Save"));
        lines.push(Line::from("[Esc] Cancel"));

        let paragraph = Paragraph::new(lines).alignment(Alignment::Left);

        f.render_widget(Clear, area);
        f.render_widget(block, area);
        f.render_widget(paragraph, inner);
    }
}

pub struct Edit;
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
