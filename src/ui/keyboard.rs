use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Padding, Paragraph},
};

use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

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

    // The actual keys for this layout, loaded from the system via the
    // `xkbcli` command-line tool. Falls back to `en_rows()` if loading
    // fails.
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

/// Discover the layouts installed on the system and, for each one, its
/// actual key mappings — both entirely via OS commands (`xkbcli`), no
/// linked keyboard-handling library required.
///
/// `list_layout_cmd` is the user-configurable command (see
/// `Config::list_layout`, defaults to `"xkbcli list"`) used to enumerate
/// installed layouts.
pub fn discover_layouts(list_layout_cmd: &str) -> Vec<LayoutInfo> {
    match get_layouts(list_layout_cmd) {
        Ok(mut layouts) => {
            println!("{} layouts", layouts.len());

            // Sort by language/group first, then layout, then variant.
            // This keeps e.g. en_* layouts together.
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

                    let rows = compile_rows(&layout.layout, &layout.variant).unwrap_or_else(|err| {
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

/// Split a user-configured command string like `"xkbcli list"` into a
/// program name and its arguments. Falls back to `fallback_program`
/// `fallback_args` if the configured command is blank.
fn split_command(cmd: &str, fallback_program: &str, fallback_args: &[&str]) -> (String, Vec<String>) {
    let mut parts = cmd.split_whitespace();

    match parts.next() {
        Some(program) => (program.to_string(), parts.map(str::to_string).collect()),
        None => (
            fallback_program.to_string(),
            fallback_args.iter().map(|s| s.to_string()).collect(),
        ),
    }
}

// ============================================================
// XKB keymap loading
// ============================================================

/// Compile an XKB layout via `xkbcli compile-keymap` and read back the
/// resulting keymap as text.
///
/// This does NOT change the system keyboard layout. It just asks
/// libxkbcommon (through its CLI, not a linked library) to resolve the
/// given layout/variant into a full keymap, which we then parse ourselves
/// to find out what each key actually produces.
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

/// Compile a layout and translate it straight into visual keyboard rows.
fn compile_rows(layout: &str, variant: &str) -> Result<Vec<Vec<Key>>, Box<dyn std::error::Error>> {
    let compiled = compile_keymap_text(layout, variant)?;
    let levels = parse_symbol_levels(&compiled);
    Ok(build_rows(&levels))
}

/// Parse every `key <NAME> { ... [ sym1, sym2, ... ] ... };` statement out
/// of a compiled XKB keymap's text (as printed by `xkbcli compile-keymap`),
/// keeping the *last* definition of each key name — later statements
/// (e.g. a layout's own overrides) win, matching how XKB itself resolves
/// them.
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

        // Bound the search for this key's symbol list to its own
        // statement, so we don't wander into the next key's brackets.
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

/// Within a single `key <NAME> { ... }` body, find the symbol list — the
/// first `[ ... ]` group whose opening bracket directly follows `=` or
/// `{` (skipping index brackets like the `[group1]` in `type[group1] =`).
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

// ============================================================
// Keysym name -> display text
// ============================================================

/// Best-effort translation of an XKB keysym *name* (as found in a compiled
/// keymap's `xkb_symbols` section, e.g. "at", "eacute", "U2018") into the
/// character it represents on screen.
///
/// Single-character keysym names pass straight through — that covers every
/// plain letter and digit ("q", "Q", "5", ...), since those are their own
/// keysym name in XKB. Everything else is looked up in a table of the
/// common Latin/Latin-1 mnemonic names; anything not covered there (most
/// non-Latin scripts, dead keys, etc.) just shows its raw keysym name —
/// the same fallback the old libxkbcommon-based code used whenever a
/// keysym had no direct Unicode representation.
fn keysym_name_to_display(name: &str) -> String {
    let name = name.trim();

    if name.is_empty() || name == "NoSymbol" {
        return String::new();
    }

    if name.chars().count() == 1 {
        return name.to_string();
    }

    if let Some(ch) = named_keysym(name) {
        return ch.to_string();
    }

    // XKB's convention for keysyms with no classic mnemonic: "U" followed
    // by 4-8 hex digits of the Unicode code point.
    if let Some(hex) = name.strip_prefix('U') {
        if (4..=8).contains(&hex.len()) && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            if let Ok(code) = u32::from_str_radix(hex, 16) {
                if let Some(ch) = char::from_u32(code) {
                    return ch.to_string();
                }
            }
        }
    }

    name.to_string()
}

fn named_keysym(name: &str) -> Option<char> {
    Some(match name {
        "space" => ' ',
        "exclam" => '!',
        "quotedbl" => '"',
        "numbersign" => '#',
        "dollar" => '$',
        "percent" => '%',
        "ampersand" => '&',
        "apostrophe" | "quoteright" => '\'',
        "parenleft" => '(',
        "parenright" => ')',
        "asterisk" => '*',
        "plus" => '+',
        "comma" => ',',
        "minus" => '-',
        "period" => '.',
        "slash" => '/',
        "colon" => ':',
        "semicolon" => ';',
        "less" => '<',
        "equal" => '=',
        "greater" => '>',
        "question" => '?',
        "at" => '@',
        "bracketleft" => '[',
        "backslash" => '\\',
        "bracketright" => ']',
        "asciicircum" => '^',
        "underscore" => '_',
        "grave" | "quoteleft" => '`',
        "braceleft" => '{',
        "bar" => '|',
        "braceright" => '}',
        "asciitilde" => '~',

        "nobreakspace" => '\u{a0}',
        "exclamdown" => '¡',
        "cent" => '¢',
        "sterling" => '£',
        "currency" => '¤',
        "yen" => '¥',
        "brokenbar" => '¦',
        "section" => '§',
        "diaeresis" => '¨',
        "copyright" => '©',
        "ordfeminine" => 'ª',
        "guillemotleft" => '«',
        "notsign" => '¬',
        "registered" => '®',
        "macron" => '¯',
        "degree" => '°',
        "plusminus" => '±',
        "twosuperior" => '²',
        "threesuperior" => '³',
        "acute" => '´',
        "mu" => 'µ',
        "paragraph" => '¶',
        "periodcentered" => '·',
        "cedilla" => '¸',
        "onesuperior" => '¹',
        "masculine" => 'º',
        "guillemotright" => '»',
        "onequarter" => '¼',
        "onehalf" => '½',
        "threequarters" => '¾',
        "questiondown" => '¿',

        "Agrave" => 'À',
        "Aacute" => 'Á',
        "Acircumflex" => 'Â',
        "Atilde" => 'Ã',
        "Adiaeresis" => 'Ä',
        "Aring" => 'Å',
        "AE" => 'Æ',
        "Ccedilla" => 'Ç',
        "Egrave" => 'È',
        "Eacute" => 'É',
        "Ecircumflex" => 'Ê',
        "Ediaeresis" => 'Ë',
        "Igrave" => 'Ì',
        "Iacute" => 'Í',
        "Icircumflex" => 'Î',
        "Idiaeresis" => 'Ï',
        "ETH" | "Eth" => 'Ð',
        "Ntilde" => 'Ñ',
        "Ograve" => 'Ò',
        "Oacute" => 'Ó',
        "Ocircumflex" => 'Ô',
        "Otilde" => 'Õ',
        "Odiaeresis" => 'Ö',
        "multiply" => '×',
        "Ooblique" => 'Ø',
        "Ugrave" => 'Ù',
        "Uacute" => 'Ú',
        "Ucircumflex" => 'Û',
        "Udiaeresis" => 'Ü',
        "Yacute" => 'Ý',
        "THORN" | "Thorn" => 'Þ',
        "ssharp" => 'ß',

        "agrave" => 'à',
        "aacute" => 'á',
        "acircumflex" => 'â',
        "atilde" => 'ã',
        "adiaeresis" => 'ä',
        "aring" => 'å',
        "ae" => 'æ',
        "ccedilla" => 'ç',
        "egrave" => 'è',
        "eacute" => 'é',
        "ecircumflex" => 'ê',
        "ediaeresis" => 'ë',
        "igrave" => 'ì',
        "iacute" => 'í',
        "icircumflex" => 'î',
        "idiaeresis" => 'ï',
        "eth" => 'ð',
        "ntilde" => 'ñ',
        "ograve" => 'ò',
        "oacute" => 'ó',
        "ocircumflex" => 'ô',
        "otilde" => 'õ',
        "odiaeresis" => 'ö',
        "division" => '÷',
        "oslash" => 'ø',
        "ugrave" => 'ù',
        "uacute" => 'ú',
        "ucircumflex" => 'û',
        "udiaeresis" => 'ü',
        "yacute" => 'ý',
        "thorn" => 'þ',
        "ydiaeresis" => 'ÿ',

        "EuroSign" => '€',

        _ => return None,
    })
}

// ============================================================
// Applying a layout to the running system
// ============================================================

/// Actually switch the system's active keyboard layout.
///
/// This targets Sway, via its IPC (`swaymsg`, or whatever
/// `Config::switch_layout` is set to), which is the mechanism Sway itself
/// uses under the hood — there's no X11 involved, so tools like
/// `setxkbmap` don't apply here. `type:keyboard` targets every connected
/// keyboard rather than a single device id, since most setups only have
/// one and it saves having to look an id up via `swaymsg -t get_inputs`
/// first.
///
/// `switch_layout_cmd` is the user-configurable program name (see
/// `Config::switch_layout`, defaults to `"swaymsg"`) — the arguments
/// themselves are still built here, since they're compositor-specific.
/// (If you switch compositors later — Hyprland, KDE, GNOME, etc. — this
/// is the one place that needs to change; each has its own equivalent of
/// this call.)
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

    // Always set the variant explicitly (even to ""), so switching from a
    // layout with a variant back to the plain layout clears the old one
    // instead of leaving it stuck. When empty, we have to hand swaymsg a
    // literal `""` token — an empty Rust string argument just disappears
    // when swaymsg joins argv into the command text, turning
    // `xkb_variant ""` into `xkb_variant` (missing its value entirely).
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

    // swaymsg exits 0 even when sway rejects the command outright; the
    // actual reason (if any) shows up in the JSON it prints to stdout, so
    // we have to check that too, not just the exit status.
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
            message.push_str(&format!("{program} exited with an error but printed nothing"));
        }

        return Err(format!("{program} failed: {message}").into());
    }

    Ok(())
}

// ============================================================
// Translating a compiled keymap into the visual keyboard rows
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

/// Build a `Key::Normal` for the given canonical XKB key name, using the
/// unshifted/shifted symbol levels from `levels`. Falls back to
/// `fallback` (the US QWERTY key at the same position) if the key can't
/// be found or the layout doesn't define it.
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

fn translate_row(row: Vec<Key>, names: &[Option<&str>], levels: &HashMap<String, Vec<String>>) -> Vec<Key> {
    row.into_iter()
        .enumerate()
        .map(|(i, key)| match names.get(i).copied().flatten() {
            Some(name) => translated_key(levels, name, key),
            None => key,
        })
        .collect()
}

/// Build the visual keyboard rows for a compiled layout, translating every
/// alphanumeric/symbol key to what that layout actually produces while
/// keeping function/modifier keys (backspace, tab, enter, shift, etc.)
/// labeled the same regardless of layout.
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

    // The bottom row (ctrl/super/alt/spacebar) has no printable keys, so it
    // stays the same regardless of layout.
    let row5 = en_row5();

    vec![row1, row2, row3, row4, row5]
}
