use super::desktop;

pub mod levels;
pub mod menus;
pub mod screens;

use alloc::boxed::Box;

pub trait Screenable: core::fmt::Debug + Send + Sync {
    fn init(&mut self) {}
    fn draw(&mut self) -> Option<screens::Screen> {
        None
    }
    fn on_key(&mut self, scancode: u8) -> Option<screens::Screen> {
        None
    }
}

//match self.active_screen.draw() {
//    Some(e) => self.active_screen = e,
//    None => {}
//}
/*    Screens::MainMenuDebugPass => {
        vga_write!(20, 5, "$3F{: ^40}", "Develloper acces");
        vga_write!(20, 6, "$3F{: ^40}", "");
        vga_write!(20, 7, "$3F{: ^40}", "Please enter the password :");
        vga_write!(20, 8, "$3F{: ^40}", "");
        let mut passStr = "";
        passStr = match self.active_screen.trys_left {
            0 => "[ ] [ ] [ ] [ ] [ ]",
            1 => "[*] [ ] [ ] [ ] [ ]",
            2 => "[*] [*] [ ] [ ] [ ]",
            3 => "[*] [*] [*] [ ] [ ]",
            4 => "[*] [*] [*] [*] [ ]",
        };

        vga_write!(20, 9, "$3F{: ^40}", passStr);
        vga_write!(20, 10, "$3F{: ^40}", "");
    }
    _ => {}
}

// if self.held.contains(&KeyCode::Tab) {
//     vga_write!(3, 3, "$3FTAB DETECTED");
// } else {
//     vga_write!(3, 3, "$3FNOT DETECTED");
// }*/

//if key_event.code == KeyCode::Escape {
//    if key_event.state == KeyState::Down {
//        self.active_screen = Screens::Menu;
//    } else {
//        self.active_screen = Screens::MainMenu;
//    };
//    return;
//}
// La meilleure solution mdr
// "held" n'est pas vraiment important dans tout les cas
// l'important c'est plutot juste certaine touche: Controle, Alt, Escape etc...
//print!("{:?}", key_event.code);
