mod app;
mod config;
mod input;
mod router;
mod theme;
mod ui;
mod util;

use std::io::{self, stdout};

use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use crossterm::terminal::supports_keyboard_enhancement;
use std::process::Command;

fn main() -> io::Result<()> {
    if handle_cli_flags() {
        return Ok(());
    }

    let mut terminal = ratatui::init();

    let enhancement_enabled = supports_keyboard_enhancement().unwrap_or(false);
    if enhancement_enabled {
        let _ = execute!(
            stdout(),
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
            )
        );
    }

    let result = app::run(&mut terminal);

    if enhancement_enabled {
        let _ = execute!(stdout(), PopKeyboardEnhancementFlags);
    }

    ratatui::restore();

    result
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

    if args.iter().any(|a| a == "--help" || a == "-H" || a == "-h") {
        println!(
            "{} {}\n{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_DESCRIPTION")
        );
        return true;
    }

    false
}
