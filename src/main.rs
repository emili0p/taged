use std::{env, io, path::PathBuf};

mod app;
mod library;
mod tracks;
mod tui;
use app::App;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut app = App::new(dir);
    let res = app.run(&mut terminal);

    tui::restore()?;
    res
}
