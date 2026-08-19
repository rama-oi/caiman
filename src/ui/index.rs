use crate::util::wrap_help_items;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, List, Padding, Paragraph},
};

const HELP_ITEMS: &[&str] = &[
    "[↑↓] navigate",
    "[:] command_mode",
    "[space] toggle_select",
    "[tab] switch_section",
    "[:s] settings",
    "[:q] quit",
];

use crate::app::App;

#[derive(Clone, Copy)]
enum Key<'a> {
    Normal {
        bottom_left: &'a str,
        top_left: &'a str,
        bottom_right: &'a str,
        top_right: &'a str,
    },
    NormalFn {
        bottom_left: &'a str,
        top_left: &'a str,
    },
    Wide {
        bottom_left: &'a str,
        top_left: &'a str,
        width: u16,
    },
}

impl<'a> Key<'a> {
    fn new(
        top_left: &'a str,
        bottom_left: &'a str,
        top_right: &'a str,
        bottom_right: &'a str,
    ) -> Self {
        Self::Normal {
            bottom_left,
            top_left,
            bottom_right,
            top_right,
        }
    }

    fn newFn(top_left: &'a str, bottom_left: &'a str) -> Self {
        Self::NormalFn {
            bottom_left,
            top_left,
        }
    }

    fn wide(bottom_left: &'a str, top_left: &'a str, width: u16) -> Self {
        Self::Wide {
            bottom_left,
            top_left,
            width,
        }
    }

    fn width(&self) -> u16 {
        match self {
            Self::Normal { .. } => 6,
            Self::NormalFn { .. } => 6,
            Self::Wide { width, .. } => *width,
        }
    }

    fn render(&self, area: Rect) -> Paragraph<'a> {
        let text = match self {
            Self::Normal {
                bottom_left,
                top_left,
                bottom_right,
                top_right,
            } => {
                format!(
                    "{} {}\n{} {}",
                    top_left, top_right, bottom_left, bottom_right
                )
            }

            Self::NormalFn {
                bottom_left,
                top_left,
            } => {
                format!("{}\n{}", top_left, bottom_left)
            }

            Self::Wide {
                bottom_left,
                top_left,
                ..
            } => {
                format!("{}\n{}", top_left, bottom_left)
            }
        };

        Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::bordered().padding(Padding::horizontal(1)))
    }
}

type KeyboardRow<'a> = &'a [Key<'a>];

fn render_keyboard(frame: &mut Frame, area: Rect, rows: &[KeyboardRow<'_>]) {
    let row_height = 4;

    // Find the width of the widest keyboard row.
    let keyboard_width = rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|key| key.width() + 1)
                .sum::<u16>()
                .saturating_sub(1)
        })
        .max()
        .unwrap_or(0);

    let keyboard_height = row_height * rows.len() as u16;

    // Center the entire keyboard inside the available area.
    let centered = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(keyboard_height),
            Constraint::Fill(1),
        ])
        .split(area)[1];

    let centered = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(keyboard_width),
            Constraint::Fill(1),
        ])
        .split(centered)[1];

    // Split the centered keyboard vertically into rows.
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            rows.iter()
                .map(|_| Constraint::Length(row_height))
                .collect::<Vec<_>>(),
        )
        .split(centered);

    for (row_index, row) in rows.iter().enumerate() {
        if row_index >= vertical.len() {
            break;
        }

        let row_area = vertical[row_index];

        let constraints = row
            .iter()
            .map(|key| Constraint::Length(key.width() + 1))
            .collect::<Vec<_>>();

        let key_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(row_area);

        for (key_index, key) in row.iter().enumerate() {
            if key_index >= key_areas.len() {
                break;
            }

            let key_area = key_areas[key_index];

            frame.render_widget(key.render(key_area), key_area);
        }
    }
}

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
        .constraints([Constraint::Length(24), Constraint::Fill(1)])
        .split(vertical[0]);

    let layout_list = List::new(["● en", "○ es", "○ pt"]).block(
        Block::bordered()
            .title(" switch layout ")
            .padding(Padding::horizontal(1)),
    );

    frame.render_widget(layout_list, horizontal[0]);

    let keyboard_block = Block::bordered().title(" current layout ");

    let keyboard_area = keyboard_block.inner(horizontal[1]);

    frame.render_widget(keyboard_block, horizontal[1]);

    let row1 = [
        Key::new("~", "`", "~", "`"),
        Key::new("!", "1", "!", "1"),
        Key::new("@", "2", "@", "2"),
        Key::new("#", "3", "#", "3"),
        Key::new("$", "4", "$", "4"),
        Key::new("%", "5", "%", "5"),
        Key::new("^", "6", "^", "6"),
        Key::new("&", "7", "&", "7"),
        Key::new("*", "8", "*", "8"),
        Key::new("(", "9", "(", "9"),
        Key::new(")", "0", ")", "0"),
        Key::new("_", "-", "_", "-"),
        Key::new("+", "=", "+", "="),
        Key::wide("backspace", "backspace", 13),
    ];

    let row2 = [
        Key::wide("tab", "tab", 13),
        Key::new("Q", "q", "Q", "q"),
        Key::new("W", "w", "W", "w"),
        Key::new("E", "e", "E", "e"),
        Key::new("R", "r", "R", "r"),
        Key::new("T", "t", "T", "t"),
        Key::new("Y", "y", "Y", "y"),
        Key::new("U", "u", "U", "u"),
        Key::new("I", "i", "I", "i"),
        Key::new("O", "o", "O", "o"),
        Key::new("P", "p", "P", "p"),
        Key::new("{", "[", "{", "["),
        Key::new("}", "]", "}", "]"),
        Key::new("|", "\\", "|", "\\"),
    ];

    let row3 = [
        Key::wide("caps lock", "caps lock", 13),
        Key::new("A", "a", "A", "a"),
        Key::new("S", "s", "S", "s"),
        Key::new("D", "d", "D", "d"),
        Key::new("F", "f", "F", "f"),
        Key::new("G", "g", "G", "g"),
        Key::new("H", "h", "H", "h"),
        Key::new("J", "j", "J", "j"),
        Key::new("K", "k", "K", "k"),
        Key::new("L", "l", "L", "l"),
        Key::new(":", ";", ":", ";"),
        Key::new("\"", "'", "\"", "'"),
        Key::wide("enter", "enter", 13),
    ];

    let row4 = [
        Key::wide("l-shift", "l-shift", 13),
        Key::new("Z", "z", "Z", "z"),
        Key::new("X", "x", "X", "x"),
        Key::new("C", "c", "C", "c"),
        Key::new("V", "v", "V", "v"),
        Key::new("B", "b", "B", "b"),
        Key::new("N", "n", "N", "n"),
        Key::new("M", "m", "M", "m"),
        Key::new("<", ",", "<", ","),
        Key::new(">", ".", ">", "."),
        Key::new("?", "/", "?", "/"),
        Key::wide("r-shift", "r-shift", 20),
    ];

    let row5 = [
        Key::newFn("ctr", "ctr"),
        Key::newFn("sup", "sup"),
        Key::newFn("alt", "alt"),
        Key::wide("spacebar", "spacebar", 69),
        Key::newFn("alt", "alt"),
        Key::newFn("ctr", "ctr"),
    ];

    let rows = [&row1[..], &row2[..], &row3[..], &row4[..], &row5[..]];

    render_keyboard(frame, keyboard_area, &rows);

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
