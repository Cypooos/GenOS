use super::{screens::Screen, Screenable};
use crate::{vga_writer, vga_writer::WRITER};
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};

use lazy_static::lazy_static;
use pc_keyboard::{KeyCode, KeyEvent, KeyState};
use spin::Mutex;

pub struct Element {
    pub next: Screen,
}

impl Element {
    pub fn new(next: Screen) -> Self {
        Self { next }
    }
}

impl Screenable for Element {
    fn init(&mut self) {
        vga_writer::WRITER.lock().clear();
        vga_write!(0, 0, "")
    }
    fn on_time(&mut self, time: u8) -> Option<Screen> {}
    fn on_key(&mut self, key_event: KeyEvent, as_char: Option<char>) -> Option<Screen> {
        // Detect key
        if key_event.state == KeyState::Down {}
        if key_event.code == KeyCode::A {}
    }
}
