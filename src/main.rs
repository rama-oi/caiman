mod app;
mod config;
mod input;
mod router;
mod theme;
mod ui;
mod util;

use std::io;

fn main() -> io::Result<()> {
    if handle_cli_flags() {
        return Ok(());
    }

    let mut terminal = ratatui::init();

    app::run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}

fn handle_cli_flags() -> bool {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args
        .iter()
        .any(|a| a == "--version" || a == "-V" || a == "-v")
    {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return true;
    }

    false
}
