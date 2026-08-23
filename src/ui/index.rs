use crate::util::{truncate_label, wrap_help_items};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph},
};

use crate::app::App;
use crate::ui::keyboard::{describe_key_event, find_highlight, highlight_label, render_keyboard};

const HELP_ITEMS: &[&str] = &["[^s] settings", "[^q] quit"];

pub fn draw_index(frame: &mut Frame, app: &mut App) {
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

    let keyboard_block = Block::bordered()
        .title_style(Style::default().fg(theme.colors.header))
        .border_style(Style::default().fg(theme.colors.border));

    let keyboard_area = keyboard_block.inner(vertical[0]);

    frame.render_widget(keyboard_block, vertical[0]);

    let key_info = app.last_key_event.as_ref().map(describe_key_event);
    let highlighted = app
        .last_key_event
        .as_ref()
        .and_then(|event| highlight_label(event.code))
        .and_then(|label| find_highlight(&app.current_layout.rows, &label));

    render_keyboard(
        frame,
        keyboard_area,
        &app.current_layout.rows,
        &theme,
        highlighted,
    );

    let preview_block = Block::bordered()
        .title_style(Style::default().fg(theme.colors.header))
        .border_style(Style::default().fg(theme.colors.border))
        .padding(Padding::horizontal(1));

    let preview_inner = preview_block.inner(vertical[1]);
    frame.render_widget(preview_block, vertical[1]);

    match key_info {
        Some(info) => {
            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                ])
                .split(preview_inner);

            let fields = [
                ("KeyCode", info.keycode_label.clone()),
                ("KeySym", info.keysym_display()),
                ("Unicode", info.unicode_display()),
                ("State", info.state_display()),
            ];

            for (i, (label, value)) in fields.iter().enumerate() {
                let col_area = columns[i * 2];
                let value = truncate_label(value, col_area.width);

                let text = vec![
                    Line::from(Span::styled(
                        *label,
                        Style::default()
                            .fg(theme.colors.header)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(Span::styled(
                        value,
                        Style::default().fg(theme.colors.accent),
                    )),
                ];

                frame.render_widget(
                    Paragraph::new(text).alignment(ratatui::layout::Alignment::Center),
                    col_area,
                );
            }

            for &divider_index in &[1usize, 3, 5] {
                let divider = Paragraph::new("│\n│")
                    .alignment(ratatui::layout::Alignment::Center)
                    .style(Style::default().fg(theme.colors.shell));
                frame.render_widget(divider, columns[divider_index]);
            }
        }
        None => {
            let status_text = app
                .status_message
                .clone()
                .unwrap_or_else(|| "press a key to see information".to_string());

            frame.render_widget(
                Paragraph::new(status_text).style(Style::default().fg(theme.colors.text)),
                preview_inner,
            );
        }
    }

    let help_text = help_lines
        .iter()
        .map(|line| format!(" {line}"))
        .collect::<Vec<_>>()
        .join("\n");

    let help = Paragraph::new(help_text).style(Style::default().fg(theme.colors.shell_light));

    frame.render_widget(help, vertical[2]);
}
