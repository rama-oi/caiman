use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Padding, Paragraph},
};

use crossterm::event::{KeyCode as CtKeyCode, KeyEvent, ModifierKeyCode};
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;
use xkbcommon::xkb;

use crate::config::Config;
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
    Empty {},
    EmptyHalf {},
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

    pub fn new_empty() -> Self {
        Self::Empty {}
    }

    pub fn new_empty_half() -> Self {
        Self::EmptyHalf {}
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
            Self::Normal { .. } => 7,
            Self::NormalFn { .. } => 7,
            Self::Empty { .. } => 7,
            Self::EmptyHalf { .. } => 3,
            Self::Wide { width, .. } => *width,
        }
    }

    fn matches_label(&self, target: &str) -> bool {
        match self {
            Self::Empty {} => false,
            Self::EmptyHalf {} => false,

            Self::Normal {
                top_left,
                bottom_left,
                ..
            } => top_left.eq_ignore_ascii_case(target) || bottom_left.eq_ignore_ascii_case(target),

            Self::NormalFn {
                top_left,
                bottom_left,
            } => top_left.eq_ignore_ascii_case(target) || bottom_left.eq_ignore_ascii_case(target),

            Self::Wide {
                top_left,
                bottom_left,
                ..
            } => top_left.eq_ignore_ascii_case(target) || bottom_left.eq_ignore_ascii_case(target),
        }
    }

    fn render(&self, _area: Rect, theme: &Theme, highlighted: bool) -> Paragraph<'static> {
        if matches!(self, Self::Empty {} | Self::EmptyHalf {}) {
            return Paragraph::new("");
        }

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

            Self::Empty {} => unreachable!(),
            Self::EmptyHalf {} => unreachable!(),

            Self::Wide {
                bottom_left,
                top_left,
                ..
            } => {
                format!("{}\n{}", bottom_left, top_left)
            }
        };

        let (style, border_style) = if highlighted {
            (
                Style::default()
                    .fg(theme.colors.selection_bg)
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(theme.colors.selection_bg),
            )
        } else {
            (
                Style::default().fg(theme.colors.text),
                Style::default().fg(theme.colors.border),
            )
        };

        Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(style)
            .block(
                Block::bordered()
                    .border_style(border_style)
                    .padding(Padding::horizontal(1)),
            )
    }
}

pub fn find_highlight(rows: &[Vec<Key>], target: &str) -> Option<(usize, usize)> {
    rows.iter().enumerate().find_map(|(row_index, row)| {
        row.iter()
            .position(|key| key.matches_label(target))
            .map(|key_index| (row_index, key_index))
    })
}

pub fn highlight_label(code: CtKeyCode) -> Option<String> {
    let label = match code {
        CtKeyCode::Char(' ') => "spacebar".to_string(),

        CtKeyCode::Char(c) => c.to_string(),

        CtKeyCode::Backspace => "backspace".to_string(),
        CtKeyCode::Tab | CtKeyCode::BackTab => "tab".to_string(),
        CtKeyCode::Enter => "enter".to_string(),
        CtKeyCode::Esc => "esc".to_string(),

        CtKeyCode::CapsLock => "caps lock".to_string(),

        CtKeyCode::Left => "←".to_string(),
        CtKeyCode::Right => "→".to_string(),
        CtKeyCode::Up => "↑".to_string(),
        CtKeyCode::Down => "↓".to_string(),

        CtKeyCode::Home => "home".to_string(),
        CtKeyCode::End => "end".to_string(),
        CtKeyCode::PageUp => "pag up".to_string(),
        CtKeyCode::PageDown => "pag dow".to_string(),

        CtKeyCode::Delete => "del".to_string(),
        CtKeyCode::Insert => "ins".to_string(),

        CtKeyCode::PrintScreen => "prt".to_string(),
        CtKeyCode::ScrollLock => "scr".to_string(),
        CtKeyCode::Pause => "pau".to_string(),

        CtKeyCode::F(n) => format!("f{n}"),

        CtKeyCode::Modifier(ModifierKeyCode::LeftShift) => "l-shift".to_string(),
        CtKeyCode::Modifier(ModifierKeyCode::RightShift) => "r-shift".to_string(),

        CtKeyCode::Modifier(ModifierKeyCode::LeftControl) => "ctrl".to_string(),
        CtKeyCode::Modifier(ModifierKeyCode::RightControl) => "ctrl".to_string(),

        CtKeyCode::Modifier(ModifierKeyCode::LeftAlt) => "alt".to_string(),
        CtKeyCode::Modifier(ModifierKeyCode::RightAlt) => "alt".to_string(),

        CtKeyCode::Modifier(ModifierKeyCode::LeftSuper) => "sup".to_string(),
        CtKeyCode::Modifier(ModifierKeyCode::RightSuper) => "sup".to_string(),

        CtKeyCode::Media(media) => xf86_name_for_media(media)?.to_string(),

        _ => return None,
    };

    Some(label)
}

