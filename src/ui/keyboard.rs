use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Padding, Paragraph},
};

use serde::Deserialize;
use std::process::Command;
use xkbcommon::xkb;

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

    fn render(&self, _area: Rect) -> Paragraph<'static> {
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

pub fn render_keyboard(frame: &mut Frame, area: Rect, rows: &[Vec<Key>]) {
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

// ============================================================
// US keyboard layout (fallback / default row shapes)
// ============================================================

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

/// The default (US QWERTY) row shapes, used as a fallback when a layout's
/// actual keysyms can't be loaded from the system.
pub fn en_rows() -> Vec<Vec<Key>> {
    vec![en_row1(), en_row2(), en_row3(), en_row4(), en_row5()]
}

// ============================================================
// Layout information
// ============================================================

pub struct LayoutInfo {
    pub id: String,
    pub name: String,

    // Actual XKB identifiers.
    pub layout: String,
    pub variant: String,

    // The actual keys for this layout, loaded from the system via
    // libxkbcommon. Falls back to `en_rows()` if loading fails.
    pub rows: Vec<Vec<Key>>,
}

// ============================================================
// XKB layout discovery
// ============================================================

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

/// Discover the layouts installed on the system.
///
/// This uses `xkbcli list`.
///
/// At this stage we're only using it to discover the layouts.
/// The actual keyboard symbols will be loaded separately
/// through libxkbcommon.
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

                    let rows = load_layout(&layout.layout, &layout.variant)
                        .map(|keymap| build_rows(&keymap))
                        .unwrap_or_else(|err| {
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

fn get_layouts() -> Result<Vec<XkbLayout>, Box<dyn std::error::Error>> {
    let output = Command::new("xkbcli").arg("list").output()?;

    if !output.status.success() {
        return Err(format!("xkbcli failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let data: XkbData = serde_yaml::from_slice(&output.stdout)?;

    Ok(data.layouts)
}

// ============================================================
// XKB keymap loading
// ============================================================

/// Compile an XKB layout using libxkbcommon.
///
/// This does NOT change the system keyboard layout.
/// It only loads the selected layout into an xkb::Keymap so
/// that we can inspect its actual key mappings.
pub fn load_layout(layout: &str, variant: &str) -> Result<xkb::Keymap, Box<dyn std::error::Error>> {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

    let rules = "evdev";
    let model = "pc105";
    let options: Option<String> = None;

    let keymap = xkb::Keymap::new_from_names(
        &context,
        rules,
        model,
        layout,
        variant,
        options,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .ok_or("failed to compile XKB keymap")?;

    Ok(keymap)
}

// ============================================================
// Translating a loaded keymap into the visual keyboard rows
// ============================================================

/// Canonical XKB key names (as assigned by the "evdev" rules) for the
/// alphanumeric/symbol keys, laid out to match the physical shape of
/// `en_row1()`..`en_row5()`. `None` marks a key that keeps its static
/// label (backspace, tab, enter, shift, ctrl, etc.) regardless of layout.
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

/// Get the displayable label for a single shift level of a key, e.g. the
/// unshifted ("a") or shifted ("A") character. Falls back to the keysym's
/// name (e.g. "backspace") if it has no direct Unicode representation, and
/// gives up (returns `None`) if the level is empty or unprintable.
fn level_label(
    keymap: &xkb::Keymap,
    keycode: xkb::Keycode,
    level: xkb::LevelIndex,
) -> Option<String> {
    let sym = *keymap.key_get_syms_by_level(keycode, 0, level).first()?;

    let utf8 = xkb::keysym_to_utf8(sym);
    if !utf8.is_empty() && !utf8.chars().any(|c| c.is_control()) {
        return Some(utf8);
    }

    let name = xkb::keysym_get_name(sym);
    if name.is_empty() { None } else { Some(name) }
}

/// Build a `Key::Normal` for the given canonical XKB key name, using the
/// unshifted/shifted keysyms from `keymap`. Falls back to `fallback` (the
/// US QWERTY key at the same position) if the key can't be found or the
/// layout doesn't define it.
fn translated_key(keymap: &xkb::Keymap, name: &str, fallback: Key) -> Key {
    let Some(keycode) = keymap.key_by_name(name) else {
        return fallback;
    };

    let unshifted = level_label(keymap, keycode, 0);
    let shifted = level_label(keymap, keycode, 1);

    let Key::Normal {
        bottom_left: fallback_unshifted,
        top_left: fallback_shifted,
        ..
    } = &fallback
    else {
        return fallback;
    };

    let unshifted = unshifted.unwrap_or_else(|| fallback_unshifted.clone());
    let shifted = shifted.unwrap_or_else(|| fallback_shifted.clone());

    Key::new(&shifted, &unshifted, &shifted, &unshifted)
}

fn translate_row(row: Vec<Key>, names: &[Option<&str>], keymap: &xkb::Keymap) -> Vec<Key> {
    row.into_iter()
        .enumerate()
        .map(|(i, key)| match names.get(i).copied().flatten() {
            Some(name) => translated_key(keymap, name, key),
            None => key,
        })
        .collect()
}

/// Build the visual keyboard rows for a loaded layout, translating every
/// alphanumeric/symbol key to what that layout actually produces while
/// keeping function/modifier keys (backspace, tab, enter, shift, etc.)
/// labeled the same regardless of layout.
pub fn build_rows(keymap: &xkb::Keymap) -> Vec<Vec<Key>> {
    let row1 = en_row1();
    let row1 = {
        let backspace = row1.last().cloned();
        let mut translated = translate_row(row1[..13].to_vec(), &ROW1_NAMES, keymap);
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
        translated.extend(translate_row(row2[1..13].to_vec(), &ROW2_NAMES, keymap));
        if let Some(fallback) = bksl {
            translated.push(translated_key(keymap, ROW2_BKSL, fallback));
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
        translated.extend(translate_row(row3[1..12].to_vec(), &ROW3_NAMES, keymap));
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
        translated.extend(translate_row(row4[1..11].to_vec(), &ROW4_NAMES, keymap));
        if let Some(r_shift) = r_shift {
            translated.push(r_shift);
        }
        translated
    };

    // The bottom row (ctrl/super/alt/spacebar) has no printable keys, so it
    // stays the same regardless of layout.
    let row5 = en_row5();

    vec![row1, row2, row3, row4, row5]
}
