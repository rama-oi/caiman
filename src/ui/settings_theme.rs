use ratatui::{
    Frame,
    style::{Modifier, Style},
    widgets::{List, ListItem},
};

use crate::app::App;
use crate::ui::index::draw_keyboard_backdrop;
use crate::ui::modal::render_modal;
use crate::util::centered_rect;

const HELP_ITEMS: &[&str] = &["[↑↓] navigate", "[enter] select", "[esc] back", "[^q] quit"];

pub fn draw_settings_themes(frame: &mut Frame, app: &mut App) {
    draw_keyboard_backdrop(frame, app, HELP_ITEMS);

    let theme = app.theme().clone();
    let full_area = frame.area();

    let modal_area = centered_rect(46, 10, full_area);
    let inner = render_modal(frame, modal_area, "themes", &theme);

    let theme_items: Vec<ListItem> = app
        .themes
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let (marker, marker_color) = if i == app.selected_theme {
                ("●", theme.colors.accent)
            } else {
                ("○", theme.colors.text)
            };
            ListItem::new(format!("{marker} {}", t.name)).style(Style::default().fg(marker_color))
        })
        .collect();

    let themes_list = List::new(theme_items).highlight_style(
        Style::default()
            .fg(theme.colors.selection_fg)
            .bg(theme.colors.selection_bg)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(themes_list, inner, &mut app.theme_list_state);
}
