use alloc::{boxed::Box, string::ToString, vec::Vec};

use super::Screenable;

use super::menus::{OneScreenMenu, PasswordMenu};

// ONERR: Bien vérifie que l'enum correspond au vec !!!!

#[derive(Copy, Clone, Debug)]
pub enum Screen {
    MainMenu = 0,
    CreditMenu,
    TestMenu,
    DebugPasswordMenu,
}

pub fn make_screens() -> Vec<Box<dyn Screenable>> {
    let mut returning: Vec<Box<dyn Screenable>> = Vec::new();
    returning.push(Box::new(OneScreenMenu::MainMenu));
    returning.push(Box::new(OneScreenMenu::CreditMenu));
    returning.push(Box::new(OneScreenMenu::TestMenu));
    returning.push(Box::new(PasswordMenu::new(
        "123456",
        Screen::TestMenu,
        Screen::MainMenu,
    )));
    returning
}
