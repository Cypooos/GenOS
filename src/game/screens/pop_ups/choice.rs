use crate::game::screens::{screens::Screen, Screenable, SA};
use crate::io::vga_writer::Color;
use crate::io::{vga_writer, vga_writer::WRITER};
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use crate::game::visuals::boxes::{box_double, box_simple};

use pc_keyboard::{KeyCode, KeyEvent, KeyState};

pub struct ChoicePopUp {
    pub name: String,
    pub escapable: Option<Vec<SA>>,
    pub size: (usize, usize),
    pub pos: (usize, usize),
    pub content: Vec<String>,
    pub options: Vec<(String, Vec<SA>)>,
    selected: usize,
}

impl ChoicePopUp {
    pub fn new(
        name: &str,
        escapable: Option<Vec<SA>>,
        size: (usize, usize),
        pos: (usize, usize),
        content: Vec<String>,
        options: Vec<(String, Vec<SA>)>,
    ) -> Self {
        Self {
            name: name.to_string(),
            escapable,
            content,
            size,
            pos,
            options,
            selected: 0,
        }
    }
}

impl Screenable for ChoicePopUp {
    fn init(&mut self) -> Option<Vec<SA>> {
        self.draw();
        None
        //Some(vec![SA::Draw])
    }
    fn draw(&self) {
        vga_colors!(Some(Color::Cyan), Some(Color::White));
        box_simple(self.pos, self.size);
        // name, tl corner & escape
        vga_write!(self.pos.0 + 1, self.pos.1, " {} ", self.name);
        if self.escapable.is_some() {
            vga_write!(self.pos.0 + self.size.0 - 5, self.pos.1, "[$34Esc$3B]");
        }
        // Content
        let mut x = 0;
        for line in &self.content {
            vga_write!(
                self.pos.0 + 1,
                self.pos.1 + x + 1,
                "{:^width$}",
                line,
                width = self.size.0 - 1
            );
            x += 1
        }
        // choices
        let mut x = 0;
        let padding = (self.size.0 - 4) / self.options.len();
        for out in &self.options {
            if x == self.selected {
                box_double(
                    (self.pos.0 + x * padding + 2, self.pos.1 + self.size.1 - 2),
                    (padding - 1, 1),
                )
            } else {
                box_simple(
                    (self.pos.0 + x * padding + 2, self.pos.1 + self.size.1 - 2),
                    (padding - 1, 1),
                )
            }
            x += 1;
        }
    }
    fn on_time(&mut self, _time: u8) -> Option<Vec<SA>> {
        None
    }
    fn on_key(&mut self, key_event: KeyEvent, _as_char: Option<char>) -> Option<Vec<SA>> {
        // Detect key
        if key_event.state == KeyState::Down {
            match key_event.code {
                KeyCode::ArrowLeft => {
                    self.selected = self.selected.checked_sub(1).unwrap_or(self.options.len());
                    None
                }
                KeyCode::ArrowRight => {
                    self.selected = self.selected + 1 % self.options.len();
                    None
                }
                KeyCode::Spacebar | KeyCode::Enter => Some(self.options[self.selected].1.clone()),
                _ => None,
            }
        } else {
            None
        }
    }
}
