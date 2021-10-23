use super::{screens::Screen, Screenable, SA};
use crate::vga_writer;
use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};

use pc_keyboard::{KeyEvent, KeyState};

const TEXT_SPEED: usize = 2;
const TEXT_LENGTH: usize = 30;
const CH_STR_X: usize = 1;
const CH_STR_Y: usize = 14;
const TXT_STR_Y: usize = CH_STR_Y + 2;
const TXT_STR_X: usize = CH_STR_X + 22;

#[derive(Debug)]
pub enum ChokeFace {
    Normal,
    Sad,
    Happy,
}

pub struct RpgDial {
    pub face: ChokeFace,
    pub name: String,
    pub text: [String; 5], // 5 lines of text
    pub next: Vec<SA>,
    time: usize,
    size: usize,
}

impl RpgDial {
    pub fn new(face: ChokeFace, name: &str, texts: [&str; 5], next: Vec<SA>) -> Self {
        Self {
            face,
            name: name.to_owned(),
            text: texts.map(|x| x.to_owned()),
            next,
            time: 0,
            size: *texts.map(|x| x.len()).iter().max().unwrap(),
        }
    }
}

impl Screenable for RpgDial {
    fn draw(&mut self) -> Option<Vec<SA>> {
        for row in CH_STR_Y..25 {
            vga_writer::WRITER.lock().clear_row(row);
        }

        for x in 1..79 {
            vga_write!(x, CH_STR_Y - 1, "\u{CD}");
            vga_write!(x, 24, "\u{CD}");
        }

        vga_write!(0, CH_STR_Y - 1, "\u{C9}");
        vga_write!(0, 24, "\u{C8}");

        vga_write!(79, CH_STR_Y - 1, "\u{BB}");
        vga_write!(79, 24, "\u{BC}");

        vga_write!(TXT_STR_X - 4, CH_STR_Y - 1, "\u{CB}");
        vga_write!(TXT_STR_X - 4, 24, "\u{CA}");

        for x in 0..10 {
            vga_write!(TXT_STR_X - 4, CH_STR_Y + x, "\u{BA}");
            vga_write!(0, CH_STR_Y + x, "\u{BA}");
            vga_write!(79, CH_STR_Y + x, "\u{BA}");
        }

        vga_write!(TXT_STR_X - 3, CH_STR_Y, "{}", self.name);

        match self.face {
            ChokeFace::Normal => {
                vga_write!(CH_STR_X, CH_STR_Y + 0, r#"     .######.    "#);
                vga_write!(CH_STR_X, CH_STR_Y + 1, r#" .#############. "#);
                vga_write!(CH_STR_X, CH_STR_Y + 2, r#"#################"#);
                vga_write!(CH_STR_X, CH_STR_Y + 3, r#"##  . ##### .  ##"#);
                vga_write!(CH_STR_X, CH_STR_Y + 4, r#"##___/#####\___##"#);
                vga_write!(CH_STR_X, CH_STR_Y + 5, r#"'####### #######'"#);
                vga_write!(CH_STR_X, CH_STR_Y + 6, r#"   ####___####   "#);
                vga_write!(CH_STR_X, CH_STR_Y + 7, r#"   '#########'   "#);
                vga_write!(CH_STR_X, CH_STR_Y + 8, r#"    ## ||| ##    "#);
                vga_write!(CH_STR_X, CH_STR_Y + 9, r#"    '#######'    "#);
            }
            ChokeFace::Sad => {
                vga_write!(CH_STR_X, CH_STR_Y + 0, r#"     .######.    "#);
                vga_write!(CH_STR_X, CH_STR_Y + 1, r#" .#############. "#);
                vga_write!(CH_STR_X, CH_STR_Y + 2, r#"#################"#);
                vga_write!(CH_STR_X, CH_STR_Y + 3, r#"####/.#####.\####"#);
                vga_write!(CH_STR_X, CH_STR_Y + 4, r#"##/__/#####\__\##"#);
                vga_write!(CH_STR_X, CH_STR_Y + 5, r#"'####### #######'"#);
                vga_write!(CH_STR_X, CH_STR_Y + 6, r#"   ####___####   "#);
                vga_write!(CH_STR_X, CH_STR_Y + 7, r#"   '#########'   "#);
                vga_write!(CH_STR_X, CH_STR_Y + 8, r#"    ## _=_ ##    "#);
                vga_write!(CH_STR_X, CH_STR_Y + 9, r#"    '#######'    "#);
            }
            ChokeFace::Happy => {
                vga_write!(CH_STR_X, CH_STR_Y + 0, r#"     .######.    "#);
                vga_write!(CH_STR_X, CH_STR_Y + 1, r#" .#############. "#);
                vga_write!(CH_STR_X, CH_STR_Y + 2, r#"#################"#);
                vga_write!(CH_STR_X, CH_STR_Y + 3, r#"## __ ##### __ ##"#);
                vga_write!(CH_STR_X, CH_STR_Y + 4, r#"##/  \#####/  \##"#);
                vga_write!(CH_STR_X, CH_STR_Y + 5, r#"'####### #######'"#);
                vga_write!(CH_STR_X, CH_STR_Y + 6, r#"   ####___####   "#);
                vga_write!(CH_STR_X, CH_STR_Y + 7, r#"   '#########'   "#);
                vga_write!(CH_STR_X, CH_STR_Y + 8, r#"    ## \_/ ##    "#);
                vga_write!(CH_STR_X, CH_STR_Y + 9, r#"    '#######'    "#);
            }
        }
        vga_write!(TXT_STR_X, TXT_STR_Y + 0, "{:.1$}", self.text[0], self.time);
        vga_write!(TXT_STR_X, TXT_STR_Y + 1, "{:.1$}", self.text[1], self.time);
        vga_write!(TXT_STR_X, TXT_STR_Y + 2, "{:.1$}", self.text[2], self.time);
        vga_write!(TXT_STR_X, TXT_STR_Y + 3, "{:.1$}", self.text[3], self.time);
        vga_write!(TXT_STR_X, TXT_STR_Y + 4, "{:.1$}", self.text[4], self.time);
        None
    }
    fn on_time(&mut self, _time: u8) -> Option<Vec<SA>> {
        if self.time < self.size {
            self.time += TEXT_SPEED
        };
        None
    }
    fn on_key(&mut self, key_event: KeyEvent, _as_char: Option<char>) -> Option<Vec<SA>> {
        //vga_print!("desktop:{},", scancode);
        if key_event.state == KeyState::Down {
            if self.time < self.size {
                self.time = self.size;
            } else {
                self.time = 0;
                return Some(self.next.clone());
            }
        };
        None
    }
}