#[derive(Debug, Clone)]
pub struct KeyPressInfo {
    pub keycode_label: String,
    pub keysym: Option<u32>,
    pub unicode: Option<u32>,
    pub state: u8,
}

impl KeyPressInfo {
    pub fn keysym_display(&self) -> String {
        self.keysym
            .map(|sym| sym.to_string())
            .unwrap_or_else(|| "--".to_string())
    }

    pub fn unicode_display(&self) -> String {
        match self.unicode {
            Some(cp) => format!("U+{cp:04X}"),
            None => "--".to_string(),
        }
    }

    pub fn state_display(&self) -> String {
        self.state.to_string()
    }
}

fn xf86_name_for_media(media: crossterm::event::MediaKeyCode) -> Option<&'static str> {
    use crossterm::event::MediaKeyCode::*;

    Some(match media {
        Play | PlayPause => "XF86AudioPlay",
        Pause => "XF86AudioPause",
        Reverse => "XF86AudioReverse",
        Stop => "XF86AudioStop",
        FastForward => "XF86AudioForward",
        Rewind => "XF86AudioRewind",
        TrackNext => "XF86AudioNext",
        TrackPrevious => "XF86AudioPrev",
        Record => "XF86AudioRecord",
        LowerVolume => "XF86AudioLowerVolume",
        RaiseVolume => "XF86AudioRaiseVolume",
        MuteVolume => "XF86AudioMute",
    })
}

fn xf86_name_for_modifier(modifier: ModifierKeyCode) -> Option<&'static str> {
    use ModifierKeyCode::*;

    Some(match modifier {
        LeftShift | RightShift => "Shift_L",
        LeftControl | RightControl => "Control_L",
        LeftAlt | RightAlt => "Alt_L",
        LeftSuper | RightSuper => "Super_L",
        LeftHyper | RightHyper => "Hyper_L",
        LeftMeta | RightMeta => "Meta_L",
        _ => return None,
    })
}

fn resolve_keysym(code: CtKeyCode) -> Option<xkb::Keysym> {
    if let CtKeyCode::Char(c) = code {
        let sym = xkb::utf32_to_keysym(c as u32);
        return (sym.raw() != 0).then_some(sym);
    }

    let name: String = match code {
        CtKeyCode::Backspace => "BackSpace".to_string(),
        CtKeyCode::Enter => "Return".to_string(),
        CtKeyCode::Left => "Left".to_string(),
        CtKeyCode::Right => "Right".to_string(),
        CtKeyCode::Up => "Up".to_string(),
        CtKeyCode::Down => "Down".to_string(),
        CtKeyCode::Home => "Home".to_string(),
        CtKeyCode::End => "End".to_string(),
        CtKeyCode::PageUp => "Prior".to_string(),
        CtKeyCode::PageDown => "Next".to_string(),
        CtKeyCode::Tab => "Tab".to_string(),
        CtKeyCode::BackTab => "ISO_Left_Tab".to_string(),
        CtKeyCode::Delete => "Delete".to_string(),
        CtKeyCode::Insert => "Insert".to_string(),
        CtKeyCode::Esc => "Escape".to_string(),
        CtKeyCode::CapsLock => "Caps_Lock".to_string(),
        CtKeyCode::ScrollLock => "Scroll_Lock".to_string(),
        CtKeyCode::NumLock => "Num_Lock".to_string(),
        CtKeyCode::PrintScreen => "Print".to_string(),
        CtKeyCode::Pause => "Pause".to_string(),
        CtKeyCode::Menu => "Menu".to_string(),
        CtKeyCode::KeypadBegin => "KP_Begin".to_string(),
        CtKeyCode::F(n) => format!("F{n}"),
        CtKeyCode::Media(media) => xf86_name_for_media(media)?.to_string(),
        CtKeyCode::Modifier(modifier) => xf86_name_for_modifier(modifier)?.to_string(),
        _ => return None,
    };

    let sym = xkb::keysym_from_name(&name, xkb::KEYSYM_NO_FLAGS);
    (sym.raw() != 0).then_some(sym)
}

pub fn describe_key_event(event: &KeyEvent) -> KeyPressInfo {
    let keysym = resolve_keysym(event.code);

    let keycode_label = match keysym {
        Some(sym) => {
            let name = xkb::keysym_get_name(sym);
            if name.is_empty() {
                format!("{:?}", event.code)
            } else {
                name
            }
        }
        None => format!("{:?}", event.code),
    };

    let unicode = keysym
        .map(xkb::keysym_to_utf32)
        .filter(|codepoint| *codepoint != 0);

    KeyPressInfo {
        keycode_label,
        keysym: keysym.map(|sym| sym.raw()),
        unicode,
        state: event.modifiers.bits(),
    }
}

