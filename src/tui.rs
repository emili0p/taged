use crate::io;
use crate::{app::App, mode::Mode};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
/*  this creates the tui */
pub fn init() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
pub fn draw(f: &mut Frame, app: &App) {
    // Layout principal (vertical)
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.size());

    // Layout interno horizontal
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(layout[0]);

    let (_mode_text, _style) = match app.mode {
        Mode::Normal => (
            "-- NORMAL --",
            ratatui::style::Style::default().fg(ratatui::style::Color::Green),
        ),
        Mode::Insert => (
            "-- INSERT --",
            ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
        ),
        Mode::Visual => (
            "-- VISUAL --",
            ratatui::style::Style::default().fg(ratatui::style::Color::Magenta),
        ),
    };

    let total = app.tracks.len();
    let current = if total == 0 { 0 } else { app.cursor + 1 };

    let (mode_label, mode_style) = match app.mode {
        Mode::Normal => (
            " NORMAL ",
            Style::default().fg(Color::Black).bg(Color::Green),
        ),
        Mode::Insert => (
            " INSERT ",
            Style::default().fg(Color::Black).bg(Color::Yellow),
        ),
        Mode::Visual => (
            " VISUAL ",
            Style::default().fg(Color::Black).bg(Color::Magenta),
        ),
    };

    let status_line = Line::from(vec![
        Span::styled(mode_label, mode_style),
        Span::raw(" | "),
        Span::raw(format!("{total} tracks")),
        Span::raw(" | "),
        Span::raw(format!("{current}/{total}")),
    ]);

    let mode_bar = Paragraph::new(status_line).alignment(Alignment::Left);
    let items: Vec<ListItem> = app
        .tracks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let prefix = if i == app.cursor { "▶ " } else { "  " };
            ListItem::new(format!("{prefix}{}", t.filename))
        })
        .collect();

    let tracks_widget =
        List::new(items).block(Block::default().title("Tracks").borders(Borders::ALL));

    let metadata = if let Some(track) = app.tracks.get(app.cursor) {
        format!(
            "Filename: {}\n\nPath: {}\n\nArtist: {}\nAlbum: {}\nDuration: {}",
            track.filename,
            track.path.display(),
            track.artist.as_deref().unwrap_or("Unknown"),
            track.album.as_deref().unwrap_or("Unknown"),
            track.duration.as_deref().unwrap_or("Unknown"),
        )
    } else {
        String::from("No track selected")
    };

    let metadata_widget =
        Paragraph::new(metadata).block(Block::default().title("Metadata").borders(Borders::ALL));

    f.render_widget(tracks_widget, main_chunks[0]);
    f.render_widget(metadata_widget, main_chunks[1]);
    f.render_widget(mode_bar, layout[1]);
}
