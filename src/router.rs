use crate::app::{App, Screen};

/// All screen-to-screen navigation goes through here, so every input
/// handler drives the same transitions instead of each keeping its own
/// copy of `app.screen = ...`.

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

/// Step back one level in the screen hierarchy:
///
/// `Index` (home) has no parent, `Settings`'s parent is `Index`, and
/// `About`/`Themes`'s parent is `Settings`. This is what `esc` calls from
/// every screen so "go back" stays consistent as more screens are added.
pub fn go_back(app: &mut App) {
    match app.screen {
        Screen::Index => {}
        Screen::Settings => go_to_index(app),
        Screen::About | Screen::Themes => go_to_settings(app),
    }
}
