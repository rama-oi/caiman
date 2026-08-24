use crate::app::{App, Screen};
use crate::router;

pub fn handle_shortcut(app: &mut App, c: char) {
    match c {
        'q' => quit(app),
        's' => router::go_to_settings(app),
        _ => {}
    }
}

fn quit(app: &mut App) {
    if matches!(app.screen, Screen::Themes) {
        app.theme_list_state.select(Some(app.selected_theme));
    }

    app.should_quit = true;
}
