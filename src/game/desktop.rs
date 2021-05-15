use lazy_static::lazy_static;

use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, KeyState, Keyboard, ScancodeSet1};
use spin::Mutex;

use core::{fmt, usize};

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use volatile::Volatile;

use crate::{game::screens, vga_writer};

lazy_static! {
    static ref DESKTOP_LOGGER: [[vga_writer::ScreenChar; vga_writer::BUFFER_WIDTH]; vga_writer::BUFFER_HEIGHT] =
        [[vga_writer::DEFAULT_SCREENCHAR; vga_writer::BUFFER_WIDTH]; vga_writer::BUFFER_HEIGHT];
}

lazy_static! {
    pub static ref DESKTOP: Mutex<DesktopTUI> = Mutex::new(DesktopTUI {
        mouse_pos: (5, 5),
        active_screen: screens::menus::OneScreenMenu,
        time: 0,
    });
}

pub struct DesktopTUI {
    mouse_pos: (usize, usize),
    active_screen: tempTest,
    time: usize,
}

impl fmt::Write for DesktopTUI {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        vga_print!("{}", s);
        Ok(())
    }
}

impl DesktopTUI {
    pub fn int_time(&mut self) {
        self.time = self.time.checked_add(1).unwrap_or(0);
        self.draw();
    }

    fn draw(&mut self) {
        use x86_64::instructions::interrupts;
        vga_write!(
            0,
            0,
            "$3F                                                                    <Discursif/>"
        );
        vga_write!(0, 0, "$3FGenOS vb1.0.0 | $3e{:?}", self.active_screen);
        vga_write!(0, 24, "$!                ");
        vga_write!(0, 24, "$50time:{:?}", self.time);
        return;
    }

    pub fn int_key(&mut self, scancode: u8) {
        use x86_64::instructions::interrupts;

        lazy_static! {
            static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> = Mutex::new(
                Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore)
            );
        };
        let test = "test".to_string();
        vga_write!(10, 15, "Test String:{}", test);
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        DESKTOP.lock().write_fmt(args).unwrap();
    });
}
