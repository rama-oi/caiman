use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Padding, Paragraph},
};

#[derive(Clone, Copy)]
pub enum Key<'a> {
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
    pub const fn new(
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

    pub const fn new_fn(top_left: &'a str, bottom_left: &'a str) -> Self {
        Self::NormalFn {
            bottom_left,
            top_left,
        }
    }

    pub const fn wide(bottom_left: &'a str, top_left: &'a str, width: u16) -> Self {
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

    fn render(&self, _area: Rect) -> Paragraph<'a> {
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

pub type KeyboardRow<'a> = &'a [Key<'a>];

pub fn render_keyboard(frame: &mut Frame, area: Rect, rows: &[KeyboardRow<'_>]) {
    let row_height = 4;

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

const EN_ROW1: [Key; 14] = [
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

const EN_ROW2: [Key; 14] = [
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

const EN_ROW3: [Key; 13] = [
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

const EN_ROW4: [Key; 12] = [
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

const EN_ROW5: [Key; 6] = [
    Key::new_fn("ctr", "ctr"),
    Key::new_fn("sup", "sup"),
    Key::new_fn("alt", "alt"),
    Key::wide("spacebar", "spacebar", 69),
    Key::new_fn("alt", "alt"),
    Key::new_fn("ctr", "ctr"),
];

pub const EN_ROWS: [&[Key]; 5] = [&EN_ROW1, &EN_ROW2, &EN_ROW3, &EN_ROW4, &EN_ROW5];

pub struct LayoutInfo {
    pub id: String,
    pub name: String,
    pub rows: &'static [&'static [Key<'static>]],
}

pub fn discover_layouts() -> Vec<LayoutInfo> {
    match get_layouts() {
        Ok(layouts) => {
            println!("{} layouts", layouts.len());

            layouts
                .into_iter()
                .map(|layout| {
                    let id = if layout.variant.is_empty() {
                        format!("{}_{}", layout.layout, layout.brief)
                    } else {
                        format!("{}_{}_{}", layout.layout, layout.brief, layout.variant)
                    };

                    LayoutInfo {
                        id: id,
                        name: layout.description,
                        rows: &EN_ROWS,
                    }
                })
                .collect()
        }

        Err(err) => {
            eprintln!("Failed to get layouts: {err}");

            vec![LayoutInfo {
                id: "us".to_string(),
                name: "English (US)".to_string(),
                rows: &EN_ROWS,
            }]
        }
    }
}

use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct XkbData {
    layouts: Vec<XkbLayout>,
}

#[derive(Debug, Deserialize)]
struct XkbLayout {
    layout: String,
    variant: String,
    brief: String,
    description: String,
}

fn get_layouts() -> Result<Vec<XkbLayout>, Box<dyn std::error::Error>> {
    let output = Command::new("xkbcli").arg("list").output()?;

    if !output.status.success() {
        return Err(format!("xkbcli failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let data: XkbData = serde_yaml::from_slice(&output.stdout)?;

    Ok(data.layouts)
}
