use super::{screens::Screen, Screenable};
use crate::vga_writer;
use alloc::{
    // boxed::Box,
    // format,
    string::{String, ToString},
    vec::Vec,
};

/* use lazy_static::lazy_static; */
use pc_keyboard::{KeyCode, KeyEvent, KeyState};

pub struct LevelChoice {
    pub name: String,
    pub content: String,
}

/*
if screens > 8
page*2 = start of screens
page*2 + 8 = end


  [-] [-] [-] [-]
<                 >
  [-] [-] [-] [-]
=======descr=======



*/

pub struct Level {
    pub name: String,
    pub description: (String, String, String),
    pub choices: Vec<LevelChoice>,
    pub next: Screen,
    pub back: Screen,
    _selected: usize,
    _page: usize,
}

impl Level {
    pub fn new(
        name: &str,
        description: (&str, &str, &str),
        choices: Vec<LevelChoice>,
        back: Screen,
        next: Screen,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: (
                description.0.to_string(),
                description.1.to_string(),
                description.2.to_string(),
            ),
            choices,
            next,
            back,
            _selected: 0,
            _page: 0,
        }
    }
}

impl Level {
    fn redraw_level(&mut self) {
        for x in 0..20 {
            vga_write!(0, x, "$3F{: ^80}", "")
        }
        match self.choices.len() {
            1 | 2 | 3 => {
                for _x in 0..self.choices.len() {
                    //self.choices[x].name
                }
            }
            _ => {}
        };
    }
}

impl Screenable for Level {
    fn init(&mut self) {
        vga_writer::WRITER.lock().clear();
        vga_write!(0, 21, "$8F{: ^80}", self.name);
        vga_write!(0, 22, "$3F{: ^80}", self.description.0);
        vga_write!(0, 23, "$3F{: ^80}", self.description.1);
        vga_write!(0, 24, "$3F{: ^80}", self.description.2);
        self.redraw_level();
    }

    fn on_time(&mut self, _time: u8) -> Option<Screen> {
        None
    }
    fn on_key(&mut self, key_event: KeyEvent, _as_char: Option<char>) -> Option<Screen> {
        // Detect key
        if key_event.state == KeyState::Down {}
        if key_event.code == KeyCode::A {}
        None
    }
}
