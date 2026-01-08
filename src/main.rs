mod app;
mod tui;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;

    let mut app = App::new();
    let res = app.run(&mut terminal);

    tui::restore()?;
    res
}
