use lazy_static::lazy_static;

use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, KeyState, Keyboard, ScancodeSet1};
use spin::Mutex;

use core::{fmt, usize};

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use volatile::Volatile;

use crate::{
    game::screens::{
        screens::{make_screens, Screen},
        Screenable,
    },
    vga_writer,
};

lazy_static! {
    static ref DESKTOP_LOGGER: [[vga_writer::ScreenChar; vga_writer::BUFFER_WIDTH]; vga_writer::BUFFER_HEIGHT] =
        [[vga_writer::DEFAULT_SCREENCHAR; vga_writer::BUFFER_WIDTH]; vga_writer::BUFFER_HEIGHT];
}

lazy_static! {
    pub static ref DESKTOP: Mutex<DesktopTUI> = Mutex::new(DesktopTUI {
        mouse_pos: (5, 5),
        active_screen: Screen::MainMenu,
        all_screens: make_screens(),
        time: 0,
    });
}

pub struct DesktopTUI {
    mouse_pos: (usize, usize),
    active_screen: Screen,
    time: u8,
    all_screens: Vec<Box<dyn Screenable>>,
}

impl fmt::Write for DesktopTUI {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        vga_writer::WRITER.lock().write_string(s);
        Ok(())
    }
}

impl DesktopTUI {
    pub fn start(&mut self) {
        self.all_screens[self.active_screen as usize].init();
    }

    pub fn int_time(&mut self) {
        self.time = self.time.checked_add(1).unwrap_or(0);
        self.draw();
    }

    fn draw(&mut self) {
        #[cfg(feature = "info-bar")]
        {
            vga_write!(
                0,
                0,
                "$3F                                                                    <Discursif/>"
            );
            vga_write!(
                0,
                0,
                "$3FChoke vb1.0.0 | $3e{:?}$3F | $35{:?}$3F | \x01",
                self.active_screen,
                self.time
            );
        }
        if let Some(x) = self.all_screens[self.active_screen as usize].draw() {
            self.active_screen = x;
            self.all_screens[self.active_screen as usize].init();
        }
        return;
    }

    pub fn int_key(&mut self, scancode: u8) {
        //vga_print!("desktop:{},", scancode);

        if let Some(x) = self.all_screens[self.active_screen as usize].on_key(scancode) {
            self.active_screen = x;
            self.all_screens[self.active_screen as usize].init();
        };
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    // interrupts::without_interrupts(|| {
    //     qemu_print!("ohdqiujhfs");
    //     DESKTOP.lock().write_fmt(args).unwrap();
    // });
}
