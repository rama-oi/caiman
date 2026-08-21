use crossterm::event::KeyCode;

use crate::app::App;

fn open_index(app: &mut App) {
    app.screen = crate::app::Screen::Index;
}

pub fn handle_settings_themes_input(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(':') => {
            app.command_mode = true;
        }
        KeyCode::Esc => {
            if app.command_mode {
                open_index(app);
                app.command_mode = false;
            }
        }
        KeyCode::Char('q') => {
            if app.command_mode {
                app.should_quit = true;
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
