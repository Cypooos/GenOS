use super::{screens::Screen, Screenable};
use crate::vga_writer;
use alloc::{string::String, vec::Vec};

use pc_keyboard::{KeyEvent, KeyState};

pub struct GifVeiwer {
    pub frames: Vec<String>,
    pub speed: usize,
    frame_nb: usize,
    pub next: Screen,
}

impl GifVeiwer {
    pub fn new(frames: Vec<String>, speed: usize, next: Screen) -> Self {
        Self {
            frames,
            speed,
            frame_nb: 0,
            next,
        }
    }
}

impl Screenable for GifVeiwer {
    fn init(&mut self) {
        vga_writer::WRITER.lock().clear();
        vga_write!(0, 0, "{}", self.frames[0]);
    }
    fn on_time(&mut self, _time: u8) -> Option<Screen> {
        self.frame_nb += 1;
        if self.frame_nb / self.speed >= self.frames.len() {
            self.frame_nb = 0;
        }
        vga_writer::WRITER.lock().clear();
        vga_write!(0, 0, "{}", self.frames[self.frame_nb / self.speed]);
        None
    }
    fn on_key(&mut self, key_event: KeyEvent, _as_char: Option<char>) -> Option<Screen> {
        // Detect key
        if key_event.state == KeyState::Down {
            return Some(self.next);
        }
        None
    }
}
