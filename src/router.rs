use crate::app::{App, Screen};

pub fn go_to_index(app: &mut App) {
    app.screen = Screen::Index;
}

pub fn go_to_settings(app: &mut App) {
    app.screen = Screen::Settings;
}

pub fn go_to_about(app: &mut App) {
    app.screen = Screen::About;
}

pub fn go_to_themes(app: &mut App) {
    app.screen = Screen::Themes;
}

pub fn go_back(app: &mut App) {
    match app.screen {
        Screen::Index => {}
        Screen::Settings => go_to_index(app),
        Screen::About | Screen::Themes => go_to_settings(app),
    }
}
