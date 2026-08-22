use ratatui::style::Color;
use serde::{Deserialize, Deserializer};

// ============================================================
// Theme data model
// ============================================================

#[derive(Debug, Clone, Deserialize)]
pub struct Theme {
    pub name: String,
    pub author: String,
    pub version: u32,
    pub colors: ThemeColors,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeColors {
    #[serde(deserialize_with = "deserialize_color")]
    pub background: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub text: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub border: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub header: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub accent: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub warning: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub error: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub success: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub selection_fg: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub selection_bg: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub claws: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub claws_light: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub claws_shadow: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub shell: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub shell_light: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub shell_shadow: Color,
}

impl Theme {
    /// Used only if literally nothing could be loaded (no user themes, no
    /// bundled themes parsed successfully). Keeps the UI on the terminal's
    /// own default colors rather than crashing.
    pub fn fallback() -> Self {
        Self {
            name: "Default".to_string(),
            author: "caiman".to_string(),
            version: 1,
            colors: ThemeColors {
                background: Color::Reset,
                text: Color::Reset,
                border: Color::DarkGray,
                header: Color::White,
                accent: Color::Blue,
                warning: Color::Yellow,
                error: Color::Red,
                success: Color::Green,
                selection_fg: Color::Black,
                selection_bg: Color::White,
                claws: Color::Blue,
                claws_light: Color::LightBlue,
                claws_shadow: Color::DarkGray,
                shell: Color::DarkGray,
                shell_light: Color::Gray,
                shell_shadow: Color::Black,
            },
        }
    }
}

// ============================================================
// Hex color parsing
// ============================================================

/// Parse a `"#RRGGBB"` (or `"RRGGBB"`) string into a ratatui `Color`.
pub fn parse_hex_color(raw: &str) -> Option<Color> {
    let hex = raw.trim().trim_start_matches('#');

    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Color::Rgb(r, g, b))
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;

    parse_hex_color(&raw)
        .ok_or_else(|| serde::de::Error::custom(format!("invalid hex color: {raw:?}")))
}

// ============================================================
// Theme discovery
// ============================================================

/// Themes bundled with caiman itself, embedded at compile time. These are
/// written out to `~/.config/caiman/themes/` the first time caiman runs
/// (see `install_bundled_themes_if_missing`) and act as the fallback set
/// if that directory is ever missing or emptied out afterwards.
const BUNDLED_THEMES: &[(&str, &str)] = &[
    (
        "catppuccin_mocha.toml",
        include_str!("../themes/catppuccin_mocha.toml"),
    ),
    (
        "catppuccin_latte.toml",
        include_str!("../themes/catppuccin_latte.toml"),
    ),
    ("dracula.toml", include_str!("../themes/dracula.toml")),
    ("mako.toml", include_str!("../themes/mako.toml")),
    (
        "melange_dark.toml",
        include_str!("../themes/melange_dark.toml"),
    ),
    (
        "pomboverso.toml",
        include_str!("../themes/pomboverso.toml"),
    ),
    ("rama.toml", include_str!("../themes/rama.toml")),
    ("teyin.toml", include_str!("../themes/teyin.toml")),
    (
        "tokyo_night.toml",
        include_str!("../themes/tokyo_night.toml"),
    ),
];

fn parse_theme_toml(raw: &str) -> Result<Theme, toml::de::Error> {
    toml::from_str(raw)
}

fn load_bundled_themes() -> Vec<Theme> {
    BUNDLED_THEMES
        .iter()
        .filter_map(|(_, raw)| match parse_theme_toml(raw) {
            Ok(theme) => Some(theme),
            Err(err) => {
                eprintln!("Failed to parse bundled theme: {err}");
                None
            }
        })
        .collect()
}

/// `~/.config/caiman/themes/` — same directory `config::load_config` seeds
/// `config.toml` into, just for theme files instead.
fn user_themes_dir() -> Option<std::path::PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(std::path::PathBuf::from(home).join(".config/caiman/themes"))
}

/// Copy the bundled themes into `~/.config/caiman/themes/` the first time
/// caiman runs and that directory doesn't exist yet — the theme
/// counterpart to `config::load_config` seeding `config.toml` from the
/// bundled sample. After this, the files on disk are what's actually
/// used and editable; the embedded copies only matter as this one-time
/// seed and as an ultimate fallback if the directory is ever emptied out.
pub fn install_bundled_themes_if_missing() {
    let Some(dir) = user_themes_dir() else {
        return;
    };

    if dir.exists() {
        return;
    }

    if let Err(err) = std::fs::create_dir_all(&dir) {
        eprintln!("Failed to create {}: {err}", dir.display());
        return;
    }

    for (filename, contents) in BUNDLED_THEMES {
        let path = dir.join(filename);
        if let Err(err) = std::fs::write(&path, contents) {
            eprintln!("Failed to write {}: {err}", path.display());
        }
    }
}

fn load_user_themes() -> Vec<Theme> {
    let Some(dir) = user_themes_dir() else {
        return Vec::new();
    };

    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };

    entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "toml"))
        .filter_map(|entry| match std::fs::read_to_string(entry.path()) {
            Ok(raw) => match parse_theme_toml(&raw) {
                Ok(theme) => Some(theme),
                Err(err) => {
                    eprintln!("Failed to parse theme {}: {err}", entry.path().display());
                    None
                }
            },
            Err(err) => {
                eprintln!("Failed to read theme {}: {err}", entry.path().display());
                None
            }
        })
        .collect()
}

/// Discover the available themes.
///
/// Reads from `~/.config/caiman/themes/`, which `install_bundled_themes_if_missing`
/// seeds with the bundled themes the first time caiman runs — so in
/// practice this is what actually gets used, and it's what the person
/// edits/adds to. Falls back to the themes embedded in the binary directly
/// if that directory is somehow still missing or was emptied out, so the
/// theme picker always has something to show.
pub fn discover_themes() -> Vec<Theme> {
    let mut themes = load_user_themes();

    if themes.is_empty() {
        themes = load_bundled_themes();
    }

    themes.sort_by(|a, b| a.name.cmp(&b.name));

    if themes.is_empty() {
        themes.push(Theme::fallback());
    }

    themes
}

fn slugify(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            c if c.is_whitespace() || c == '_' => '-',
            c => c.to_ascii_lowercase(),
        })
        .collect()
}

/// Find the index of the theme matching a config value like
/// `"catppuccin-mocha"` against a theme named `"Catppuccin Mocha"`
/// (case/whitespace-insensitive). Falls back to `0` if nothing matches.
pub fn find_theme_index(themes: &[Theme], wanted: &str) -> usize {
    let wanted = slugify(wanted);
    themes
        .iter()
        .position(|t| slugify(&t.name) == wanted)
        .unwrap_or(0)
}
