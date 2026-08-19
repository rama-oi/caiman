mod app;

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
        println!("caiman {}", env!("CARGO_PKG_VERSION"));
        return true;
    }

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "caiman {}\n\nA terminal keyboard layout switcher.\n\nUSAGE:\n    caiman [OPTIONS]\n\nOPTIONS:\n    -V, --version    Print version information\n    -h, --help       Print this help message",
            env!("CARGO_PKG_VERSION")
        );
        return true;
    }

    false
}
