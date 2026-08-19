use crossterm::event::KeyCode;

use crate::app::App;

fn open_settings(app: &mut App) {
    app.screen = crate::app::Screen::Settings;
}

pub fn handle_index_input(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(':') => {
            app.command_mode = true;
        }
        KeyCode::Char('q') => {
            if app.command_mode {
                app.should_quit = true;
                app.command_mode = false;
            }
        }
        KeyCode::Char('s') => {
            if app.command_mode {
                open_settings(app);
                app.command_mode = false;
            }
        }
        // KeyCode::Space => {}
        KeyCode::Tab => {}
        KeyCode::Down => {}

        KeyCode::Up => {}
        _ => {}
    }
}
