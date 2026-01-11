use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

use crate::app::App;
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
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(f.size());

    let items: Vec<ListItem> = app
        .tracks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let prefix = if i == app.cursor { "▶ " } else { "  " };
            ListItem::new(format!("{prefix}{}", t.filename))
        })
        .collect();

    f.render_widget(
        List::new(items).block(Block::default().title("Tracks").borders(Borders::ALL)),
        chunks[0],
    );
    /* TODO create a help to draw and keys to move in the drawing  */
    f.render_widget(
        Block::default().title("Metadata").borders(Borders::ALL),
        chunks[1],
    );
}