pub fn render_keyboard(
    frame: &mut Frame,
    area: Rect,
    rows: &[Vec<Key>],
    theme: &Theme,
    highlighted: Option<(usize, usize)>,
) {
    let row_height = 4;

    let keyboard_width = rows.iter().map(|row| row_width(row)).max().unwrap_or(0);

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
            .enumerate()
            .map(|(i, key)| Constraint::Length(key.width()))
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
            let is_highlighted = highlighted == Some((row_index, key_index));

            frame.render_widget(key.render(key_area, theme, is_highlighted), key_area);
        }
    }
}

fn en_row1() -> Vec<Key> {
    vec![
        Key::new_fn("esc", ""),
        Key::new_empty(),
        Key::new_fn("f1", ""),
        Key::new_fn("f2", ""),
        Key::new_fn("f3", ""),
        Key::new_fn("f4", ""),
        Key::new_empty_half(),
        Key::new_fn("f5", ""),
        Key::new_fn("f6", ""),
        Key::new_fn("f7", ""),
        Key::new_fn("f8", ""),
        Key::new_empty_half(),
        Key::new_fn("f9", ""),
        Key::new_fn("f10", ""),
        Key::new_fn("f11", ""),
        Key::new_fn("f12", ""),
        Key::new_empty_half(),
        Key::new_fn("prt", "scr"),
        Key::new_fn("scr", "lck"),
        Key::new_fn("pau", "brk"),
    ]
}

fn en_row2() -> Vec<Key> {
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
        Key::wide("backspace", "", 13),
        Key::new_empty_half(),
        Key::new_fn("ins", ""),
        Key::new_fn("home", ""),
        Key::new_fn("pag", "up"),
    ]
}

fn en_row3() -> Vec<Key> {
    vec![
        Key::wide("tab", "", 13),
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
        Key::new_empty_half(),
        Key::new_fn("del", ""),
        Key::new_fn("end", ""),
        Key::new_fn("pag", "dow"),
    ]
}

fn en_row4() -> Vec<Key> {
    vec![
        Key::wide("caps lock", "", 13),
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
        Key::wide("enter", "", 14),
        Key::new_empty_half(),
        Key::new_empty(),
        Key::new_empty(),
        // Key::new_empty(),
    ]
}

fn en_row5() -> Vec<Key> {
    vec![
        Key::wide("l-shift", "", 13),
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
        Key::wide("r-shift", "", 21),
        Key::new_empty_half(),
        Key::new_empty(),
        Key::new_fn("↑", ""),
        Key::new_empty(),
    ]
}

fn en_row6() -> Vec<Key> {
    vec![
        Key::new_fn("ctrl", ""),
        Key::new_fn("super", ""),
        Key::new_fn("alt", ""),
        Key::wide("spacebar", "", 69),
        Key::new_fn("alt", ""),
        Key::new_fn("ctrl", ""),
        Key::new_empty_half(),
        Key::new_fn("←", ""),
        Key::new_fn("↓", ""),
        Key::new_fn("→", ""),
    ]
}

pub fn en_rows() -> Vec<Vec<Key>> {
    vec![
        en_row1(),
        en_row2(),
        en_row3(),
        en_row4(),
        en_row5(),
        en_row6(),
    ]
}

pub struct LayoutInfo {
    pub name: String,

    pub layout: String,
    pub variant: String,

    pub rows: Vec<Vec<Key>>,
}

