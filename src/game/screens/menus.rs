use super::{screens::Screen, Screenable, SA};
use crate::io::vga_writer;
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use core::{fmt, hash::Hasher};
use pc_keyboard::{KeyCode, KeyEvent, KeyState};

#[derive(Debug)]
pub enum OneScreenMenu {
    MainMenu,
    //CreditMenu,
    _404,
}

impl Screenable for OneScreenMenu {
    fn init(&mut self) -> Option<Vec<SA>> {
        Some(vec![SA::Draw])
    }
    fn draw(&self) {
        vga_writer::WRITER.lock().clear();
        match self {
            OneScreenMenu::MainMenu => {
                vga_write!(
                    0,
                    6,
                    "                   $0B.o88b.  db   db   .d88b.   db   dD  d88888b  \
                    \n                  $0Bd8P  Y8  88   88  .8P  Y8.  88 ,8P'  88'     \
                    \n                  $038P       88ooo88  88    88  88,8P    88ooooo \
                    \n                  $038b       88~~~88  88    88  88`8b    88~~~~~ \
                    \n                  $02Y8b  d8  88   88  `8b  d8'  88 `88.  88.     \
                    \n                  $0A `Y88P'  YP   YP   `Y88P'   YP   YD  Y88888P \
                    \n\n                                  $0ESince 1985. \
                    \n\n\n                  $0A- Press Space to $A0start$0A\
                    \n                  $0C- Press Tab  for $C0files$0C\
                    \n                  $0A- Press Esc  for $A0credits$0A"
                );
            }
            // OneScreenMenu::CreditMenu => {
            //     vga_write!(17, 5, "$8F{: ^46}", "CHOKE: credits");
            //     vga_write!(17, 6, "$3F{: ^49}", "A OS-game by $3E<Discursif/>");
            //     vga_write!(17, 7, "$3F{: ^46}", " ");
            //     vga_write!(17, 8, "$3F{: ^46}", " ");
            //     vga_write!(17, 9, "$3F{: ^46}", "Everything made by :");
            //     vga_write!(17, 10, "$3F{: ^49}", "@$3ECypooos");
            //     vga_write!(17, 11, "$3F{: ^46}", "");
            //     vga_write!(17, 12, "$3F{: ^46}", "");
            //     vga_write!(
            //         17,
            //         13,
            //         "$3F{: ^46}",
            //         "OS made in rust based on PHIL'S tutorial"
            //     );
            // }
            OneScreenMenu::_404 => {
                vga_write!(
                    0,
                    7,
                    "                   $0B d888b    .d8b.    88b  d88.  d88888b  \
                    \n                  $0B 88' Y8b  d8' `8b  88'YbdP`88  88'      \
                    \n                  $03 88       88ooo88  88  88  88  88ooooo  \
                    \n                  $03 88  ooo  88~~~88  88  88  88  88~~~~~  \
                    \n                  $02 88. ~8~  88   88  88  88  88  88.      \
                    \n                  $0A  Y888P   YP   YP  YP  YP  YP  Y88888P  \
                    \n\n\n\n                              $0EThis is a 404"
                );
            }
        };
    }
    fn on_time(&mut self, _time: u8) -> Option<Vec<SA>> {
        None
    }
    fn on_key(&mut self, key_event: KeyEvent, _as_char: Option<char>) -> Option<Vec<SA>> {
        match key_event.code {
            KeyCode::Spacebar => {
                if key_event.state == KeyState::Down {
                    match self {
                        OneScreenMenu::MainMenu => return Some(vec![SA::Change(Screen::Intro(0))]),
                        _ => (),
                    };
                };
            }
            KeyCode::Escape => {
                if key_event.state == KeyState::Down {
                    match self {
                        OneScreenMenu::MainMenu => {
                            return Some(vec![SA::Change(Screen::CreditMenu)])
                        }
                        //OneScreenMenu::CreditMenu => {
                        //    return Some(vec![SA::Change(Screen::MainMenu)])
                        //}
                        _ => (),
                    };
                };
            }
            KeyCode::Tab => {
                if key_event.state == KeyState::Down {
                    match self {
                        OneScreenMenu::MainMenu => {
                            return Some(vec![SA::Load(Screen::FilesPassword)])
                        }
                        _ => (),
                    };
                };
            }
            _ => {}
        };
        None
    }
}

pub struct PasswordMenu {
    pub code: String,
    act_code: String,
    counter: usize,
    pub if_ok: Vec<SA>,
    pub if_nok: Vec<SA>,
}

impl PasswordMenu {
    pub fn new(code: &str, if_ok: Vec<SA>, if_nok: Vec<SA>) -> Self {
        Self {
            code: code.to_string(),
            act_code: String::new(),
            counter: PASSWORD_MENU_TIME,
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

const PASSWORD_MENU_TIME: usize = 20;

impl Screenable for PasswordMenu {
    fn init(&mut self) -> Option<Vec<SA>> {
        Some(vec![SA::Draw])
    }
    fn draw(&self) {
        let yee = format!("$3E{:_<1$}", self.act_code, self.code.len());

        vga_write!(17, 5, "$4F{: ^46}", "[Password required]");
        vga_write!(17, 6, "$3F{: ^46}", "");
        vga_write!(17, 7, "$3F{: ^46}", "Please enter the password:");
        vga_write!(17, 8, "$3F{: ^46}", "");
        vga_write!(17, 9, "$3F{: ^49}", yee);
        vga_write!(17, 10, "$3F{: ^46}", "");
    }
    fn on_time(&mut self, _time: u8) -> Option<Vec<SA>> {
        if self.act_code.len() == self.code.len() {
            self.counter -= 1;
            if self.counter == 0 {
                let returning = Some(if self.act_code == self.code {
                    self.if_ok.clone()
                } else {
                    self.if_nok.clone()
                });
                self.act_code = String::new();
                self.counter = PASSWORD_MENU_TIME;
                return returning;
            };
        }
        None
    }
    fn on_key(&mut self, key_event: KeyEvent, as_char: Option<char>) -> Option<Vec<SA>> {
        if self.act_code.len() == self.code.len() {
            return None;
        };

        if key_event.state == KeyState::Down {
            match key_event.code {
                KeyCode::Escape | KeyCode::Enter => {
                    self.act_code = "-".repeat(self.code.len());
                    vga_write!(27, 7, "$4F{: ^26}", "INVALID PASSWORD");
                    return None;
                }
                KeyCode::Backspace | KeyCode::Delete => {
                    self.act_code.pop();
                    let yee = format!("$3E{:_<1$}", self.act_code, self.code.len());
                    vga_write!(17, 9, "$3F{: ^49}", yee);
                    return None;
                }
                _ => {}
            }
        };
        if let Some(character) = as_char {
            self.act_code.push(character);
        };

        if self.act_code.len() == self.code.len() {
            if self.act_code == self.code {
                vga_write!(27, 7, "$AF{: ^26}", "PASSWORD ACCEPTED");
            } else {
                vga_write!(27, 7, "$4F{: ^26}", "INVALID PASSWORD");
            }
        }

        let yee = format!("$3E{:_<1$}", self.act_code, self.code.len());
        vga_write!(17, 9, "$3F{: ^49}", yee);
        None
    }
}
