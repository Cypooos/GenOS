use crate::TUI::screen::Screens;

use lazy_static::lazy_static;

use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use spin::Mutex;

use core::fmt;

lazy_static! {
    pub static ref DESKTOP: Mutex<DesktopTUI> = Mutex::new(DesktopTUI {
        mouse_pos: (5, 5),
        active_screen: Screens::DefaultMenu(),
    });
}

pub struct DesktopTUI {
    mouse_pos: (u16, u16),
    active_screen: Screens,
}

impl DesktopTUI {
    pub fn main(&mut self) -> ! {}

    pub fn print(&self, s: fmt::Arguments) {
        crate::vga_writer::_print(s);
    }

    pub fn on_key(&mut self, scancode: u8) {
        lazy_static! {
            static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
                Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
            );
        };

        let mut keyboard = KEYBOARD.lock();
        //print!("desktop:{},",scancode);

        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            match key_event.code {
                KeyCode::Escape => {
                    self.active_screen = match self.active_screen {
                        Screens::TestMenu() => Screens::DefaultMenu(),
                        Screens::DefaultMenu() => Screens::TestMenu(),
                    };
                }
                KeyCode::WindowsLeft => {
                    println!("Windows menu");
                }
                _ => {
                    print!("{:?}", key_event.code);
                    if let Some(key) = keyboard.process_keyevent(key_event) {
                        match key {
                            DecodedKey::Unicode(character) => print!("{}", character),
                            DecodedKey::RawKey(key) => print!("{:?}", key),
                        }
                    };
                }
            }
        }
    }
}
