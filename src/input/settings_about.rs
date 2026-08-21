use crossterm::event::KeyCode;

use crate::app::App;
use crate::router;

pub fn handle_settings_about_input(app: &mut App, key: KeyCode) {
    if app.command_mode {
        match key {
            KeyCode::Char('q') => {
                app.should_quit = true;
                app.command_mode = false;
            }
            KeyCode::Esc => {
                app.command_mode = false;
            }
            _ => {}
        }
        return;
    }

    match key {
        KeyCode::Char(':') => {
            app.command_mode = true;
        }
        KeyCode::Esc => router::go_back(app),
        KeyCode::Tab => {}
        KeyCode::Down => {}
        KeyCode::Up => {}
        _ => {}
    }
}
