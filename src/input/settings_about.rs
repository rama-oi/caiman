use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::router;

pub fn handle_settings_about_input(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('q') {
        app.should_quit = true;
        return;
    }

    match key.code {
        KeyCode::Esc => router::go_back(app),
        _ => {}
    }
}
