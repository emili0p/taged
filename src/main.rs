use std::{env, io, path::PathBuf};
mod app;
mod edit;
mod help;
mod library;
mod tracks;
mod tui;
use app::App;
mod mode;
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

/*
* TODO
* make a menu to edit tracks when insert mode is pressed similar to the help menu
* then add a multi edit with macros using vline, example change all selected files artist to other
*
*/
