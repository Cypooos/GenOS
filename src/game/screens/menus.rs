use super::{screens::Screen, Screenable};
use crate::vga_writer;
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
};

use core::fmt;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, KeyState, Keyboard, ScancodeSet1};
use spin::Mutex;

#[derive(Debug)]
pub enum OneScreenMenu {
    MainMenu,
    CreditMenu,
    TestMenu,
}

impl Screenable for OneScreenMenu {
    fn init(&mut self) {
        match self {
            OneScreenMenu::MainMenu => {
                vga_writer::WRITER.lock().clear();
                vga_write!(
                    0,
                    7,
                    "                   $0B.o88b.  db   db   .d88b.   db   dD  d88888b  \
                    \n                  $0Bd8P  Y8  88   88  .8P  Y8.  88 ,8P'  88'     \
                    \n                  $038P       88ooo88  88    88  88,8P    88ooooo \
                    \n                  $038b       88~~~88  88    88  88`8b    88~~~~~ \
                    \n                  $02Y8b  d8  88   88  `8b  d8'  88 `88.  88.     \
                    \n                  $0A `Y88P'  YP   YP   `Y88P'   YP   YD  Y88888P \
                    \n\n\n\n                            $0ESince 1985. \
                    \n                                    $0DPress Space to $D0start$0D."
                );
            }
            OneScreenMenu::CreditMenu => {
                vga_write!(17, 5, "$3E{: ^46}", "CHOKE: credits");
                vga_write!(17, 6, "$3F{: ^46}", "A OS-game by <Discursif/>");
                vga_write!(17, 7, "$3F{: ^46}", " ");
                vga_write!(17, 8, "$3F{: ^46}", " ");
                vga_write!(17, 9, "$3F{: ^46}", "Everything made by :");
                vga_write!(17, 10, "$3F{: ^49}", "@$3ECypooos");
                vga_write!(17, 11, "$3F{: ^46}", "");
                vga_write!(17, 12, "$3F{: ^46}", "");
                vga_write!(
                    17,
                    13,
                    "$3F{: ^46}",
                    "OS made in rust based on PHIL'S tutorial"
                );
            }
            TestMenu => {
                vga_writer::WRITER.lock().clear();
                vga_write!(
                    0,
                    7,
                    "                   $0B d888b    .d8b.    88b  d88.  d88888b  \
                    \n                  $0B 88' Y8b  d8' `8b  88'YbdP`88  88'      \
                    \n                  $03 88       88ooo88  88  88  88  88ooooo  \
                    \n                  $03 88  ooo  88~~~88  88  88  88  88~~~~~  \
                    \n                  $02 88. ~8~  88   88  88  88  88  88.      \
                    \n                  $0A  Y888P   YP   YP  YP  YP  YP  Y88888P  \
                    \n\n\n\n                              $0EYou are in !"
                );
            }
        };
    }
    fn draw(&mut self) -> Option<Screen> {
        None
    }
    fn on_key(&mut self, scancode: u8) -> Option<Screen> {
        lazy_static! {
            static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> = Mutex::new(
                Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore)
            );
        };

        let mut keyboard = KEYBOARD.lock();

        //vga_print!("desktop:{},", scancode);
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            match key_event.code {
                KeyCode::Spacebar => {
                    if key_event.state == KeyState::Down {
                        match self {
                            OneScreenMenu::MainMenu => return Some(Screen::TestMenu),
                            _ => (),
                        };
                    };
                }
                KeyCode::Escape => {
                    if key_event.state == KeyState::Down {
                        match self {
                            OneScreenMenu::MainMenu => return Some(Screen::CreditMenu),
                            OneScreenMenu::CreditMenu => return Some(Screen::MainMenu),
                            _ => (),
                        };
                    };
                }
                KeyCode::Tab => {
                    if key_event.state == KeyState::Down {
                        match self {
                            OneScreenMenu::MainMenu => return Some(Screen::DebugPasswordMenu),
                            _ => (),
                        };
                    };
                }
                _ => {}
            }
        };
        None
    }
}

pub struct PasswordMenu {
    pub code: String,
    act_code: String,
    pub if_ok: Screen,
    pub if_nok: Screen,
}

impl PasswordMenu {
    pub fn new(code: &str, if_ok: Screen, if_nok: Screen) -> Self {
        Self {
            code: code.to_string(),
            act_code: String::new(),
            if_ok,
            if_nok,
        }
    }
}

impl fmt::Debug for PasswordMenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PasswordMenu")
            .field("code", &self.code)
            .field("->", &self.if_ok)
            .finish()
    }
}

impl Screenable for PasswordMenu {
    fn init(&mut self) {
        let yee = format!("$3E{:_<1$}", self.act_code, self.code.len());

        vga_write!(17, 5, "$4F{: ^46}", "[Password required]");
        vga_write!(17, 6, "$3F{: ^46}", "");
        vga_write!(17, 7, "$3F{: ^46}", "Please enter the password:");
        vga_write!(17, 8, "$3F{: ^46}", "");
        vga_write!(17, 9, "$3F{: ^49}", yee);
        vga_write!(17, 10, "$3F{: ^46}", "");
    }
    fn draw(&mut self) -> Option<Screen> {
        None
    }
    fn on_key(&mut self, scancode: u8) -> Option<Screen> {
        lazy_static! {
            static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> = Mutex::new(
                Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore)
            );
        };

        let mut keyboard = KEYBOARD.lock();

        //vga_print!("desktop:{},", scancode);
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if key_event.code == KeyCode::Escape {
                if key_event.state == KeyState::Down {
                    return Some(self.if_nok);
                };
                return None;
            }

            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        self.act_code.push(character);
                    }
                    DecodedKey::RawKey(key) => {}
                }
            };
        }

        if self.act_code.len() == self.code.len() {
            let returning = Some(if self.act_code == self.code {
                self.if_ok
            } else {
                self.if_nok
            });
            self.act_code = String::new();
            return returning;
        }

        let yee = format!("$3E{:_<1$}", self.act_code, self.code.len());
        vga_write!(17, 9, "$3F{: ^49}", yee);
        None
    }
}
