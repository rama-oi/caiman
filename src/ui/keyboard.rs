use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Padding, Paragraph},
};

use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;
use xkbcommon::xkb;

use crate::theme::Theme;

#[derive(Clone)]
pub enum Key {
    Normal {
        bottom_left: String,
        top_left: String,
        bottom_right: String,
        top_right: String,
    },
    NormalFn {
        bottom_left: String,
        top_left: String,
    },
    Wide {
        bottom_left: String,
        top_left: String,
        width: u16,
    },
}

impl Key {
    pub fn new(top_left: &str, bottom_left: &str, top_right: &str, bottom_right: &str) -> Self {
        Self::Normal {
            bottom_left: bottom_left.to_string(),
            top_left: top_left.to_string(),
            bottom_right: bottom_right.to_string(),
            top_right: top_right.to_string(),
        }
    }

    pub fn new_fn(top_left: &str, bottom_left: &str) -> Self {
        Self::NormalFn {
            bottom_left: bottom_left.to_string(),
            top_left: top_left.to_string(),
        }
    }

    pub fn wide(bottom_left: &str, top_left: &str, width: u16) -> Self {
        Self::Wide {
            bottom_left: bottom_left.to_string(),
            top_left: top_left.to_string(),
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

    fn render(&self, _area: Rect, theme: &Theme) -> Paragraph<'static> {
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
            .style(Style::default().fg(theme.colors.text))
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(theme.colors.border))
                    .padding(Padding::horizontal(1)),
            )
    }
}

pub fn render_keyboard(frame: &mut Frame, area: Rect, rows: &[Vec<Key>], theme: &Theme) {
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

            frame.render_widget(key.render(key_area, theme), key_area);
        }
    }
}

fn en_row1() -> Vec<Key> {
    vec![
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
    ]
}

fn en_row2() -> Vec<Key> {
    vec![
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
    ]
}

fn en_row3() -> Vec<Key> {
    vec![
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
    ]
}

fn en_row4() -> Vec<Key> {
    vec![
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
    ]
}

fn en_row5() -> Vec<Key> {
    vec![
        Key::new_fn("ctr", "ctr"),
        Key::new_fn("sup", "sup"),
        Key::new_fn("alt", "alt"),
        Key::wide("spacebar", "spacebar", 69),
        Key::new_fn("alt", "alt"),
        Key::new_fn("ctr", "ctr"),
    ]
}

pub fn en_rows() -> Vec<Vec<Key>> {
    vec![en_row1(), en_row2(), en_row3(), en_row4(), en_row5()]
}

pub struct LayoutInfo {
    pub id: String,
    pub name: String,

    pub layout: String,
    pub variant: String,

    pub rows: Vec<Vec<Key>>,
}

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

pub fn discover_layouts(list_layout_cmd: &str) -> Vec<LayoutInfo> {
    match get_layouts(list_layout_cmd) {
        Ok(mut layouts) => {
            println!("{} layouts", layouts.len());

            layouts.sort_by(|a, b| {
                a.brief
                    .cmp(&b.brief)
                    .then_with(|| a.layout.cmp(&b.layout))
                    .then_with(|| a.variant.cmp(&b.variant))
                    .then_with(|| a.description.cmp(&b.description))
            });

            layouts
                .into_iter()
                .map(|layout| {
                    let id = if layout.variant.is_empty() {
                        format!("{}_{}", layout.brief, layout.layout)
                    } else {
                        format!("{}_{}_{}", layout.brief, layout.layout, layout.variant)
                    };

                    let rows =
                        compile_rows(&layout.layout, &layout.variant).unwrap_or_else(|err| {
                            eprintln!(
                                "Failed to load keys for {} ({}): {err}",
                                layout.layout, layout.variant
                            );
                            en_rows()
                        });

                    LayoutInfo {
                        id,
                        name: layout.description,
                        layout: layout.layout,
                        variant: layout.variant,
                        rows,
                    }
                })
                .collect()
        }

        Err(err) => {
            eprintln!("Failed to get layouts: {err}");

            vec![LayoutInfo {
                id: "us".to_string(),
                name: "English (US)".to_string(),
                layout: "us".to_string(),
                variant: String::new(),
                rows: en_rows(),
            }]
        }
    }
}

