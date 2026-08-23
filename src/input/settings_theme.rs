use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::router;

fn preview_theme(app: &mut App, index: usize) {
    if index >= app.themes.len() {
        return;
    }

    app.theme_list_state.select(Some(index));

    // app.apply_theme(index);
}

fn select_next_theme(app: &mut App) {
    if app.themes.is_empty() {
        return;
    }

    let current = app
        .theme_list_state
        .selected()
        .unwrap_or(app.selected_theme);

    let next = (current + 1) % app.themes.len();

    preview_theme(app, next);
}

fn select_prev_theme(app: &mut App) {
    if app.themes.is_empty() {
        return;
    }

    let current = app
        .theme_list_state
        .selected()
        .unwrap_or(app.selected_theme);

    let prev = (current + app.themes.len() - 1) % app.themes.len();

    preview_theme(app, prev);
}

fn commit_theme(app: &mut App) {
    let Some(index) = app.theme_list_state.selected() else {
        return;
    };

    app.selected_theme = index;

    // app.save_theme();
}

pub fn handle_settings_themes_input(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('q') {
        // app.apply_theme(app.selected_theme);
        app.theme_list_state.select(Some(app.selected_theme));
        app.should_quit = true;
        return;
    }

    match key.code {
        KeyCode::Up => {
            select_prev_theme(app);
        }

        KeyCode::Down => {
            select_next_theme(app);
        }

        KeyCode::Enter => {
            commit_theme(app);
            router::go_back(app);
        }

        KeyCode::Esc => {
            // app.apply_theme(app.selected_theme);
            app.theme_list_state.select(Some(app.selected_theme));

            router::go_back(app);
        }

        _ => {}
    }
}
