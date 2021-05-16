use super::{screens::Screen, Screenable};
use crate::vga_writer;
use alloc::string::String;

use pc_keyboard::{KeyEvent, KeyState};

const TEXT_SPEED: usize = 2;
const TEXT_LENGTH: usize = 30;
const CHOKE_START_X: usize = 20;
const TEXT_START_X: usize = CHOKE_START_X + 15;

#[derive(Debug)]
pub enum ChokeFace {
    Normal,
    Happy,
    OwO,
}

pub struct SimpleDialogue {
    pub face: ChokeFace,
    pub text: [String; 5], // 5 lines of text
    pub next: Screen,
    time: usize,
}

impl SimpleDialogue {
    pub fn new(face: ChokeFace, texts: [String; 5], next: Screen) -> Self {
        Self {
            face,
            text: texts,
            next,
            time: 0,
        }
    }
}

impl Screenable for SimpleDialogue {
    fn init(&mut self) {
        vga_writer::WRITER.lock().clear();
        match self.face {
            ChokeFace::Normal => {
                vga_write!(CHOKE_START_X, 03, r#"   _____         "#);
                vga_write!(CHOKE_START_X, 04, r#"  /     \        "#);
                vga_write!(CHOKE_START_X, 05, r#" | () () |       "#);
                vga_write!(CHOKE_START_X, 06, r#" (   ^   )       "#);
                vga_write!(CHOKE_START_X, 07, r#"  \|,,,|X\       "#);
                vga_write!(CHOKE_START_X, 08, r#"   |"""| \\      "#);
                vga_write!(CHOKE_START_X, 09, r#"          \\     "#);
                vga_write!(CHOKE_START_X, 10, r#"       /[#####]\ "#);
                vga_write!(CHOKE_START_X, 11, r#"      //[#####]\\"#);
                vga_write!(CHOKE_START_X, 12, r#"O====[] [#####]||"#);
                vga_write!(CHOKE_START_X, 13, r#"        [#####]||"#);
                vga_write!(CHOKE_START_X, 14, r#"               ()"#);
            }
            ChokeFace::Happy => {
                vga_write!(CHOKE_START_X, 03, r#"   _____         "#);
                vga_write!(CHOKE_START_X, 04, r#"  /     \        "#);
                vga_write!(CHOKE_START_X, 05, r#" | /\ /\ |       "#);
                vga_write!(CHOKE_START_X, 06, r#" (   ^   )       "#);
                vga_write!(CHOKE_START_X, 07, r#"  \|\,/|X\       "#);
                vga_write!(CHOKE_START_X, 08, r#"   |---| \\      "#);
                vga_write!(CHOKE_START_X, 09, r#"          \\     "#);
                vga_write!(CHOKE_START_X, 10, r#"       /[#####]\ "#);
                vga_write!(CHOKE_START_X, 11, r#"      //[#####]\\"#);
                vga_write!(CHOKE_START_X, 12, r#"O====[] [#####]||"#);
                vga_write!(CHOKE_START_X, 13, r#"        [#####]||"#);
                vga_write!(CHOKE_START_X, 14, r#"               ()"#);
            }
            ChokeFace::OwO => {
                vga_write!(CHOKE_START_X, 03, r#"   _____         "#);
                vga_write!(CHOKE_START_X, 04, r#"  /     \        "#);
                vga_write!(CHOKE_START_X, 05, r#" |(O) (O)|       "#);
                vga_write!(CHOKE_START_X, 06, r#" (   ^   )       "#);
                vga_write!(CHOKE_START_X, 07, r#"  \|---|X\       "#);
                vga_write!(CHOKE_START_X, 08, r#"   |---| \\      "#);
                vga_write!(CHOKE_START_X, 09, r#"          \\     "#);
                vga_write!(CHOKE_START_X, 10, r#"       /[#####]\ "#);
                vga_write!(CHOKE_START_X, 11, r#"      //[#####]\\"#);
                vga_write!(CHOKE_START_X, 12, r#"O====[] [#####]||"#);
                vga_write!(CHOKE_START_X, 13, r#"        [#####]||"#);
                vga_write!(CHOKE_START_X, 14, r#"               ()"#);
            }
        }
    }
    fn on_time(&mut self, _time: u8) -> Option<Screen> {
        if self.time < TEXT_LENGTH {
            self.time += TEXT_SPEED
        };
        vga_write!(TEXT_START_X, 4, "{:.1$}", self.text[0], self.time);
        vga_write!(TEXT_START_X, 5, "{:.1$}", self.text[1], self.time);
        vga_write!(TEXT_START_X, 6, "{:.1$}", self.text[2], self.time);
        vga_write!(TEXT_START_X, 7, "{:.1$}", self.text[3], self.time);
        vga_write!(TEXT_START_X, 8, "{:.1$}", self.text[4], self.time);
        None
    }
    fn on_key(&mut self, key_event: KeyEvent, _as_char: Option<char>) -> Option<Screen> {
        //vga_print!("desktop:{},", scancode);
        if key_event.state == KeyState::Down {
            if self.time < TEXT_LENGTH {
                self.time = TEXT_LENGTH;
            } else {
                self.time = 0;
                return Some(self.next);
            }
        };
        None
    }
}
