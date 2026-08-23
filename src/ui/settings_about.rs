use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::Paragraph,
};

use crate::app::App;
use crate::ui::index::draw_keyboard_backdrop;
use crate::ui::modal::render_modal;
use crate::util::centered_rect;

const HELP_ITEMS: &[&str] = &["[esc] back", "[^q] quit"];

pub fn draw_settings_about(frame: &mut Frame, app: &mut App) {
    draw_keyboard_backdrop(frame, app, HELP_ITEMS);

    let theme = app.theme().clone();
    let full_area = frame.area();

    let modal_area = centered_rect(46, 26, full_area);
    let inner = render_modal(frame, modal_area, "about", &theme);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(4)])
        .split(inner);

    let logo = Paragraph::new(
        "████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████\n\
████████████████████",
    )
    .alignment(Alignment::Center);
    frame.render_widget(logo, vertical[0]);

    let content = Paragraph::new(format!(
        "{} {}\n{}\n{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    ))
    .alignment(Alignment::Center)
    .style(Style::default().fg(theme.colors.text));

    frame.render_widget(content, vertical[1]);
}
