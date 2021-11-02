use crate::game::screens::{screens::Screen, Screenable, SA};
use crate::io::{vga_writer, vga_writer::WRITER};
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use lazy_static::lazy_static;
use pc_keyboard::{KeyCode, KeyEvent, KeyState};
use spin::Mutex;

pub struct PopUp {
    pub name: String,
    pub content: Vec<String>,
    pub size: (usize, usize),
    pub pos: (usize, usize),
    pub next: Vec<SA>,
}

impl PopUp {
    pub fn new(
        name: &str,
        size: (usize, usize),
        pos: (usize, usize),
        content: Vec<String>,
        next: Vec<SA>,
    ) -> Self {
        Self {
            name: name.to_string(),
            content,
            size,
            pos,
            next,
        }
    }
}

impl Screenable for PopUp {
    fn init(&mut self) -> Option<Vec<SA>> {
        self.draw();
        None
        //Some(vec![SA::Draw])
    }
    fn draw(&self) {
        // Haut-bas
        for x in 0..=self.size.0 {
            vga_write!(x + self.pos.0, self.pos.1, "$3B\u{C4}");
            vga_write!(x + self.pos.0, self.pos.1 + self.size.1, "$3B\u{C4}",);
        }
        // gauche-droite
        for y in 1..self.size.1 {
            vga_write!(
                self.pos.0,
                self.pos.1 + y,
                "$3B\u{B3}{:width$}$3B\u{B3}",
                ' ',
                width = self.size.0 - 1
            );
        }
        // name & escape
        vga_write!(self.pos.0, self.pos.1, "$3B\u{DA}$3F {} ", self.name);
        vga_write!(
            self.pos.0 + self.size.0 - 5,
            self.pos.1,
            "$3B[$34Esc$3B]\u{BF}"
        );
        // right corners
        vga_write!(self.pos.0, self.pos.1 + self.size.1, "$3B\u{BF}");
        vga_write!(
            self.pos.0 + self.size.0,
            self.pos.1 + self.size.1,
            "$3B\u{D9}",
        );
        // Content
        let mut x = 0;
        for line in &self.content {
            vga_write!(
                self.pos.0 + 1,
                self.pos.1 + x + 1,
                "$3F{:^width$}",
                line,
                width = self.size.0 - 1
            );
            x += 1
        }
    }
    fn on_time(&mut self, _time: u8) -> Option<Vec<SA>> {
        None
    }
    fn on_key(&mut self, key_event: KeyEvent, _as_char: Option<char>) -> Option<Vec<SA>> {
        // Detect key
        if key_event.state == KeyState::Down {
            match key_event.code {
                KeyCode::Spacebar | KeyCode::Enter | KeyCode::Escape => Some(self.next.clone()),
                _ => None,
            }
        } else {
            None
        }
    }
}
