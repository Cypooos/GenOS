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
    pub if_sel: Screen,
}

impl LevelChoice {
    pub fn new(name: &str, content: &str, screen: Screen) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
            if_sel: screen,
        }
    }
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
    selected: usize,
    page: usize,
}

impl Level {
    pub fn new(name: &str, description: (&str, &str, &str), choices: Vec<LevelChoice>) -> Self {
        Self {
            name: name.to_string(),
            description: (
                description.0.to_string(),
                description.1.to_string(),
                description.2.to_string(),
            ),
            choices,
            selected: 0,
            page: 0,
        }
    }
}

impl Level {
    fn redraw_level(&mut self) {
        match self.choices.len() {
            1 | 2 | 3 | 4 => {
                let nb = self.choices.len();
                for x in 0..nb {
                    if x == self.selected {
                        vga_write!(
                            x * (80 / nb),
                            3,
                            " $E0{: ^1$}",
                            self.choices[x].name,
                            (80 / nb) - 2
                        );
                    } else {
                        vga_write!(
                            x * (80 / nb),
                            3,
                            " $8F{: ^1$}",
                            self.choices[x].name,
                            (80 / nb) - 2
                        );
                    }
                    for y in 4..17 {
                        vga_write!(x * (80 / nb), y, " $3F{: ^1$}", "", (80 / nb) - 2);
                    }
                }
            }
            nb => {
                for x in 0..4 {
                    if x + self.page == self.selected {
                        vga_write!(
                            x * 20,
                            3,
                            " $E0{: ^1$}",
                            self.choices[x + self.page].name,
                            12
                        );
                    } else {
                        vga_write!(
                            x * 20,
                            3,
                            "   $8F{: ^1$}",
                            self.choices[x + self.page].name,
                            14
                        );
                    }
                    for y in 4..17 {
                        vga_write!(x * 20, y, "   $3F{: ^1$}", "", 14);
                    }
                }
            }
        }
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
        if key_event.state != KeyState::Down {
            return None;
        };

        match key_event.code {
            KeyCode::D | KeyCode::ArrowRight => {
                self.selected = (self.selected + 1) % self.choices.len();
                if (self.selected >= self.page + 4) {
                    self.page = self.selected - 3
                } else if (self.selected < self.page) {
                    self.page = self.selected
                };
                self.redraw_level();
                None
            }
            KeyCode::Q | KeyCode::ArrowLeft => {
                self.selected = self
                    .selected
                    .checked_sub(1)
                    .unwrap_or(self.choices.len() - 1);
                if (self.selected >= self.page + 4) {
                    self.page = self.selected - 3
                } else if (self.selected < self.page) {
                    self.page = self.selected
                };
                self.redraw_level();
                None
            }
            KeyCode::Spacebar | KeyCode::Enter => Some(self.choices[self.selected].if_sel),
            _ => None,
        }
    }
}
