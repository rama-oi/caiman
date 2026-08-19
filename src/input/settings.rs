use crossterm::event::KeyCode;

use crate::app::App;

fn open_index(app: &mut App) {
    app.screen = crate::app::Screen::Index;
}

pub fn handle_settings_input(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            open_index(app);
        }
        // KeyCode::Space => {}
        KeyCode::Tab => {}
        KeyCode::Down => {}

        KeyCode::Up => {}
        _ => {}
    }
}
