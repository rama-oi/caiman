use std::io;

use crate::config::{Config, load_config};
use crate::input::index::handle_index_input;
use crate::input::settings::handle_settings_input;
use crate::input::settings_about::handle_settings_about_input;
use crate::input::settings_theme::handle_settings_themes_input;
use crate::theme::{Theme, discover_themes, find_theme_index};
use crate::ui::index::draw_index;
use crate::ui::keyboard::{LayoutInfo, discover_layouts};
use crate::ui::settings::draw_settings;
use crate::ui::settings_about::draw_settings_about;
use crate::ui::settings_theme::draw_settings_themes;

use crossterm::event::{self, Event};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::ListState};

pub enum Screen {
    Index,
    Settings,
    Themes,
    About,
}
pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub command_mode: bool,
    pub selected_layout: usize,
    pub layouts: Vec<LayoutInfo>,
    pub layout_list_state: ListState,
    pub search_query: String,
    pub status_message: Option<String>,
    pub themes: Vec<Theme>,
    pub selected_theme: usize,
    pub theme_list_state: ListState,
    pub settings_list_state: ListState,
    pub config: Config,
}

impl App {
    /// The currently active theme. `themes` always has at least one entry
    /// (see `discover_themes`), so this never has to fall back to a
    /// hardcoded default.
    pub fn theme(&self) -> &Theme {
        self.themes
            .get(self.selected_theme)
            .unwrap_or(&self.themes[0])
    }

    /// Indices into `layouts` of the layouts matching the current search
    /// query (case-insensitive substring match against id/name/xkb layout
    /// code). Returns every index when the query is empty.
    pub fn filtered_layout_indices(&self) -> Vec<usize> {
        let query = self.search_query.trim().to_lowercase();

        if query.is_empty() {
            return (0..self.layouts.len()).collect();
        }

        self.layouts
            .iter()
            .enumerate()
            .filter(|(_, layout)| {
                layout
                    .id
                    .to_lowercase()
                    .split('_')
                    .any(|part| part.starts_with(&query))
            })
            .map(|(index, _)| index)
            .collect()
    }
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let config = load_config();
    let themes = discover_themes();
    let selected_theme = find_theme_index(&themes, &config.theme);

    let mut app = App {
        screen: Screen::Index,
        should_quit: false,
        command_mode: false,
        selected_layout: 0,
        layouts: discover_layouts(&config.list_layout),
        layout_list_state: ListState::default().with_selected(Some(0)),
        search_query: String::new(),
        status_message: None,
        themes,
        selected_theme,
        theme_list_state: ListState::default().with_selected(Some(selected_theme)),
        settings_list_state: ListState::default().with_selected(Some(0)),
        config,
    };

    loop {
        terminal.draw(|frame| match app.screen {
            Screen::Index => draw_index(frame, &mut app),
            Screen::Settings => draw_settings(frame, &mut app),
            Screen::About => draw_settings_about(frame, &mut app),
            Screen::Themes => draw_settings_themes(frame, &mut app),
        })?;

        if let Event::Key(key) = event::read()? {
            match app.screen {
                Screen::Index => handle_index_input(&mut app, key.code),
                Screen::Settings => handle_settings_input(&mut app, key.code),
                Screen::About => handle_settings_about_input(&mut app, key.code),
                Screen::Themes => handle_settings_themes_input(&mut app, key.code),
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
