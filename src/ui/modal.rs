use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Clear, Padding},
};

use crate::theme::Theme;

pub fn render_modal(frame: &mut Frame, area: Rect, title: &str, theme: &Theme) -> Rect {
    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title(format!(" {title} "))
        .title_style(Style::default().fg(theme.colors.header))
        .border_style(Style::default().fg(theme.colors.border))
        .style(
            Style::default()
                .fg(theme.colors.text)
                .bg(theme.colors.background),
        )
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    inner
}
