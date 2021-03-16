use crate::TUI::screen::Screens;

use lazy_static::lazy_static;


use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use spin::Mutex;


lazy_static! {
    pub static ref DESKTOP: Mutex<DesktopTUI> = Mutex::new(DesktopTUI {
        mouse_pos: (5,5),
        active_screen:Screens::DefaultMenu(),
    });
}

pub struct DesktopTUI {
    mouse_pos:(u16,u16),
    active_screen:Screens,
}


impl DesktopTUI {
    pub fn on_key(&mut self,scancode:u8 ) {
        
        lazy_static! {
            static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> = Mutex::new(
                Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore)
            );
        }

        let mut keyboard = KEYBOARD.lock();
        //print!("desktop:{},",scancode);

        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {

            match key_event.code {
                KeyCode::WindowsLeft => {
                    println!("WINDOWS LEFT DETECTED");
                }
                _ => {
                    if let Some(key) = keyboard.process_keyevent(key_event) {
                        match key {
                            DecodedKey::Unicode(character) => print!("{}", character),
                            DecodedKey::RawKey(key) => print!("{:?}", key),
                        }
                    };
                }
            }
            

        }
        
    }
}