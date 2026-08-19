use std::time::Instant;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Cell, List, Padding, Paragraph, Row, Table},
};

use crate::app::App;

pub fn draw_index(frame: &mut Frame, app: &mut App) {
    let full_area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(full_area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Fill(1)])
        .split(vertical[0]);

    let items = ["en", "es", "pt"];
    let layout_list = List::new(items).block(Block::bordered().title("Switch Layouts"));

    frame.render_widget(layout_list, horizontal[0]);
}
