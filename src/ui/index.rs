use crate::util::wrap_help_items;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, List, Padding, Paragraph},
};

use crate::app::App;
use crate::ui::keyboard::render_keyboard;
use crate::util::truncate_label;

const HELP_ITEMS: &[&str] = &[
    "[↑↓] navigate",
    "[:] command_mode",
    "[space] toggle_select",
    "[tab] switch_section",
    "[:s] settings",
    "[:q] quit",
];

pub fn draw_index(frame: &mut Frame, app: &mut App) {
    let full_area = frame.area();

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

    let label_width = horizontal[0].width.saturating_sub(6); // borders(2) + padding(2) + marker(2)

    let layout_items: Vec<String> = app
        .layouts
        .iter()
        .enumerate()
        .map(|(i, layout)| {
            let marker = if i == app.selected_layout {
                "●"
            } else {
                "○"
            };
            // format!("{marker} {}", layout.id)
            format!("{marker} {}", truncate_label(&layout.id, label_width))
            // frame.render_stateful_widget(layout_list, horizontal[0], &mut app.layout_list_state);
        })
        .collect();

    let layout_list = List::new(layout_items).block(
        Block::bordered()
            .title(" switch layout ")
            .padding(Padding::horizontal(1)),
    );

    // frame.render_widget(layout_list, horizontal[0]);
    frame.render_stateful_widget(layout_list, horizontal[0], &mut app.layout_list_state);

    let keyboard_block = Block::bordered().title(" current layout ");

    let keyboard_area = keyboard_block.inner(horizontal[1]);

    frame.render_widget(keyboard_block, horizontal[1]);

    render_keyboard(frame, keyboard_area, app.layouts[app.selected_layout].rows);

    let key_preview =
        List::new(["key preview"]).block(Block::bordered().padding(Padding::horizontal(1)));

    frame.render_widget(key_preview, vertical[1]);

    let help_text = help_lines
        .iter()
        .map(|line| format!(" {line}"))
        .collect::<Vec<_>>()
        .join("\n");

    let help = Paragraph::new(help_text);

    frame.render_widget(help, vertical[2]);
}
