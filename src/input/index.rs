use crossterm::event::KeyCode;

use crate::app::App;

fn open_settings(app: &mut App) {
    app.screen = crate::app::Screen::Settings;
}

fn select_next_layout(app: &mut App) {
    if app.layouts.is_empty() {
        return;
    }
    app.selected_layout = (app.selected_layout + 1) % app.layouts.len();
    app.layout_list_state.select(Some(app.selected_layout));
}

fn select_prev_layout(app: &mut App) {
    if app.layouts.is_empty() {
        return;
    }
    app.selected_layout = (app.selected_layout + app.layouts.len() - 1) % app.layouts.len();
    app.layout_list_state.select(Some(app.selected_layout));
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
        KeyCode::Down => select_next_layout(app),
        KeyCode::Up => select_prev_layout(app),
        _ => {}
    }
}
