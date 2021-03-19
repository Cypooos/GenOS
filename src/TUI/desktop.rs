use crate::TUI::screen::Screens;

use lazy_static::lazy_static;

use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, KeyState, Keyboard, ScancodeSet1};
use spin::Mutex;

use core::{fmt, intrinsics::write_bytes};

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use core::fmt::Write;

lazy_static! {
    pub static ref DESKTOP: Mutex<DesktopTUI> = Mutex::new(DesktopTUI {
        mouse_pos: (5, 5),
        active_screen: Screens::DebugScreen,
        logging: Vec::new(),
        held: Vec::new()
    });
}

pub struct DesktopTUI {
    mouse_pos: (usize, usize),
    active_screen: Screens,
    logging: Vec<String>,
    held: Vec<KeyCode>,
}

impl fmt::Write for DesktopTUI {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.logging.push(s.to_string());
        Ok(())
    }
}

impl DesktopTUI {
    pub fn draw(&mut self) {
        vga_write!(
            0,
            0,
            "$3F                                                                    <Discursif/>"
        );
        vga_write!(0, 0, "$3FGenOS vb1.0.0 | $3e{:?}", self.active_screen);
        if self.held.contains(&KeyCode::Tab) {
            vga_write!(3, 3, "$3FTAB DETECTED");
        } else {
            vga_write!(3, 3, "$3FNOT DETECTED");
        }
    }

    pub fn print(&mut self, args: fmt::Arguments) {
        self.write_fmt(args);
    }

    pub fn on_key(&mut self, scancode: u8) {
        lazy_static! {
            static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> = Mutex::new(
                Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore)
            );
        };

        let mut keyboard = KEYBOARD.lock();
        //print!("desktop:{},",scancode);

        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if key_event.code == KeyCode::Escape {
                if key_event.state == KeyState::Down {
                    self.active_screen = Screens::DebugScreen;
                } else {
                    self.active_screen = Screens::LoggingScreen;
                };
                return;
            }

            if key_event.state == KeyState::Down && !self.held.contains(&key_event.code) {
                self.held.push(key_event.code);
            } else if key_event.state == KeyState::Up {
                let e = self.held.iter().position(|&r| r == key_event.code);
                match e {
                    Some(e) => {
                        self.held.remove(e);
                    }
                    None => {}
                }
            }

            // print!("{:?}", key_event.code);
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            };
        }
    }
}
