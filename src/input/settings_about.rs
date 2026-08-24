use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::input::command::handle_shortcut;
use crate::router;

pub fn handle_settings_about_input(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if let KeyCode::Char(c) = key.code {
            handle_shortcut(app, c.to_ascii_lowercase());
            return;
        }
    }

    match key.code {
        KeyCode::Esc => router::go_back(app),
        _ => {}
    }
}
