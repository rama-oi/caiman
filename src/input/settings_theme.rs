use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::router;

fn select_next_theme(app: &mut App) {
    if app.themes.is_empty() {
        return;
    }

    let next = (app.selected_theme + 1) % app.themes.len();
    app.selected_theme = next;
    app.theme_list_state.select(Some(next));
}

fn select_prev_theme(app: &mut App) {
    if app.themes.is_empty() {
        return;
    }

    let prev = (app.selected_theme + app.themes.len() - 1) % app.themes.len();
    app.selected_theme = prev;
    app.theme_list_state.select(Some(prev));
}

pub fn handle_settings_themes_input(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('q') => {
                app.should_quit = true;
                return;
            }
            _ => {}
        }
    }

    match key.code {
        KeyCode::Esc => router::go_back(app),
        KeyCode::Down => select_next_theme(app),
        KeyCode::Up => select_prev_theme(app),
        _ => {}
    }
}
