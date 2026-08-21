use crate::util::wrap_help_items;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, List, Padding, Paragraph},
};

use crate::app::App;
use crate::ui::keyboard::render_keyboard;

const HELP_ITEMS: &[&str] = &[
    "[↑↓] navigate",
    "[esc] exit_about",
    "[:] command_mode",
    "[:q] quit",
];

pub fn draw_settings_about(frame: &mut Frame, app: &mut App) {
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

    let layout_list = List::new(["Caiman v2026.1", "Pombo", "github.com/rama-oi/caiman"]).block(
        Block::bordered()
            .title(" about ")
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(layout_list, horizontal[0]);

    let keyboard_block = Block::bordered();
    let keyboard_inner = keyboard_block.inner(horizontal[1]);

    frame.render_widget(keyboard_block, horizontal[1]);

    render_keyboard(
        frame,
        keyboard_inner,
        &app.layouts[app.selected_layout].rows,
    );

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
