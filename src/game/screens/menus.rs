use crate::vga_writer;

pub enum OneScreenMenu {
    MainMenu,
    CreditMenu,
}

/*
pub struct PasswordMenu<T: ScreenTrait> {
    pub code: &'static str,
    pub total_tries: usize,
    act_trie: usize,
    bad_tot: usize,
    act_tot: usize,
    pub if_ok: T,
    pub if_nok: T,
}

impl ScreenTrait for OneScreenMenu {
    fn draw(&mut self) -> Option<Screen> {
        match self {
            OneScreenMenu::MainMenu => {
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
            _ => {}
        }
    }
    fn key(&mut self, keycode: u8) -> Option<Screen> {
        None
    }
}

impl ScreenTrait for PasswordMenu {
    fn draw(&mut self) -> Option<Screen> {
        None
    }
    fn key(&mut self, keycode: u8) -> Option<Screen> {
        None
    }
}

pub static MAIN_MENU_PASSWORD_DEBUG: PasswordMenu<OneScreenMenu> = PasswordMenu {
    code: "tests",
    total_tries: 3,
    act_trie: 0,
    bad_tot: 0,
    act_tot: 0,
    if_ok: OneScreenMenu::TestMenu,
    if_nok: OneScreenMenu::MainMenu,
};
pub static MAIN_MENU: OneScreenMenu::MainMenu;
pub static TEST_MENU: OneScreenMenu::TestMenu;
*/
