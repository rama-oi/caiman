// use crossterm::event::KeyCode;

// use crate::app::App;
// use crate::input::command::{handle_command_input, preview_entry};

// pub fn handle_index_input(app: &mut App, key: KeyCode) {
//     if app.command_mode {
//         handle_command_input(app, key);
//         return;
//     }

//     match key {
//         KeyCode::Char(':') => {
//             app.status = None;
//             app.command_mode = true;
//             app.command_buffer.clear();
//         }
//         _ => {}
//     }
// }