#[derive(Debug, Deserialize)]
struct XkbLayout {
    layout: String,
    variant: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct XkbListData {
    layouts: Vec<XkbLayout>,
}

#[derive(Debug, Deserialize)]
struct SwayInput {
    #[serde(rename = "type")]
    kind: Option<String>,
    xkb_active_layout_name: Option<String>,
}

pub fn detect_current_layout(config: &Config) -> LayoutInfo {
    let detected = if is_wayland_session() {
        detect_wayland_layout(&config.current_layout_wayland, &config.list_layout)
    } else {
        detect_x11_layout(&config.current_layout_x11)
    };

    match detected {
        Ok((layout, variant)) => {
            let rows = compile_rows(&layout, &variant).unwrap_or_else(|err| {
                eprintln!("Failed to load keys for {layout} ({variant}): {err}");
                en_rows()
            });

            let name = if variant.is_empty() {
                layout.clone()
            } else {
                format!("{layout} ({variant})")
            };

            LayoutInfo {
                name,
                layout,
                variant,
                rows,
            }
        }

        Err(err) => {
            eprintln!("Failed to detect current layout: {err}");
            LayoutInfo {
                name: "English (US)".to_string(),
                layout: "us".to_string(),
                variant: String::new(),
                rows: en_rows(),
            }
        }
    }
}

fn is_wayland_session() -> bool {
    std::env::var_os("WAYLAND_DISPLAY").is_some() || std::env::var_os("SWAYSOCK").is_some()
}

fn detect_x11_layout(cmd: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let (program, args) = split_command(cmd, "setxkbmap", &["-query"]);

    let output = Command::new(&program).args(&args).output()?;

    if !output.status.success() {
        return Err(format!("{cmd} failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let text = String::from_utf8_lossy(&output.stdout);

    let mut layout = String::new();
    let mut variant = String::new();

    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("layout:") {
            layout = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("variant:") {
            variant = rest.trim().to_string();
        }
    }

    if layout.is_empty() {
        return Err(format!("{cmd} did not report a layout").into());
    }

    Ok((layout, variant))
}

fn detect_wayland_layout(
    cmd: &str,
    list_layout_cmd: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let (program, args) = split_command(cmd, "swaymsg", &["-t", "get_inputs"]);

    let output = Command::new(&program).args(&args).output()?;

    if !output.status.success() {
        return Err(format!("{cmd} failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let inputs: Vec<SwayInput> = serde_yaml::from_slice(&output.stdout)?;

    let active_name = inputs
        .into_iter()
        .find(|input| input.kind.as_deref() == Some("keyboard"))
        .and_then(|input| input.xkb_active_layout_name)
        .ok_or("swaymsg reported no active keyboard layout")?;

    let layouts = list_xkb_layouts(list_layout_cmd)?;

    let matched = layouts
        .into_iter()
        .find(|candidate| candidate.description.eq_ignore_ascii_case(&active_name))
        .ok_or_else(|| format!("no layout in `{list_layout_cmd}` matches '{active_name}'"))?;

    Ok((matched.layout, matched.variant))
}

fn list_xkb_layouts(list_layout_cmd: &str) -> Result<Vec<XkbLayout>, Box<dyn std::error::Error>> {
    let (program, args) = split_command(list_layout_cmd, "xkbcli", &["list"]);

    let output = Command::new(&program).args(&args).output()?;

    if !output.status.success() {
        return Err(format!(
            "{list_layout_cmd} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let data: XkbListData = serde_yaml::from_slice(&output.stdout)?;

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

const ROW2_NAMES: [Option<&str>; 13] = [
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

const ROW3_NAMES: [Option<&str>; 12] = [
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
const ROW3_BKSL: &str = "BKSL";

const ROW4_NAMES: [Option<&str>; 11] = [
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

const ROW5_NAMES: [Option<&str>; 10] = [
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

fn row_width(row: &[Key]) -> u16 {
    row.iter()
        .enumerate()
        .map(|(i, key)| key.width() + if i + 1 < row.len() { 1 } else { 0 })
        .sum()
}

fn build_rows(levels: &HashMap<String, Vec<String>>) -> Vec<Vec<Key>> {
    let row1 = en_row1();

    let row2 = en_row2();
    let row2 = {
        let backspace = row2[13].clone();
        let trailing = row2[14..].to_vec();
        let mut translated = translate_row(row2[..13].to_vec(), &ROW2_NAMES, levels);
        translated.push(backspace);
        translated.extend(trailing);
        translated
    };

    let row3 = en_row3();
    let row3 = {
        let tab = row3[0].clone();
        let bksl = row3[13].clone();
        let trailing = row3[14..].to_vec();
        let mut translated = vec![tab];
        translated.extend(translate_row(row3[1..13].to_vec(), &ROW3_NAMES, levels));
        translated.push(translated_key(levels, ROW3_BKSL, bksl));
        translated.extend(trailing);
        translated
    };

    let row4 = en_row4();
    let row4 = {
        let caps = row4[0].clone();
        let enter = row4[12].clone();
        let trailing = row4[13..].to_vec();
        let mut translated = vec![caps];
        translated.extend(translate_row(row4[1..12].to_vec(), &ROW4_NAMES, levels));
        translated.push(enter);
        translated.extend(trailing);
        translated
    };

    let row5 = en_row5();
    let row5 = {
        let l_shift = row5[0].clone();
        let r_shift = row5[11].clone();
        let trailing = row5[12..].to_vec();
        let mut translated = vec![l_shift];
        translated.extend(translate_row(row5[1..11].to_vec(), &ROW5_NAMES, levels));
        translated.push(r_shift);
        translated.extend(trailing);
        translated
    };

    let row6 = en_row6();

    vec![row1, row2, row3, row4, row5, row6]
}
