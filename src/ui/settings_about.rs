use crate::util::wrap_help_items;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, List, Padding, Paragraph},
};

use crate::app::App;
use crate::ui::keyboard::render_keyboard;

const HELP_ITEMS: &[&str] = &[
    "[↑↓] navigate",
    "[esc] back",
    "[:] command_mode",
    "[:q] quit",
];

pub fn draw_settings_about(frame: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
    let full_area = frame.area();

    frame.render_widget(
        Block::default().style(
            Style::default()
                .bg(theme.colors.background)
                .fg(theme.colors.text),
        ),
        full_area,
    );

    let help_width = full_area.width.saturating_sub(2);
    let help_lines = wrap_help_items(HELP_ITEMS, help_width);
    let help_height = help_lines.len() as u16;

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Length(help_height),
        ])
        .split(full_area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(42), Constraint::Fill(1)])
        .split(vertical[0]);

    let layout_list = List::new([
        format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        env!("CARGO_PKG_AUTHORS").to_string(),
        env!("CARGO_PKG_HOMEPAGE").to_string(),
    ])
    .block(
        Block::bordered()
            .title(" about ")
            .title_style(Style::default().fg(theme.colors.header))
            .border_style(Style::default().fg(theme.colors.border))
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(layout_list, horizontal[0]);

    let keyboard_block = Block::bordered().border_style(Style::default().fg(theme.colors.border));
    let keyboard_inner = keyboard_block.inner(horizontal[1]);

    frame.render_widget(keyboard_block, horizontal[1]);

    render_keyboard(
        frame,
        keyboard_inner,
        &app.layouts[app.selected_layout].rows,
        &theme,
    );

    let key_preview = List::new(["key preview"]).block(
        Block::bordered()
            .border_style(Style::default().fg(theme.colors.border))
            .padding(Padding::horizontal(1)),
    );

    frame.render_widget(key_preview, vertical[1]);

    let help_text = help_lines
        .iter()
        .map(|line| format!(" {line}"))
        .collect::<Vec<_>>()
        .join("\n");

    let help = Paragraph::new(help_text).style(Style::default().fg(theme.colors.shell_light));
    frame.render_widget(help, vertical[2]);
}
