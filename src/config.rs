use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_list_layout")]
    pub list_layout: String,
    #[serde(default = "default_current_layout_x11")]
    pub current_layout_x11: String,
    #[serde(default = "default_current_layout_wayland")]
    pub current_layout_wayland: String,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_list_layout() -> String {
    "xkbcli list".to_string()
}

fn default_current_layout_x11() -> String {
    "setxkbmap -query".to_string()
}

fn default_current_layout_wayland() -> String {
    "swaymsg -t get_inputs".to_string()
}

fn default_theme() -> String {
    "catppuccin-mocha".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            list_layout: default_list_layout(),
            current_layout_x11: default_current_layout_x11(),
            current_layout_wayland: default_current_layout_wayland(),
            theme: default_theme(),
        }
    }
}

const SAMPLE_CONFIG: &str = include_str!("../caiman_config.toml.sample");

fn config_path() -> Option<std::path::PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(std::path::PathBuf::from(home).join(".config/rama/caiman_config.toml"))
}

pub fn load_config() -> Config {
    let Some(path) = config_path() else {
        return Config::default();
    };

    if !path.exists() {
        if let Some(parent) = path.parent() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                eprintln!("Failed to create {}: {err}", parent.display());
            }
        }

        if let Err(err) = std::fs::write(&path, SAMPLE_CONFIG) {
            eprintln!("Failed to write {}: {err}", path.display());
        }
    }

    match std::fs::read_to_string(&path) {
        Ok(raw) => match toml::from_str(&raw) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Failed to parse {}: {err}", path.display());
                Config::default()
            }
        },
        Err(err) => {
            eprintln!("Failed to read {}: {err}", path.display());
            Config::default()
        }
    }
}
