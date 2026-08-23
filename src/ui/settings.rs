use ratatui::{
    Frame,
    style::{Modifier, Style},
    widgets::{List, ListItem},
};

use crate::app::App;
use crate::ui::index::draw_keyboard_backdrop;
use crate::ui::modal::render_modal;
use crate::util::centered_rect;

const HELP_ITEMS: &[&str] = &["[↑↓] navigate", "[enter] open", "[esc] back", "[^q] quit"];

pub const ITEM_SWITCH_THEME: usize = 0;
pub const ITEM_ABOUT_CAIMAN: usize = 1;
pub const ITEM_COUNT: usize = 2;

pub fn draw_settings(frame: &mut Frame, app: &mut App) {
    draw_keyboard_backdrop(frame, app, HELP_ITEMS);

    let theme = app.theme().clone();
    let full_area = frame.area();

    let modal_area = centered_rect(46, 10, full_area);
    let inner = render_modal(frame, modal_area, "settings", &theme);

    let items = [
        format!("Switch Theme\n{}", theme.name),
        "About Caiman".to_string(),
    ];

    let settings_list = List::new(items.into_iter().map(ListItem::new)).highlight_style(
        Style::default()
            .fg(theme.colors.selection_fg)
            .bg(theme.colors.selection_bg)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(settings_list, inner, &mut app.settings_list_state);
}
