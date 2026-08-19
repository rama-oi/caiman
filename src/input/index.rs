use crossterm::event::KeyCode;

use crate::app::App;

pub fn handle_index_input(app: &mut App, key: KeyCode) {
    match key {
        // KeyCode::Char(':') => {
        //     app.status = None;
        //     app.command_mode = true;
        //     app.command_buffer.clear();
        // }
        _ => {}
    }
}
