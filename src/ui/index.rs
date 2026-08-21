use crate::util::{truncate_label, wrap_help_items};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, List, ListItem, Padding, Paragraph},
};

use crate::app::App;
use crate::ui::keyboard::render_keyboard;

const HELP_ITEMS: &[&str] = &[
    "[type] search",
    "[↑↓] navigate",
    "[space] apply",
    "[esc] clear search",
    "[:] command_mode",
    "[tab] switch_section",
    "[:s] settings",
    "[:q] quit",
];

pub fn draw_index(frame: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
    let full_area = frame.area();

    // Paint the theme's background/text across the whole frame first;
    // everything drawn afterwards only overrides the specific style bits
    // it cares about (border color, highlight, etc.), so the base color
    // shows through everywhere else.
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
        .constraints([Constraint::Length(28), Constraint::Fill(1)])
        .split(vertical[0]);

    // The whole left-hand panel (search bar + list) lives inside one
    // "switch layout" block, so we draw the block once and split its
    // inner area ourselves rather than giving the list its own border.
    let switch_block = Block::bordered()
        .title(" switch layout ")
        .title_style(Style::default().fg(theme.colors.header))
        .border_style(Style::default().fg(theme.colors.border))
        .padding(Padding::horizontal(1));
    let switch_inner = switch_block.inner(horizontal[0]);
    frame.render_widget(switch_block, horizontal[0]);

    let switch_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // search input
            Constraint::Length(1), // divider
            Constraint::Fill(1),   // filtered list
        ])
        .split(switch_inner);

    let search_area = switch_layout[0];
    let divider_area = switch_layout[1];
    let list_area = switch_layout[2];

    let search_text = if app.search_query.is_empty() {
        Paragraph::new(" search layouts…").style(Style::default().fg(theme.colors.shell_light))
    } else {
        Paragraph::new(format!(" {}", app.search_query))
            .style(Style::default().fg(theme.colors.text))
    };
    frame.render_widget(search_text, search_area);

    // Put the terminal's real cursor at the end of the typed query so it's
    // obvious this box is what's receiving keystrokes.
    frame.set_cursor_position((
        search_area.x + 1 + app.search_query.chars().count() as u16,
        search_area.y,
    ));

    let divider = Paragraph::new("─".repeat(divider_area.width as usize))
        .style(Style::default().fg(theme.colors.shell));
    frame.render_widget(divider, divider_area);

    // Available width for the label text itself, after accounting for the
    // "● " marker (2). The block's border/padding is already excluded
    // since `list_area` comes from `switch_block.inner(..)`.
    let label_width = list_area.width.saturating_sub(2);

    let filtered_indices = app.filtered_layout_indices();

    let layout_items: Vec<ListItem> = filtered_indices
        .iter()
        .map(|&i| {
            let layout = &app.layouts[i];
            let (marker, marker_color) = if i == app.selected_layout {
                ("●", theme.colors.accent)
            } else {
                ("○", theme.colors.shell_light)
            };
            ListItem::new(format!(
                "{marker} {}",
                truncate_label(&layout.id, label_width)
            ))
            .style(Style::default().fg(marker_color))
        })
        .collect();

    let layout_list = List::new(layout_items).highlight_style(
        Style::default()
            .fg(theme.colors.selection_fg)
            .bg(theme.colors.selection_bg)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(layout_list, list_area, &mut app.layout_list_state);

    let keyboard_block = Block::bordered()
        .title(" current layout ")
        .title_style(Style::default().fg(theme.colors.header))
        .border_style(Style::default().fg(theme.colors.border));

    let keyboard_area = keyboard_block.inner(horizontal[1]);

    frame.render_widget(keyboard_block, horizontal[1]);

    render_keyboard(
        frame,
        keyboard_area,
        &app.layouts[app.selected_layout].rows,
        &theme,
    );

    let status_text = app
        .status_message
        .clone()
        .unwrap_or_else(|| "press <space> to apply the selected layout".to_string());

    let key_preview = List::new([status_text]).block(
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
