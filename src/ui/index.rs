use crate::util::wrap_help_items;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, List, Paragraph},
};

const HELP_ITEMS: &[&str] = &[
    "[↑↓] navigate",
    "[space] select_layout",
    "[tab] switch_section",
    "[s] settings",
    "[q] quit",
];

use crate::app::App;

pub fn draw_index(frame: &mut Frame, app: &mut App) {
    let full_area = frame.area();

    let help_width = full_area.width.saturating_sub(2);
    let help_lines = wrap_help_items(HELP_ITEMS, help_width);
    let help_height = help_lines.len() as u16;

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Length(help_height),
        ])
        .split(full_area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Fill(1)])
        .split(vertical[0]);

    let layout_list =
        List::new(["● en", "○ es", "○ pt"]).block(Block::bordered().title(" switch layout "));
    frame.render_widget(layout_list, horizontal[0]);

    let keyboard_text = r#"┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌───────────┐
│ ~ ~ ││ ! ! ││ @ @ ││ # # ││ $ $ ││ % % ││ ^ ^ ││ & & ││ * * ││ ( ( ││ ) ) ││ _ _ ││ + + ││ backspace │
│ ` ` ││ 1 1 ││ 2 2 ││ 3 3 ││ 4 4 ││ 5 5 ││ 6 6 ││ 7 7 ││ 8 8 ││ 9 9 ││ 0 0 ││ - - ││ = = ││ backspace │
└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└───────────┘
┌───────────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐
│    tab    ││ Q Q ││ W W ││ E E ││ R R ││ T T ││ Y Y ││ U U ││ I I ││ O O ││ P P ││ { { ││ } } ││ | | │
│    tab    ││ q q ││ w w ││ e e ││ r r ││ t t ││ y y ││ u u ││ i i ││ o o ││ p p ││ [ [ ││ ] ] ││ \ \ │
└───────────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘
┌───────────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┏━━━━━┓┌─────┐┌─────┐┌─────┐┌────────────┐
│ caps lock ││ A A ││ S S ││ D D ││ F F ││ G G ││ H H ││ J J │┃ K K ┃│ L L ││ : : ││ " " ││    enter   │
│ caps lock ││ a a ││ s s ││ d d ││ f f ││ g g ││ h h ││ j j │┃ k k ┃│ l l ││ ; ; ││ ' ' ││    enter   │
└───────────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘┗━━━━━┛└─────┘└─────┘└─────┘└────────────┘
┌───────────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌───────────────────┐
│  l-shift  ││ Z Z ││ X X ││ C C ││ V V ││ B B ││ N N ││ M M ││ < < ││ > > ││ ? ? ││      r-shift      │
│  l-shift  ││ z z ││ x x ││ c c ││ v v ││ b b ││ n n ││ m m ││ , , ││ . . ││ / / ││      r-shift      │
└───────────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└─────┘└───────────────────┘
┌─────┐┌─────┐┌─────┐┌───────────────────────────────────────────────────────────────────┐┌─────┐┌─────┐
│ ctr ││ sup ││ alt ││                              spacebar                             ││ alt ││ ctr │
│ ctr ││ sup ││ alt ││                              spacebar                             ││ alt ││ ctr │
└─────┘└─────┘└─────┘└───────────────────────────────────────────────────────────────────┘└─────┘└─────┘"#;

    let keyboard_block = Block::bordered().title(" current layout ");
    let keyboard_inner = keyboard_block.inner(horizontal[1]);

    frame.render_widget(keyboard_block, horizontal[1]);

    let keyboard_height = keyboard_text.lines().count() as u16;

    // Crop each line to the available width, keeping the crop centered.
    let keyboard_text = keyboard_text
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let width = keyboard_inner.width as usize;

            if chars.len() <= width {
                line.to_string()
            } else {
                let start = (chars.len() - width) / 2;

                chars[start..start + width].iter().collect()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    let keyboard_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(keyboard_height),
            Constraint::Fill(1),
        ])
        .split(keyboard_inner);

    let keyboard = Paragraph::new(keyboard_text).alignment(Alignment::Center);

    frame.render_widget(keyboard, keyboard_area[1]);

    let key_preview = List::new(["key preview"]).block(Block::bordered());

    frame.render_widget(key_preview, vertical[1]);

    let help_text = help_lines
        .iter()
        .map(|line| format!(" {line}"))
        .collect::<Vec<_>>()
        .join("\n");

    let help = Paragraph::new(help_text);
    frame.render_widget(help, vertical[2]);
}
