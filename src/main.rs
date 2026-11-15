mod app;
mod ui;

use std::io;

fn main() -> io::Result<()> {
    ratatui::run(|mut terminal| app::App::default().run(&mut terminal))
}
