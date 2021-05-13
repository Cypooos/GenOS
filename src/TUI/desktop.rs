use crate::tui::screen::Screens;

use lazy_static::lazy_static;

use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, KeyState, Keyboard, ScancodeSet1};
use spin::Mutex;

use core::fmt;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use volatile::Volatile;

use crate::vga_writer;

lazy_static! {
    static ref DESKTOP_LOGGER: [[vga_writer::ScreenChar; vga_writer::BUFFER_WIDTH]; vga_writer::BUFFER_HEIGHT] =
        [[vga_writer::DEFAULT_SCREENCHAR; vga_writer::BUFFER_WIDTH]; vga_writer::BUFFER_HEIGHT];
}

lazy_static! {
    pub static ref DESKTOP: Mutex<DesktopTUI> = Mutex::new(DesktopTUI {
        mouse_pos: (5, 5),
        active_screen: Screens::DrawScreen,
    });
}

pub struct DesktopTUI {
    mouse_pos: (usize, usize),
    active_screen: Screens,
}

impl fmt::Write for DesktopTUI {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        qemu_print!(" WRITING A STRING");
        vga_print!("{}", s);
        /*if (self.logging_line == LOGGING_SIZE) {
            for row in 1..LOGGING_SIZE {
                self.logging[row - 1] = self.logging[LOGGING_SIZE];
            }
        } else {
            self.logging_line += 1;
        }
        self.logging[self.logging_line] = s;
        qemu_print!(" WRITING DONE");
        */
        Ok(())
    }
}

impl DesktopTUI {
    pub fn draw(&mut self) {
        // Pas de print! car ça apelle DESKTOPTUI.draw
        vga_write!(
            0,
            0,
            "$3F                                                                    <Discursif/>"
        );
        vga_write!(0, 0, "$3FGenOS vb1.0.0 | $3e{:?}", self.active_screen);
        // vga_write!(0, 1, "$3f{}", self.count);

        match self.active_screen {
            Screens::DebugScreen => {
                vga_write!(20, 5, "$3F{: ^40}", "Menu Debug");
                vga_write!(20, 6, "$3F{: ^40}", "");
                vga_write!(20, 7, "$3F{: ^40}", "GenOS vb1.0.1");
                vga_write!(20, 8, "$3F{: ^40}", "Build <Unknow>");
                vga_write!(20, 9, "$3F{: ^40}", "");
                vga_write!(20, 10, "$3F{: ^40}", "By Discursif");
            }
            Screens::DrawScreen => {}
        }

        // if self.held.contains(&KeyCode::Tab) {
        //     vga_write!(3, 3, "$3FTAB DETECTED");
        // } else {
        //     vga_write!(3, 3, "$3FNOT DETECTED");
        // }
    }

    // Le on_key ne retourne pas
    pub fn on_key(&mut self, scancode: u8) {
        lazy_static! {
            static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> = Mutex::new(
                Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore)
            );
        };

        let mut keyboard = KEYBOARD.lock();
        // vga_print!("desktop:{},",scancode);

        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if key_event.code == KeyCode::Escape {
                if key_event.state == KeyState::Down {
                    self.active_screen = Screens::DebugScreen;
                } else {
                    self.active_screen = Screens::DrawScreen;
                };
                return;
            }

            // La meilleure solution mdr
            // "held" n'est pas vraiment important dans tout les cas
            // l'important c'est plutot juste certaine touche: Controle, Alt, Escape etc...

            // print!("{:?}", key_event.code);
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => vga_print!("{}", character),
                    DecodedKey::RawKey(key) => vga_print!("{:?}", key),
                }
            };
        }
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    qemu_print!("CALLING _PRINT A STRING");

    interrupts::without_interrupts(|| {
        DESKTOP.lock().write_fmt(args).unwrap();
    });
}
