use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};
pub struct Help;
impl Help {
    pub fn draw(f: &mut Frame) {
        let area = centered_rect(60, 60, f.size());

        let text = vec![
            Line::from("Taged - Help"),
            Line::from(""),
            Line::from("?       Open this window"),
            Line::from("q       Quit the app"),
            Line::from("↑ ↓       Navigation"),
            Line::from("Enter       Edit Metadata"),
            Line::from(""),
            Line::from("Esc       Close this window"),
        ];

        let block = Block::default().title("Help").borders(Borders::ALL);

        let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Left);

        f.render_widget(Clear, area); //clears background
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