fn get_layouts(list_layout_cmd: &str) -> Result<Vec<XkbLayout>, Box<dyn std::error::Error>> {
    let (program, args) = split_command(list_layout_cmd, "xkbcli", &["list"]);

    let output = Command::new(&program).args(&args).output()?;

    if !output.status.success() {
        return Err(format!(
            "{list_layout_cmd} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let data: XkbData = serde_yaml::from_slice(&output.stdout)?;

    Ok(data.layouts)
}

fn split_command(
    cmd: &str,
    fallback_program: &str,
    fallback_args: &[&str],
) -> (String, Vec<String>) {
    let mut parts = cmd.split_whitespace();

    match parts.next() {
        Some(program) => (program.to_string(), parts.map(str::to_string).collect()),
        None => (
            fallback_program.to_string(),
            fallback_args.iter().map(|s| s.to_string()).collect(),
        ),
    }
}

fn compile_keymap_text(layout: &str, variant: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut command = Command::new("xkbcli");
    command.args([
        "compile-keymap",
        "--rules",
        "evdev",
        "--model",
        "pc105",
        "--layout",
        layout,
    ]);

    if !variant.is_empty() {
        command.args(["--variant", variant]);
    }

    let output = command.output()?;

    if !output.status.success() {
        return Err(format!(
            "xkbcli compile-keymap failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn compile_rows(layout: &str, variant: &str) -> Result<Vec<Vec<Key>>, Box<dyn std::error::Error>> {
    let compiled = compile_keymap_text(layout, variant)?;
    let levels = parse_symbol_levels(&compiled);
    Ok(build_rows(&levels))
}

fn parse_symbol_levels(compiled: &str) -> HashMap<String, Vec<String>> {
    let mut levels: HashMap<String, Vec<String>> = HashMap::new();
    let mut search_from = 0;

    while let Some(rel) = compiled[search_from..].find("key <") {
        let name_start = search_from + rel + "key <".len();

        let Some(name_end_rel) = compiled[name_start..].find('>') else {
            break;
        };
        let name = compiled[name_start..name_start + name_end_rel].to_string();
        let after_name = name_start + name_end_rel + 1;

        let Some(block_end_rel) = compiled[after_name..].find("};") else {
            break;
        };
        let block = &compiled[after_name..after_name + block_end_rel];

        if let Some(symbols) = extract_symbol_list(block) {
            levels.insert(name, symbols);
        }

        search_from = after_name + block_end_rel + 2;
    }

    levels
}

fn extract_symbol_list(block: &str) -> Option<Vec<String>> {
    let chars: Vec<char> = block.chars().collect();
    let mut idx = 0;

    while idx < chars.len() {
        if chars[idx] == '[' {
            let mut back = idx;
            while back > 0 && chars[back - 1].is_whitespace() {
                back -= 1;
            }
            let preceded_by_assignment = back > 0 && matches!(chars[back - 1], '=' | '{');

            if preceded_by_assignment {
                let close_offset = chars[idx..].iter().position(|&c| c == ']')?;
                let inner: String = chars[idx + 1..idx + close_offset].iter().collect();

                return Some(
                    inner
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                );
            }
        }

        idx += 1;
    }

    None
}

fn keysym_name_to_display(name: &str) -> String {
    let name = name.trim();

    if name.is_empty() || name == "NoSymbol" {
        return String::new();
    }

    if name.chars().count() == 1 {
        return name.to_string();
    }

    let sym = xkb::keysym_from_name(name, xkb::KEYSYM_NO_FLAGS);
    let utf8 = xkb::keysym_to_utf8(sym);

    if !utf8.is_empty() && !utf8.chars().any(|c| c.is_control()) {
        return utf8;
    }

    name.to_string()
}

pub fn apply_layout(
    switch_layout_cmd: &str,
    layout: &str,
    variant: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let program = if switch_layout_cmd.trim().is_empty() {
        "swaymsg"
    } else {
        switch_layout_cmd.trim()
    };

    run_switch_command(program, &["input", "type:keyboard", "xkb_layout", layout])?;

    let variant_arg = if variant.is_empty() { "\"\"" } else { variant };
    run_switch_command(
        program,
        &["input", "type:keyboard", "xkb_variant", variant_arg],
    )?;

    Ok(())
}

fn run_switch_command(program: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(program).args(args).output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let ipc_rejected =
        stdout.contains("\"success\":false") || stdout.contains("\"success\": false");

    if !output.status.success() || ipc_rejected {
        let mut message = String::new();
        if !stderr.trim().is_empty() {
            message.push_str(stderr.trim());
        }
        if !stdout.trim().is_empty() {
            if !message.is_empty() {
                message.push_str(" | ");
            }
            message.push_str(stdout.trim());
        }
        if message.is_empty() {
            message.push_str(&format!(
                "{program} exited with an error but printed nothing"
            ));
        }

        return Err(format!("{program} failed: {message}").into());
    }

    Ok(())
}

const ROW1_NAMES: [Option<&str>; 13] = [
    Some("TLDE"),
    Some("AE01"),
    Some("AE02"),
    Some("AE03"),
    Some("AE04"),
    Some("AE05"),
    Some("AE06"),
    Some("AE07"),
    Some("AE08"),
    Some("AE09"),
    Some("AE10"),
    Some("AE11"),
    Some("AE12"),
];

const ROW2_NAMES: [Option<&str>; 12] = [
    Some("AD01"),
    Some("AD02"),
    Some("AD03"),
    Some("AD04"),
    Some("AD05"),
    Some("AD06"),
    Some("AD07"),
    Some("AD08"),
    Some("AD09"),
    Some("AD10"),
    Some("AD11"),
    Some("AD12"),
];
const ROW2_BKSL: &str = "BKSL";

const ROW3_NAMES: [Option<&str>; 11] = [
    Some("AC01"),
    Some("AC02"),
    Some("AC03"),
    Some("AC04"),
    Some("AC05"),
    Some("AC06"),
    Some("AC07"),
    Some("AC08"),
    Some("AC09"),
    Some("AC10"),
    Some("AC11"),
];

const ROW4_NAMES: [Option<&str>; 10] = [
    Some("AB01"),
    Some("AB02"),
    Some("AB03"),
    Some("AB04"),
    Some("AB05"),
    Some("AB06"),
    Some("AB07"),
    Some("AB08"),
    Some("AB09"),
    Some("AB10"),
];

fn translated_key(levels: &HashMap<String, Vec<String>>, name: &str, fallback: Key) -> Key {
    let Some(syms) = levels.get(name) else {
        return fallback;
    };

    let Key::Normal {
        bottom_left: fallback_unshifted,
        top_left: fallback_shifted,
        ..
    } = &fallback
    else {
        return fallback;
    };

    let unshifted = syms
        .first()
        .map(|s| keysym_name_to_display(s))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| fallback_unshifted.clone());

    let shifted = syms
        .get(1)
        .map(|s| keysym_name_to_display(s))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| fallback_shifted.clone());

    Key::new(&shifted, &unshifted, &shifted, &unshifted)
}

fn translate_row(
    row: Vec<Key>,
    names: &[Option<&str>],
    levels: &HashMap<String, Vec<String>>,
) -> Vec<Key> {
    row.into_iter()
        .enumerate()
        .map(|(i, key)| match names.get(i).copied().flatten() {
            Some(name) => translated_key(levels, name, key),
            None => key,
        })
        .collect()
}

fn build_rows(levels: &HashMap<String, Vec<String>>) -> Vec<Vec<Key>> {
    let row1 = en_row1();
    let row1 = {
        let backspace = row1.last().cloned();
        let mut translated = translate_row(row1[..13].to_vec(), &ROW1_NAMES, levels);
        if let Some(backspace) = backspace {
            translated.push(backspace);
        }
        translated
    };

    let row2 = en_row2();
    let row2 = {
        let tab = row2.first().cloned();
        let bksl = row2.last().cloned();
        let mut translated = vec![];
        if let Some(tab) = tab {
            translated.push(tab);
        }
        translated.extend(translate_row(row2[1..13].to_vec(), &ROW2_NAMES, levels));
        if let Some(fallback) = bksl {
            translated.push(translated_key(levels, ROW2_BKSL, fallback));
        }
        translated
    };

    let row3 = en_row3();
    let row3 = {
        let caps = row3.first().cloned();
        let enter = row3.last().cloned();
        let mut translated = vec![];
        if let Some(caps) = caps {
            translated.push(caps);
        }
        translated.extend(translate_row(row3[1..12].to_vec(), &ROW3_NAMES, levels));
        if let Some(enter) = enter {
            translated.push(enter);
        }
        translated
    };

    let row4 = en_row4();
    let row4 = {
        let l_shift = row4.first().cloned();
        let r_shift = row4.last().cloned();
        let mut translated = vec![];
        if let Some(l_shift) = l_shift {
            translated.push(l_shift);
        }
        translated.extend(translate_row(row4[1..11].to_vec(), &ROW4_NAMES, levels));
        if let Some(r_shift) = r_shift {
            translated.push(r_shift);
        }
        translated
    };

    let row5 = en_row5();

    vec![row1, row2, row3, row4, row5]
}
