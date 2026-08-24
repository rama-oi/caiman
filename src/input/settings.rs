use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::input::command::handle_shortcut;
use crate::router;
use crate::ui::settings::{ITEM_ABOUT_CAIMAN, ITEM_COUNT, ITEM_SWITCH_THEME};

fn select_next_item(app: &mut App) {
    let selected = app.settings_list_state.selected().unwrap_or(0);
    let next = (selected + 1) % ITEM_COUNT;
    app.settings_list_state.select(Some(next));
}

fn select_prev_item(app: &mut App) {
    let selected = app.settings_list_state.selected().unwrap_or(0);
    let prev = (selected + ITEM_COUNT - 1) % ITEM_COUNT;
    app.settings_list_state.select(Some(prev));
}

fn open_selected_item(app: &mut App) {
    match app.settings_list_state.selected() {
        Some(ITEM_SWITCH_THEME) => router::go_to_themes(app),
        Some(ITEM_ABOUT_CAIMAN) => router::go_to_about(app),
        _ => {}
    }
}

pub fn handle_settings_input(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if let KeyCode::Char(c) = key.code {
            handle_shortcut(app, c.to_ascii_lowercase());
            return;
        }
    }

    match key.code {
        KeyCode::Esc => router::go_back(app),
        KeyCode::Down => select_next_item(app),
        KeyCode::Up => select_prev_item(app),
        KeyCode::Enter => open_selected_item(app),
        _ => {}
    }
}
