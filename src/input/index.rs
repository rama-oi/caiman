use crossterm::event::KeyCode;

use crate::app::App;
use crate::router;

fn select_next_layout(app: &mut App) {
    let indices = app.filtered_layout_indices();
    if indices.is_empty() {
        return;
    }

    let pos = indices
        .iter()
        .position(|&i| i == app.selected_layout)
        .unwrap_or(0);
    let next_pos = (pos + 1) % indices.len();

    app.selected_layout = indices[next_pos];
    app.layout_list_state.select(Some(next_pos));
}

fn select_prev_layout(app: &mut App) {
    let indices = app.filtered_layout_indices();
    if indices.is_empty() {
        return;
    }

    let pos = indices
        .iter()
        .position(|&i| i == app.selected_layout)
        .unwrap_or(0);
    let prev_pos = (pos + indices.len() - 1) % indices.len();

    app.selected_layout = indices[prev_pos];
    app.layout_list_state.select(Some(prev_pos));
}

fn refresh_filtered_selection(app: &mut App) {
    let indices = app.filtered_layout_indices();

    if indices.is_empty() {
        app.layout_list_state.select(None);
        return;
    }

    match indices.iter().position(|&i| i == app.selected_layout) {
        Some(pos) => app.layout_list_state.select(Some(pos)),
        None => {
            app.selected_layout = indices[0];
            app.layout_list_state.select(Some(0));
        }
    }
}

pub fn handle_index_input(app: &mut App, key: KeyCode) {
    if app.command_mode {
        match key {
            KeyCode::Char('q') => {
                app.should_quit = true;
                app.command_mode = false;
            }
            KeyCode::Char('s') => {
                router::go_to_settings(app);
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
        KeyCode::Tab => {}
        KeyCode::Down => select_next_layout(app),
        KeyCode::Up => select_prev_layout(app),
        KeyCode::Backspace => {
            app.search_query.pop();
            refresh_filtered_selection(app);
        }
        KeyCode::Esc => {
            app.search_query.clear();
            refresh_filtered_selection(app);
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            refresh_filtered_selection(app);
        }
        _ => {}
    }
}
