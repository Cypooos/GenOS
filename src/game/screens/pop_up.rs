use super::{screens::Screen, Screenable, SA};
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
    pub color: String,
}

impl PopUp {
    pub fn new(
        name: &str,
        size: (usize, usize),
        pos: (usize, usize),
        content: Vec<String>,
        next: Vec<SA>,
        color: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            content,
            size,
            pos,
            next,
            color: color.to_string(),
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
        for x in 0..=self.size.0 {
            vga_write!(x + self.pos.0, self.pos.1, "{}\u{CD}", self.color);
            vga_write!(
                x + self.pos.0,
                self.pos.1 + self.size.1,
                "{}\u{C4}",
                self.color,
            );
        }
        for y in 0..self.size.1 {
            vga_write!(
                self.pos.0,
                self.pos.1 + y,
                "{}\u{B3}{:width$}",
                self.color,
                ' ',
                width = self.size.0
            );
            vga_write!(
                self.pos.0 + self.size.0,
                self.pos.1 + y,
                "{}\u{B3}",
                self.color,
            );
        }
        vga_write!(
            self.pos.0,
            self.pos.1,
            "{}\u{D5}\u{CD} {} ",
            self.color,
            self.name
        );
        vga_write!(self.pos.0 + self.size.0, self.pos.1, "{}\u{B8}", self.color);
        vga_write!(self.pos.0, self.pos.1 + self.size.1, "{}\u{C0}", self.color);
        vga_write!(
            self.pos.0 + self.size.0,
            self.pos.1 + self.size.1,
            "{}\u{D9}",
            self.color
        );

        let mut x = 0;
        for line in &self.content {
            vga_write!(
                self.pos.0 + 1,
                self.pos.1 + x + 1,
                "{}{:^width$}",
                self.color,
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
