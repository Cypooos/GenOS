use crate::game::screen::Screens;

use lazy_static::lazy_static;

use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, KeyState, Keyboard, ScancodeSet1};
use spin::Mutex;

use core::{fmt, usize};

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
        active_screen: Screens::MainMenu,
        time: 0,
        last_key: None
    });
}

pub enum Effects {
    SimpleGlitch(usize),
}

pub static mut NEED_REFRESH: bool = false;

pub struct DesktopTUI {
    mouse_pos: (usize, usize),
    active_screen: Screens,
    time: usize,
    last_key: Option<u8>,
}

impl fmt::Write for DesktopTUI {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        //qemu_print!(" WRITING A STRING");
        // TODO : DEBUG AND LOG SCREEN
        //vga_print!("{}", s);
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
    pub fn reload(&mut self) {
        debug!("reloading");
        // self.key_reload();
        // self.draw();
        // unsafe { NEED_REFRESH = false };
        return;
    }

    pub fn time_interrupt(&mut self) {
        debug!("time int");
        //self.time = self.time.checked_add(1).unwrap_or(0);
        // unsafe { NEED_REFRESH = true };
        self.draw()
    }

    pub fn draw(&mut self) {
        debug!("draw");
        // Pas de print! car ça apelle DESKTOPTUI.draw
        vga_write!(
            0,
            0,
            "$3F                                                                    <Discursif/>"
        );
        vga_write!(0, 0, "$3FGenOS vb1.0.0 | $3e{:?}", self.active_screen);
        // vga_write!(0, 1, "$3f{}", self.count);

        match self.active_screen {
            Screens::MainMenu => {
                vga_write!(
                    0,
                    7,
                    "                  $0B.o88b.  db   db   .d88b.   db   dD  d88888b  \
                    \n                 $0Bd8P  Y8  88   88  .8P  Y8.  88 ,8P'  88'     \
                    \n                 $038P       88ooo88  88    88  88,8P    88ooooo \
                    \n                 $038b       88~~~88  88    88  88`8b    88~~~~~ \
                    \n                 $02Y8b  d8  88   88  `8b  d8'  88 `88.  88.     \
                    \n                 $0A `Y88P'  YP   YP   `Y88P'   YP   YD  Y88888P \
                    \n\n\n\n                           $0ESince 1985. \
                    \n                                   $0DPress Space to $D0start$0D."
                );
            }
            Screens::Menu => {}
        }

        // if self.held.contains(&KeyCode::Tab) {
        //     vga_write!(3, 3, "$3FTAB DETECTED");
        // } else {
        //     vga_write!(3, 3, "$3FNOT DETECTED");
        // }
    }

    // Le on_key ne retourne pas
    pub fn on_key(&mut self, scancode: u8) {
        debug!("key reloading");
        lazy_static! {
            static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> = Mutex::new(
                Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore)
            );
        };

        self.last_key = None;
        let mut keyboard = KEYBOARD.lock();

        //vga_print!("desktop:{},", scancode);
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if key_event.code == KeyCode::Escape {
                if key_event.state == KeyState::Down {
                    self.active_screen = Screens::Menu;
                } else {
                    self.active_screen = Screens::MainMenu;
                };
                return;
            }
            // La meilleure solution mdr
            // "held" n'est pas vraiment important dans tout les cas
            // l'important c'est plutot juste certaine touche: Controle, Alt, Escape etc...
            print!("{:?}", key_event.code);
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => vga_write!(4, 4, "{}", character),
                    DecodedKey::RawKey(key) => vga_write!(4, 4, "{:?}", key),
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
