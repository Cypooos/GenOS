use crate::game::screens::{screens::Screen, Screenable, SA};
use crate::io::vga_writer::{Color, WRITER};

use alloc::{string::String, vec, vec::Vec};

use core::fmt;
use pc_keyboard::KeyEvent;

pub struct FileExplorerElement {
    pub name: String,
    pub show: String,
    pub color: Color,
    pub is_folder: bool,
    pub protection: Option<String>,
    pub execute: Option<Screen>,
}

pub struct FileExplorer {
    _containing_folder: Vec<FileExplorerElement>,
    _parent: Option<Screen>,
}

impl Screenable for FileExplorer {
    fn init(&mut self) -> Option<Vec<SA>> {
        Some(vec![SA::Draw])
    }
    fn on_time(&mut self, _time: u8) -> Option<Vec<SA>> {
        None
    }
    fn on_key(&mut self, _key_event: KeyEvent, _as_char: Option<char>) -> Option<Vec<SA>> {
        None
    }
}

impl fmt::Debug for FileExplorer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileExplorer").finish()
    }
}
