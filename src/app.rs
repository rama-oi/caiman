use std::io;

use crate::input::index::handle_index_input;
use crate::input::settings::handle_settings_input;
use crate::ui::index::draw_index;
use crate::ui::keyboard::{LayoutInfo, discover_layouts};
use crate::ui::settings::draw_settings;

use crossterm::event::{self, Event};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::ListState};

pub enum Screen {
    Index,
    Settings,
}
pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub command_mode: bool,
    pub selected_layout: usize,
    pub layouts: Vec<LayoutInfo>,
    pub layout_list_state: ListState,
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App {
        screen: Screen::Index,
        should_quit: false,
        command_mode: false,
        selected_layout: 0,
        layouts: discover_layouts(),
        layout_list_state: ListState::default().with_selected(Some(0)),
    };

    loop {
        terminal.draw(|frame| match app.screen {
            Screen::Index => draw_index(frame, &mut app),
            Screen::Settings => draw_settings(frame, &mut app),
        })?;

        if let Event::Key(key) = event::read()? {
            match app.screen {
                Screen::Index => handle_index_input(&mut app, key.code),
                Screen::Settings => handle_settings_input(&mut app, key.code),
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
