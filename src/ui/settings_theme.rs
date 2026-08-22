use crate::util::wrap_help_items;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, List, ListItem, Padding, Paragraph},
};

use crate::app::App;
use crate::ui::keyboard::render_keyboard;

const HELP_ITEMS: &[&str] = &[
    "[↑↓] navigate",
    "[esc] back",
    "[:] command_mode",
    "[:q] quit",
];

pub fn draw_settings_themes(frame: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
    let full_area = frame.area();

    frame.render_widget(
        Block::default()
            .style(Style::default().bg(theme.colors.background).fg(theme.colors.text)),
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

    let theme_items: Vec<ListItem> = app
        .themes
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let (marker, marker_color) = if i == app.selected_theme {
                ("●", theme.colors.accent)
            } else {
                ("○", theme.colors.shell_light)
            };
            ListItem::new(format!("{marker} {}", t.name)).style(Style::default().fg(marker_color))
        })
        .collect();

    let themes_list = List::new(theme_items)
        .block(
            Block::bordered()
                .title(" themes ")
                .title_style(Style::default().fg(theme.colors.header))
                .border_style(Style::default().fg(theme.colors.border))
                .padding(Padding::horizontal(1)),
        )
        .highlight_style(
            Style::default()
                .fg(theme.colors.selection_fg)
                .bg(theme.colors.selection_bg)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_stateful_widget(themes_list, horizontal[0], &mut app.theme_list_state);

    let keyboard_block = Block::bordered().border_style(Style::default().fg(theme.colors.border));
    let keyboard_inner = keyboard_block.inner(horizontal[1]);

    frame.render_widget(keyboard_block, horizontal[1]);

    render_keyboard(
        frame,
        keyboard_inner,
        &app.layouts[app.selected_layout].rows,
        &theme,
    );

    let key_preview = List::new([format!("{} — by {}", theme.name, theme.author)]).block(
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
