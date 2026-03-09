use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::tracks::Track;

pub struct Edit;

impl Edit {
    pub fn draw(f: &mut Frame, track: &Track, active_field: usize) {
        let area = centered_rect(60, 70, f.size());

        let fields = vec![
            ("Title", track.title.as_deref().unwrap_or("")),
            ("Artist", track.artist.as_deref().unwrap_or("")),
            ("Album", track.album.as_deref().unwrap_or("")),
            ("Genre", track.genre.as_deref().unwrap_or("")),
            ("Year", track.year.as_deref().unwrap_or("")),
        ];

        let mut lines: Vec<Line> = Vec::new();

        for (i, (label, value)) in fields.iter().enumerate() {
            let style = if i == active_field {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };

            lines.push(Line::from(vec![
                Span::styled(format!("{label:<8}: "), Style::default().fg(Color::Cyan)),
                Span::styled(value.to_string(), style),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from("[Tab] Next field"));
        lines.push(Line::from("[Enter] Save"));
        lines.push(Line::from("[Esc] Cancel"));

        let block = Block::default()
            .title("Edit Metadata")
            .borders(Borders::ALL);

        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }
}

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
