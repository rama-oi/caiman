use crate::util::wrap_help_items;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Padding, Paragraph},
};

use crate::app::App;

const HELP_ITEMS: &[&str] = &[
    "[↑↓] navigate",
    "[esc] back",
    "[:] command_mode",
    "[:q] quit",
];

pub fn draw_settings_about(frame: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
    let full_area = frame.area();

    let block = Block::bordered()
        .title(" about ")
        .title_style(Style::default().fg(theme.colors.header))
        .border_style(Style::default().fg(theme.colors.border))
        .style(
            Style::default()
                .fg(theme.colors.text)
                .bg(theme.colors.background),
        )
        .padding(Padding::horizontal(2));

    frame.render_widget(block, full_area);

    let inner_area = full_area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 1,
    });

    let help_width = inner_area.width.saturating_sub(2);
    let help_lines = wrap_help_items(HELP_ITEMS, help_width);
    let help_height = help_lines.len() as u16;

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(20),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(help_height),
        ])
        .split(inner_area);

    let logo = Paragraph::new(
        "████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████\n
████████████████████",
    )
    .alignment(Alignment::Center);
    frame.render_widget(logo, vertical[1]);

    let content = Paragraph::new(format!(
        "{} {}\n{}\n{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    ))
    .alignment(Alignment::Center)
    .style(Style::default().fg(theme.colors.text));

    frame.render_widget(content, vertical[2]);

    let help_text = help_lines
        .iter()
        .map(|line| format!(" {line}"))
        .collect::<Vec<_>>()
        .join("\n");

    let help = Paragraph::new(help_text).style(Style::default().fg(theme.colors.shell_light));

    frame.render_widget(help, vertical[4]);
}
